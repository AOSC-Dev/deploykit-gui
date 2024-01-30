// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use eyre::bail;
use eyre::Result;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use parser::list_zoneinfo;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use tokio::runtime::Runtime;
use utils::is_efi;
use utils::Recipe;
use zbus::dbus_proxy;
use zbus::Connection;
use zbus::Result as zResult;

use crate::utils::get_download_info;
use crate::utils::handle_serde_config;

mod parser;
mod utils;

#[dbus_proxy(
    interface = "io.aosc.Deploykit1",
    default_service = "io.aosc.Deploykit",
    default_path = "/io/aosc/Deploykit"
)]
trait Deploykit {
    async fn set_config(&self, field: &str, value: &str) -> zResult<String>;
    async fn get_config(&self, field: &str) -> zResult<String>;
    async fn get_progress(&self) -> zResult<String>;
    async fn reset_config(&self) -> zResult<String>;
    async fn get_list_devices(&self) -> zResult<String>;
    async fn auto_partition(&self, dev: &str) -> zResult<String>;
    async fn start_install(&self) -> zResult<String>;
    async fn get_auto_partition_progress(&self) -> zResult<String>;
    async fn get_list_partitions(&self, dev: &str) -> zResult<String>;
    async fn get_recommend_swap_size(&self) -> zResult<String>;
    async fn get_memory(&self) -> zResult<String>;
    async fn find_esp_partition(&self, dev: &str) -> zResult<String>;
}

#[derive(Debug, Deserialize)]
struct Dbus {
    result: DbusResult,
    data: Value,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum DbusResult {
    Ok,
    Error,
}

impl TryFrom<String> for Dbus {
    type Error = eyre::Error;

    fn try_from(value: String) -> std::prelude::v1::Result<Self, <Dbus as TryFrom<String>>::Error> {
        let res = serde_json::from_str::<Dbus>(&value)?;

        match res.result {
            DbusResult::Ok => Ok(res),
            DbusResult::Error => bail!("Failed to execute query: {:?}", res.data),
        }
    }
}

impl Dbus {
    async fn set_config(proxy: &DeploykitProxy<'_>, field: &str, value: &str) -> Result<Self> {
        let res = proxy.set_config(field, value).await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }

    async fn auto_partition(proxy: &DeploykitProxy<'_>, dev: &str) -> Result<Self> {
        let res = proxy.auto_partition(dev).await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }

    async fn get_progress(proxy: &DeploykitProxy<'_>) -> Result<Self> {
        let res = proxy.get_progress().await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }

    async fn start_install(proxy: &DeploykitProxy<'_>) -> Result<Self> {
        let res = proxy.start_install().await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }

    async fn get_auto_partition_progress(proxy: &DeploykitProxy<'_>) -> Result<Self> {
        let res = proxy.get_auto_partition_progress().await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }

