// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::bail;
use eyre::Result;
use parser::list_zoneinfo;
use parser::ZoneInfo;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::io;
use std::io::ErrorKind;
use std::process;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::State;
use tauri::Window;
use tokio::sync::Mutex;
use tokio::time::sleep;
use utils::candidate_sqfs;
use utils::is_efi;
use utils::Recipe;
use utils::Squashfs;
use utils::Variant;
use zbus::dbus_proxy;
use zbus::Connection;
use zbus::Result as zResult;

use crate::utils::get_download_info;
use crate::utils::handle_serde_config;
use crate::utils::CommandExt;

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
    async fn cancel_install(&self) -> zResult<String>;
    async fn disk_is_right_combo(&self, dev: &str) -> zResult<String>;
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
    CancelInstall,
    ResetConfig,
    DiskIsRightCombo(&'a str),
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
            DbusMethod::CancelInstall => proxy.cancel_install().await?,
            DbusMethod::ResetConfig => proxy.reset_config().await?,
            DbusMethod::DiskIsRightCombo(dev) => proxy.disk_is_right_combo(dev).await?,
        };

        let res = Self::try_from(s)?;
        Ok(res)
    }
}

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
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),
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
async fn list_devices(state: State<'_, DkState<'_>>) -> TauriResult<Value> {
    let proxy = &state.proxy;
    let res = Dbus::run(proxy, DbusMethod::ListDevice).await?;

    Ok(res.data)
}

#[tauri::command]
fn list_timezone() -> TauriResult<Vec<ZoneInfo>> {
    Ok(list_zoneinfo()?)
}

#[tauri::command]
async fn set_config(state: State<'_, DkState<'_>>, config: &str) -> TauriResult<()> {
    let proxy = &state.proxy;
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
        &proxy,
        DbusMethod::SetConfig("locale", &config.locale.locale),
    )
    .await?;

    Dbus::run(
        &proxy,
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
        &proxy,
        DbusMethod::SetConfig("timezone", &config.timezone.data),
    )
    .await?;

    Dbus::run(&proxy, DbusMethod::SetConfig("hostname", &config.hostname)).await?;
    Dbus::run(
        &proxy,
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

    Dbus::run(&proxy, DbusMethod::SetConfig("swapfile", &swap_config)).await?;

    let part_config = serde_json::to_string(&config.partition)?;

    Dbus::run(
        &proxy,
        DbusMethod::SetConfig("target_partition", &part_config),
    )
    .await?;

    if let Some(efi) = config.efi_partition {
        let part_config = serde_json::to_string(&efi)?;
        Dbus::run(&proxy, DbusMethod::SetConfig("efi_partition", &part_config)).await?;
    } else if is_efi() {
        let parent_path = config.partition.parent_path;

        if parent_path.is_none() {
            return Err(DeploykitGuiError::Io(io::Error::new(
                ErrorKind::NotFound,
                "Failed to find EFI partition",
            )));
        }
        let efi_part =
            Dbus::run(&proxy, DbusMethod::FindEspPartition(&parent_path.unwrap())).await?;

        let part_config = serde_json::to_string(&efi_part.data)?;
        Dbus::run(&proxy, DbusMethod::SetConfig("efi_partition", &part_config)).await?;
    }

    println!("{:?}", proxy.get_config("").await?);

    Ok(())
}

#[tauri::command]
async fn get_recipe(state: State<'_, DkState<'_>>) -> TauriResult<Recipe> {
    let mut lock = state.recipe.lock().await;

    match &*lock {
        Some(r) => Ok(r.to_owned()),
        None => {
            let recipe = utils::get_recpie().await?;
            *lock = Some(recipe.clone());
            Ok(recipe)
        }
    }
}

#[tauri::command]
fn get_squashfs_info(v: Variant, url: &str) -> TauriResult<Squashfs> {
    let c = candidate_sqfs(v.squashfs, url)?;

    Ok(c.0)
}

