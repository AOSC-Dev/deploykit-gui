// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::http::HeaderValue;
use axum::http::Method;
use axum::Router;
use eyre::bail;
use eyre::eyre;
use eyre::Result;
use parser::list_zoneinfo;
use parser::ZoneInfo;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::io::ErrorKind;
use std::process;
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tauri::api::dialog;
use tauri::Manager;
use tauri::State;
use tauri::Window;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use utils::candidate_sqfs;
use utils::get_mirror_speed_score;
use utils::is_efi;
use utils::Mirror;
use utils::Recipe;
use utils::Squashfs;
use utils::Variant;
use zbus::proxy;
use zbus::Connection;
use zbus::Result as zResult;

use crate::utils::get_download_info;
use crate::utils::handle_serde_config;
use crate::utils::DownloadInfo;

mod parser;
mod utils;

static SKIP_DESKTOP_OR_INSTALL: AtomicBool = AtomicBool::new(false);
const BGM_LIST: &[u8] = include_bytes!("../bgm.json");
static IS_BASE_SQFS: AtomicBool = AtomicBool::new(false);

#[proxy(
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
    async fn ping(&self) -> zResult<String>;
    async fn get_all_esp_partitions(&self) -> zResult<String>;
    async fn reset_progress_status(&self) -> zResult<String>;
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
    ListPartitions(&'a str),
    ListDevice,
    GetRecommendSwapSize,
    GetMemory,
    CancelInstall,
    ResetConfig,
    DiskIsRightCombo(&'a str),
    Ping,
    GetAllEspPartitions,
    ResetProgressStatus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "status")]
enum AutoPartitionProgress {
    Pending,
    Working,
    Finish { res: Result<Value, Value> },
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
            DbusMethod::ListPartitions(dev) => proxy.get_list_partitions(dev).await?,
            DbusMethod::ListDevice => proxy.get_list_devices().await?,
            DbusMethod::GetRecommendSwapSize => proxy.get_recommend_swap_size().await?,
            DbusMethod::GetMemory => proxy.get_memory().await?,
            DbusMethod::CancelInstall => proxy.cancel_install().await?,
            DbusMethod::ResetConfig => proxy.reset_config().await?,
            DbusMethod::DiskIsRightCombo(dev) => proxy.disk_is_right_combo(dev).await?,
            DbusMethod::Ping => proxy.ping().await?,
            DbusMethod::GetAllEspPartitions => proxy.get_all_esp_partitions().await?,
            DbusMethod::ResetProgressStatus => proxy.reset_progress_status().await?,
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
    #[error("Failed to auto partition: {path}, err: {err}")]
    AutoPartitionFailed { path: String, err: Value },
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

    let DownloadInfo {
        url,
        checksum,
        name,
    } = get_download_info(&config)?;

    if name == "Base" || name == "Server" {
        IS_BASE_SQFS.store(true, Ordering::Relaxed);
    }

    let download_value = serde_json::json!({
        "Http": {
            "url": url,
            "hash": checksum,
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

    let swap_config = if config.swapfile.size == 0.0 {
        "\"Disable\"".to_string()
    } else {
        serde_json::json!({
            "Custom": (config.swapfile.size * 1024.0 * 1024.0 * 1024.0) as u64,
        })
        .to_string()
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
    let res = Dbus::run(&state.proxy, DbusMethod::ListPartitions(dev)).await?;

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
async fn disk_is_right_combo(state: State<'_, DkState<'_>>, disk: &str) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::DiskIsRightCombo(disk)).await?;

    Ok(())
}

#[tauri::command]
fn is_efi_api() -> TauriResult<bool> {
    Ok(is_efi())
}

#[tauri::command]
async fn find_all_esp_parts(state: State<'_, DkState<'_>>) -> TauriResult<Value> {
    let proxy = &state.proxy;
    let res = Dbus::run(proxy, DbusMethod::GetAllEspPartitions).await?;

    Ok(res.data)
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

#[tauri::command]
async fn mirrors_speedtest(mirrors: Vec<Mirror>) -> Vec<Mirror> {
    let mut speedtest_mirror = vec![];

    let client = reqwest::Client::builder()
        .user_agent("deploykit")
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let mut task = vec![];
    for mirror in &mirrors {
        task.push(get_mirror_speed_score(&mirror.url, &client))
    }
    let results = futures::future::join_all(task).await;
    for (index, result) in results.into_iter().enumerate() {
        if let Ok(score) = result {
            speedtest_mirror.push((mirrors[index].loc_tr.to_owned(), score));
        }
    }
    speedtest_mirror.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    let mut new_mirrors = vec![];
    for (name, _) in speedtest_mirror {
        let index = mirrors.iter().position(|x| x.loc_tr == name).unwrap();
        new_mirrors.push(mirrors[index].to_owned());
    }

    new_mirrors
}

#[tauri::command]
async fn get_bgm_list() -> TauriResult<Vec<Value>> {
    let mut bgm_list: Vec<Value> = serde_json::from_slice(BGM_LIST)?;
    let mut rng = thread_rng();

    bgm_list.shuffle(&mut rng);

    Ok(bgm_list)
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
    eta_lo: Option<u8>,
    eta_hi: Option<u8>,
}

#[derive(Debug, Serialize)]
struct GuiProgressStatus {
    c: u8,
    t: u8,
    p: f64,
}

#[tauri::command]
async fn start_install(state: State<'_, DkState<'_>>) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::StartInstall).await?;

    Ok(())
}

#[tauri::command]
async fn auto_partition(
    window: Window,
    state: State<'_, DkState<'_>>,
    dev: &str,
) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::AutoPartition(dev)).await?;

