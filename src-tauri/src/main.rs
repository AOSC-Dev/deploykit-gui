// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::bail;
use eyre::Result;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use serde_json::Value;
use tokio::runtime::Runtime;
use zbus::dbus_proxy;
use zbus::Connection;
use zbus::Result as zResult;

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
}

#[derive(Debug, Deserialize)]
struct Dbus {
    result: DbusResult,
    data: Value,
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum DbusResult {
    Ok,
    Error,
}

static TOKIO: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

static PROXY: OnceCell<DeploykitProxy> = OnceCell::new();

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn set_config(field: &str, value: &str) -> String {
    let proxy = PROXY.get().unwrap();
    let res = TOKIO.block_on(proxy.set_config(field, value));

    match res {
        Ok(res) => res,
        Err(e) => serde_json::json!({
            "result": "Error",
            "data": format!("{:?}", e),
        })
        .to_string(),
    }
}

fn main() -> Result<()> {
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
        .invoke_handler(tauri::generate_handler![set_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
