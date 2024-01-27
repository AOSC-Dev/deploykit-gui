// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use parser::list_zoneinfo;
use serde::Serialize;
use serde_json::Value;
use tokio::runtime::Runtime;
use zbus::dbus_proxy;
use zbus::Connection;
use zbus::Result as zResult;

mod parser;

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
            list_timezone
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