#[tauri::command]
async fn list_partitions(state: State<'_, DkState<'_>>, dev: &str) -> TauriResult<Value> {
    let res = Dbus::run(&state.proxy, DbusMethod::ListPpartitions(dev)).await?;

    Ok(res.data)
}

#[tauri::command]
async fn get_recommend_swap_size(state: State<'_, DkState<'_>>) -> TauriResult<Value> {
    let proxy = &state.proxy;
    let res = Dbus::run(proxy, DbusMethod::GetRecommendSwapSize).await?;

    Ok(res.data)
}

#[tauri::command]
async fn get_memory(state: State<'_, DkState<'_>>) -> TauriResult<Value> {
    let proxy = &state.proxy;
    let res = Dbus::run(proxy, DbusMethod::GetMemory).await?;

    Ok(res.data)
}

#[tauri::command]
async fn firefox() -> TauriResult<()> {
    tokio::task::spawn_blocking(|| Command::new("firefox").spawn_detached()).await??;

    Ok(())
}

#[tauri::command]
async fn disk_is_right_combo(state: State<'_, DkState<'_>>, disk: &str) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::DiskIsRightCombo(disk)).await?;

    Ok(())
}

#[tauri::command(async)]
async fn cancel_install_and_exit(
    state: State<'_, DkState<'_>>,
    reset_config: bool,
) -> TauriResult<()> {
    let progress = Dbus::run(&state.proxy, DbusMethod::GetProgress).await?;
    let progress = serde_json::from_value::<ProgressStatus>(progress.data)?;

    sleep(Duration::from_millis(50)).await;

    if let ProgressStatus::Working { .. } = progress {
        Dbus::run(&state.proxy, DbusMethod::CancelInstall).await?;
    }

    if reset_config {
        Dbus::run(&state.proxy, DbusMethod::ResetConfig).await?;
    }

    process::exit(0);
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status")]
enum ProgressStatus {
    Pending,
    Working { step: u8, progress: f64, v: usize },
    Error(Value),
    Finish,
}

#[derive(Debug, Serialize)]
struct GuiProgress {
    status: GuiProgressStatus,
}

#[derive(Debug, Serialize)]
struct GuiProgressStatus {
    c: u8,
    t: u8,
    p: f64,
}

#[tauri::command]
async fn start_install(window: Window, state: State<'_, DkState<'_>>) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::StartInstall).await?;

    loop {
        let progress = Dbus::run(proxy, DbusMethod::GetProgress).await?;
        let data: ProgressStatus = serde_json::from_value(progress.data)?;
        match data {
            ProgressStatus::Working { step, progress, .. } => {
                let data = GuiProgress {
                    status: GuiProgressStatus {
                        c: step,
                        t: 8,
                        p: progress,
                    },
                };
                window.emit("progress", &data).unwrap();
                println!("emit:{:?}", data);
            }
            _ => window.emit("progress", &data).unwrap(),
        }

        thread::sleep(Duration::from_millis(100));
    }
}

struct DkState<'a> {
    recipe: Mutex<Option<Recipe>>,
    proxy: DeploykitProxy<'a>,
}

fn main() {
    let tokio = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    let proxy = tokio.block_on(async {
        let conn = Connection::system()
            .await
            .expect("Failed to connect to system bus");
        DeploykitProxy::new(&conn)
            .await
            .expect("Failed to create Deploykit dbus proxy")
    });

    tauri::Builder::default()
        .manage(DkState {
            recipe: Mutex::new(None),
            proxy,
        })
        .invoke_handler(tauri::generate_handler![
            set_config,
            list_devices,
            list_partitions,
            gparted,
            list_timezone,
            get_recommend_swap_size,
            get_memory,
            get_recipe,
            start_install,
            firefox,
            cancel_install_and_exit,
            get_squashfs_info,
            disk_is_right_combo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