    loop {
        let progress = Dbus::run(proxy, DbusMethod::GetAutoPartitionProgress).await?;
        let data: AutoPartitionProgress = serde_json::from_value(progress.data)?;

        match data {
            AutoPartitionProgress::Finish { ref res } => match res {
                Err(v) => {
                    return Err(DeploykitGuiError::AutoPartitionFailed {
                        path: dev.to_string(),
                        err: v.to_owned(),
                    })
                }
                Ok(_) => {
                    window.emit("auto_partition_progress", &data).unwrap();
                    return Ok(());
                }
            },
            _ => {
                window.emit("auto_partition_progress", &data).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[tauri::command]
async fn reboot() -> TauriResult<()> {
    Command::new("systemctl").arg("reboot").output()?;

    Ok(())
}

#[tauri::command]
fn is_skip() -> bool {
    SKIP_DESKTOP_OR_INSTALL.load(Ordering::SeqCst)
}

#[tauri::command]
async fn i18n_recipe(state: State<'_, DkState<'_>>, locale: &str) -> TauriResult<Value> {
    let mut lock = state.recipe_i18n.lock().await;

    let map = match &*lock {
        Some(r) => r.to_owned(),
        None => {
            let recipe = utils::get_i18n_file().await?;
            *lock = Some(recipe.clone());

            recipe
        }
    };

    let value = match locale {
        "zh-CN" | "zh-TW" => map.get("zh-CN"),
        _ => map.get("en"),
    }
    .ok_or_else(|| eyre!("Failed to get language value in i18n file"))?
    .to_owned();

    Ok(value)
}

#[tauri::command]
async fn reset_progress_status(state: State<'_, DkState<'_>>) -> TauriResult<()> {
    let proxy = &state.proxy;
    Dbus::run(proxy, DbusMethod::ResetProgressStatus).await?;

    Ok(())
}

struct DkState<'a> {
    recipe: Mutex<Option<Recipe>>,
    recipe_i18n: Mutex<Option<HashMap<String, Value>>>,
    proxy: DeploykitProxy<'a>,
}

async fn init() -> Result<DeploykitProxy<'static>> {
    let conn = Connection::system().await?;
    let proxy = DeploykitProxy::new(&conn).await?;

    Dbus::run(&proxy, DbusMethod::Ping).await?;

    Ok(proxy)
}

#[tokio::main]
async fn main() {
    let proxy = init().await;

    match proxy {
        Ok(p) => {
            let pc = p.clone();
            tauri::Builder::default()
                .setup(move |app| {
                    let dir = match app.get_cli_matches() {
                        // `matches` here is a Struct with { args, subcommand }.
                        // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
                        // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
                        Ok(matches) => {
                            if matches
                                .args
                                .get("skip")
                                .map(|x| x.occurrences != 0)
                                .unwrap_or(false)
                            {
                                SKIP_DESKTOP_OR_INSTALL.store(true, Ordering::SeqCst);
                            }

                            let resources_dir = matches.args.get("resource-dir").unwrap();
                            let resource_dir = &resources_dir.value.as_str().unwrap();

                            resource_dir.to_string()
                        }
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    };

                    // FIXME: Workaround https://github.com/tauri-apps/tauri/issues/3725
                    tokio::spawn(async move {
                        let serve_dir = ServeDir::new(dir);

                        let axum_app = Router::new().nest_service("/", serve_dir).layer(
                            CorsLayer::new()
                                .allow_origin("*".parse::<HeaderValue>().unwrap())
                                .allow_methods([Method::GET]),
                        );

                        let listener = tokio::net::TcpListener::bind("127.0.0.1:23333")
                            .await
                            .unwrap();

                        axum::serve(listener, axum_app).await.unwrap();
                    });

                    let pc = pc.clone();
                    let pcc = pc.clone();
                    let window = app.get_window("main").unwrap();

                    tauri::async_runtime::spawn(async move {
                        // 重新设置后端状态，若后端状态是已完成安装或安装遇到了错误
                        let progress = Dbus::run(&pcc, DbusMethod::GetProgress).await;
                        if let Ok(progress) = progress {
                            let data: Result<ProgressStatus, _> =
                                serde_json::from_value(progress.data);
                            if let Ok(data) = data {
                                match data {
                                    ProgressStatus::Error(_) | ProgressStatus::Finish => {
                                        Dbus::run(&pcc, DbusMethod::ResetProgressStatus).await.ok();
                                    }
                                    _ => {}
                                }
                            }
                        }
                    });
                    tauri::async_runtime::spawn(async move { progress_event(window, pc).await });
                    Ok(())
                })
                .manage(DkState {
                    recipe: Mutex::new(None),
                    recipe_i18n: Mutex::new(None),
                    proxy: p,
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
                    cancel_install_and_exit,
                    get_squashfs_info,
                    disk_is_right_combo,
                    is_efi_api,
                    auto_partition,
                    mirrors_speedtest,
                    find_all_esp_parts,
                    reboot,
                    is_skip,
                    reset_progress_status,
                    get_bgm_list,
                    i18n_recipe
                ])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        Err(e) => dialog::blocking::message(
            None::<Window>.as_ref(),
            "Error",
            format!("Failed to connect D-Bus: {e}"),
        ),
    }
}

async fn progress_event(window: Window, p: DeploykitProxy<'_>) -> TauriResult<()> {
    let mut all: i8 = -1;
    let mut now_step = 0;
    loop {
        let progress = Dbus::run(&p, DbusMethod::GetProgress).await?;
        let data: ProgressStatus = serde_json::from_value(progress.data)?;
        match data {
            ProgressStatus::Working { step, progress, .. } => {
                if now_step == 0 {
                    all = if IS_BASE_SQFS.load(Ordering::Relaxed) {
                        24
                    } else {
                        31
                    };
                }

                if step != now_step {
                    now_step = step;
                    if step != 1 {
                        let (lo, hi) = calc_eta(step - 1, IS_BASE_SQFS.load(Ordering::Relaxed));
                        if lo.is_none() {
                            all -= hi.unwrap_or(0) as i8;
                        } else {
                            all -= lo.unwrap() as i8;
                        }
                    }
                }

                let data = GuiProgress {
                    status: GuiProgressStatus {
                        c: step,
                        t: 8,
                        p: progress,
                    },
                    eta_hi: Some(all as u8),
                    eta_lo: None,
                };
                window.emit("progress", &data).unwrap();
                // println!("emit:{:?}", data);
            }
            ProgressStatus::Error(_) => {
                window.emit("progress", &data).unwrap();
                println!("emit {:?}", data);
                Dbus::run(&p, DbusMethod::ResetProgressStatus).await?;
            }
            ProgressStatus::Finish => {
                window.emit("progress", &data).unwrap();
                Dbus::run(&p, DbusMethod::ResetProgressStatus).await?;
            }
            ProgressStatus::Pending => {
                // println!("emit {:?}", data);
                window.emit("progress", &data).unwrap()
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn calc_eta(step: u8, is_base: bool) -> (Option<u8>, Option<u8>) {
    match step {
        1 => (None, Some(1)),
        2 => {
            if is_base {
                (None, Some(8))
            } else {
                (None, Some(15))
            }
        }
        3 => (None, Some(5)),
        4 => (None, Some(1)),
        5 => (Some(2), Some(3)),
        6 => (None, Some(1)),
        7 => (Some(2), Some(3)),
        8 => (None, Some(1)),
        _ => (None, None),
    }
}
