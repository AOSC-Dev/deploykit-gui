// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::bail;
use eyre::Result;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use parser::list_zoneinfo;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::io;
use std::io::ErrorKind;
use std::process::Command;
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

#[derive(Debug)]
enum DbusMethod<'a> {
    SetConfig(&'a str, &'a str),
    AutoPartition(&'a str),
    GetProgress,
    StartInstall,
    GetAutoPartitionProgress,
    FindEspPartition(&'a str),
    ListPpartitions(&'a str),
    ListDevice,
    GetRecommendSwapSize,
    GetMemory,
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
    async fn run(proxy: &DeploykitProxy<'_>, method: DbusMethod<'_>) -> Result<Self> {
        let s = match method {
            DbusMethod::SetConfig(field, value) => proxy.set_config(field, value).await?,
            DbusMethod::AutoPartition(p) => proxy.auto_partition(p).await?,
            DbusMethod::GetProgress => proxy.get_progress().await?,
            DbusMethod::StartInstall => proxy.start_install().await?,
            DbusMethod::GetAutoPartitionProgress => proxy.get_auto_partition_progress().await?,
            DbusMethod::FindEspPartition(dev) => proxy.find_esp_partition(dev).await?,
            DbusMethod::ListPpartitions(dev) => proxy.get_list_partitions(dev).await?,
            DbusMethod::ListDevice => proxy.get_list_devices().await?,
            DbusMethod::GetRecommendSwapSize => proxy.get_recommend_swap_size().await?,
            DbusMethod::GetMemory => proxy.get_memory().await?,
        };

        let res = Self::try_from(s)?;
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

type TauriResult<T> = std::result::Result<T, DeploykitGuiError>;

#[derive(Debug, thiserror::Error)]
enum DeploykitGuiError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Zbus(#[from] zbus::Error),
    #[error(transparent)]
    Eyre(#[from] eyre::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl Serialize for DeploykitGuiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn gparted() -> TauriResult<()> {
    Command::new("gparted").output()?;

    Ok(())
}

#[tauri::command]
async fn list_devices() -> TauriResult<String> {
    let proxy = PROXY.get().unwrap();
    let res = Dbus::run(proxy, DbusMethod::ListDevice).await?;

    Ok(serde_json::to_string(&res.data)?)
}

#[derive(Debug, Serialize)]
pub struct ZoneInfoResult {
    pub result: String,
    pub data: Value,
}

#[tauri::command]
fn list_timezone() -> TauriResult<String> {
    Ok(list_zoneinfo()?)
}

#[tauri::command]
async fn set_config(config: &str) -> TauriResult<()> {
    let proxy = PROXY.get().unwrap();
    let config = handle_serde_config(config)?;

    let (url, sha256sum) = get_download_info(&config)?;

    let download_value = serde_json::json!({
        "Http": {
            "url": url,
            "hash": sha256sum,
        }
    });

    Dbus::run(
        proxy,
        DbusMethod::SetConfig("download", &download_value.to_string()),
    )
    .await?;
    Dbus::run(
        proxy,
        DbusMethod::SetConfig("locale", &config.locale.locale),
    )
    .await?;

    Dbus::run(
        proxy,
        DbusMethod::SetConfig(
            "user",
            &serde_json::json! {{
                "username": &config.user,
                "password": &config.pwd,
            }}
            .to_string(),
        ),
    )
    .await?;

    Dbus::run(
        proxy,
        DbusMethod::SetConfig("timezone", &config.timezone.data),
    )
    .await?;

    Dbus::run(proxy, DbusMethod::SetConfig("hostname", &config.hostname)).await?;
    Dbus::run(
        proxy,
        DbusMethod::SetConfig("rtc_as_localtime", &(!config.rtc_utc).to_string()),
    )
    .await?;

    let swap_config = match config.swapfile.size {
        0 => "\"Disable\"".to_string(),
        x => serde_json::json!({
            "Custom": x * 1024 * 1024 * 1024
        })
        .to_string(),
    };

    Dbus::run(proxy, DbusMethod::SetConfig("swapfile", &swap_config)).await?;

    let part_config = serde_json::to_string(&config.partition)?;

    Dbus::run(
        proxy,
        DbusMethod::SetConfig("target_partition", &part_config),
    )
    .await?;

    if let Some(efi) = config.efi_partition {
        let part_config = serde_json::to_string(&efi)?;
        Dbus::run(proxy, DbusMethod::SetConfig("efi_partition", &part_config)).await?;
    } else if is_efi() {
        let parent_path = config.partition.parent_path;

        if parent_path.is_none() {
            return Err(DeploykitGuiError::Io(io::Error::new(
                ErrorKind::NotFound,
                "Failed to find EFI partition",
            )));
        }
        let efi_part =
            Dbus::run(proxy, DbusMethod::FindEspPartition(&parent_path.unwrap())).await?;

        let part_config = serde_json::to_string(&efi_part.data)?;
        Dbus::run(proxy, DbusMethod::SetConfig("efi_partition", &part_config)).await?;
    }

    println!("{:?}", proxy.get_config("").await?);

    Ok(())
}

#[tauri::command]
fn get_recipe() -> TauriResult<String> {
    let recipe = RECIPE.get_or_try_init(|| TOKIO.block_on(utils::get_recpie()))?;

    Ok(serde_json::to_string(recipe)?)
}

#[tauri::command]
async fn list_partitions(dev: &str) -> TauriResult<String> {
    let proxy = PROXY.get().unwrap();
    let res = Dbus::run(proxy, DbusMethod::ListPpartitions(dev)).await?;

    Ok(serde_json::to_string(&res.data)?)
}

#[tauri::command]
async fn get_recommend_swap_size() -> TauriResult<String> {
    let proxy = PROXY.get().unwrap();
    let res = Dbus::run(proxy, DbusMethod::GetRecommendSwapSize).await?;

    Ok(serde_json::to_string(&res.data)?)
}

#[tauri::command]
async fn get_memory() -> TauriResult<String> {
    let proxy = PROXY.get().unwrap();
    let res = Dbus::run(proxy, DbusMethod::GetMemory).await?;

    Ok(serde_json::to_string(&res.data)?)
}

#[derive(Debug, Deserialize)]
enum ProgressStatus {
    Pending,
    Working(u8, f64, usize),
    Error(Value),
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