    async fn find_esp_partition(proxy: &DeploykitProxy<'_>, dev: &str) -> Result<Self> {
        let res = proxy.find_esp_partition(dev).await?;
        let res = Self::try_from(res)?;

        Ok(res)
    }
}

static TOKIO: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

static PROXY: OnceCell<DeploykitProxy> = OnceCell::new();
static RECIPE: OnceCell<Recipe> = OnceCell::new();

#[tauri::command]
fn gparted() {
    Command::new("gparted").output().ok();
}

#[tauri::command]
fn list_devices() -> String {
    let proxy = PROXY.get().unwrap();
    let res = TOKIO.block_on(proxy.get_list_devices());

    match res {
        Ok(res) => res,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

#[derive(Debug, Serialize)]
pub struct ZoneInfoResult {
    pub result: String,
    pub data: Value,
}

#[tauri::command]
fn list_timezone() -> String {
    let timezone = list_zoneinfo();

    match timezone {
        Ok(t) => t,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

#[tauri::command]
fn set_config(config: &str) -> String {
    let proxy = PROXY.get().unwrap();
    let config = handle_serde_config(config);

    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return serde_json::json!({
                "result": "Error",
                "data": format!("{:?}", e),
            })
            .to_string()
        }
    };

    let (url, sha256sum) = match get_download_info(&config) {
        Ok((url, sha256sum)) => (url, sha256sum),
        Err(e) => {
            return serde_json::json!({
                "result": "Error",
                "data": format!("{:?}", e),
            })
            .to_string()
        }
    };

    let download_value = serde_json::json!({
        "Http": {
            "url": url,
            "hash": sha256sum,
        }
    });

    if let Err(e) = TOKIO.block_on(Dbus::set_config(
        proxy,
        "download",
        &download_value.to_string(),
    )) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Err(e) = TOKIO.block_on(Dbus::set_config(proxy, "locale", &config.locale.locale)) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Err(e) = TOKIO.block_on(Dbus::set_config(
        proxy,
        "user",
        &serde_json::json! {{
            "username": &config.user,
            "password": &config.pwd,
        }}
        .to_string(),
    )) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Err(e) = TOKIO.block_on(Dbus::set_config(proxy, "timezone", &config.timezone.data)) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Err(e) = TOKIO.block_on(Dbus::set_config(proxy, "hostname", &config.hostname)) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Err(e) = TOKIO.block_on(Dbus::set_config(
        proxy,
        "rtc_as_localtime",
        &(!config.rtc_utc).to_string(),
    )) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    let swap_config = match config.swapfile.size {
        0 => "\"Disable\"".to_string(),
        x => serde_json::json!({
            "Custom": x * 1024 * 1024 * 1024
        })
        .to_string(),
    };

    if let Err(e) = TOKIO.block_on(Dbus::set_config(proxy, "swapfile", &swap_config)) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    let part_config = match serde_json::to_string(&config.partition) {
        Ok(p) => p,
        Err(e) => {
            return serde_json::json!({
                "result": "Error",
                "data": format!("{:?}", e),
            })
            .to_string();
        }
    };

    if let Err(e) = TOKIO.block_on(Dbus::set_config(proxy, "target_partition", &part_config)) {
        return serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string();
    }

    if let Some(efi) = config.efi_partition {
        let part_config = match serde_json::to_string(&efi) {
            Ok(p) => p,
            Err(e) => {
                return serde_json::json!({
                    "result": "Error",
                    "data": format!("{:?}", e),
                })
                .to_string();
            }
        };
        if let Err(e) = TOKIO.block_on(proxy.set_config("efi_partition", &part_config)) {
            return serde_json::json!({
                "result": "Error",
                "data": format!("{:?}", e),
            })
            .to_string();
        }
    } else if is_efi() {
        let parent_path = config.partition.parent_path;

        if parent_path.is_none() {
            return serde_json::json!({
                "result": "Error",
                "data": format!("Parent path is not set"),
            })
            .to_string();
        }
        let efi_part = TOKIO.block_on(Dbus::find_esp_partition(proxy, &parent_path.unwrap()));

        match efi_part {
            Ok(efi) => {
                dbg!(1);
                let part_config = match serde_json::to_string(&efi.data) {
                    Ok(p) => p,
                    Err(e) => {
                        return serde_json::json!({
                            "result": "Error",
                            "data": format!("{:?}", e),
                        })
                        .to_string();
                    }
                };

                if let Err(e) =
                    TOKIO.block_on(Dbus::set_config(proxy, "efi_partition", &part_config))
                {
                    return serde_json::json!({
                        "result": "Error",
                        "data": format!("{:?}", e),
                    })
                    .to_string();
                }
            }
            Err(e) => {
                return serde_json::json!({
                    "result": "Error",
                    "data": format!("{:?}", e),
                })
                .to_string();
            }
        }
    }

    println!("{:?}", TOKIO.block_on(proxy.get_config("")));

    return serde_json::json!({
        "result": "Ok",
        "data": "",
    })
    .to_string();
}

#[tauri::command]
fn get_recipe() -> String {
    match RECIPE.get_or_try_init(|| TOKIO.block_on(utils::get_recpie())) {
        Ok(s) => serde_json::json!({
            "result": "Ok",
            "data": s,
        })
        .to_string(),
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

#[tauri::command]
fn list_partitions(dev: &str) -> String {
    let proxy = PROXY.get().unwrap();
    let res = TOKIO.block_on(proxy.get_list_partitions(dev));

    match res {
        Ok(res) => res,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

#[tauri::command]
fn get_recommend_swap_size() -> String {
    let proxy = PROXY.get().unwrap();
    let res = TOKIO.block_on(proxy.get_recommend_swap_size());

    match res {
        Ok(res) => res,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

#[tauri::command]
fn get_memory() -> String {
    let proxy = PROXY.get().unwrap();
    let res = TOKIO.block_on(proxy.get_memory());

    match res {
        Ok(res) => res,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

fn main() {
    // init tokio runtime
    let tokio = &*TOKIO;

    // init dbus proxy
    PROXY.get_or_init(move || {
        tokio.block_on(async {
            let conn = Connection::system()
                .await
                .expect("Failed to connect to system bus");
            DeploykitProxy::new(&conn)
                .await
                .expect("Failed to create dbus proxy for io.aosc.Deploykit1")
        })
    });

    let proxy = &*PROXY.get().unwrap();

    println!("{:?}", TOKIO.block_on(proxy.get_config("")));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            set_config,
            list_devices,
            list_partitions,
            gparted,
            list_timezone,
            get_recommend_swap_size,
            get_memory,
            get_recipe,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
