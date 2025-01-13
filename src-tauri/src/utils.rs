use std::collections::HashMap;
use std::path::Path;

use eyre::{OptionExt, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Instant;
use url::Url;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::{
    AtomEnum, ClientMessageEvent, ConnectionExt as ConnectionExtB, EventMask,
};
use x11rb::wrapper::ConnectionExt;

const LOCAL_RECIPE: &str = "/run/livekit/livemnt/manifest/recipe.json";
const LOCAL_RECIPE_I18N: &str = "/run/livekit/livemnt/manifest/recipe-i18n.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    variants: Value,
    mirrors: Value,
}

pub fn is_local_recipe() -> bool {
    Path::new(LOCAL_RECIPE).is_file()
}

pub async fn get_recipe_local() -> Result<Recipe> {
    let content = tokio::fs::read_to_string(LOCAL_RECIPE).await?;
    let recipe: Recipe = serde_json::from_str(&content)?;

    Ok(recipe)
}

pub async fn get_recipe_i18n_local() -> Result<HashMap<String, Value>> {
    let content = tokio::fs::read_to_string(LOCAL_RECIPE_I18N).await?;
    let res: HashMap<String, Value> = serde_json::from_str(&content)?;

    Ok(res)
}

pub async fn get_recipe(client: &Client) -> Result<Recipe> {
    if is_local_recipe() {
        return get_recipe_local().await;
    }

    get_recpie_online(client).await
}

pub async fn get_i18n_file(client: &Client) -> Result<HashMap<String, Value>> {
    if is_local_recipe() {
        return get_recipe_i18n_local().await;
    }

    get_i18n_file_online(client).await
}

pub async fn get_recpie_online(client: &Client) -> Result<Recipe> {
    get(client, "https://releases.aosc.io/manifest/recipe.json").await
}

pub async fn get_i18n_file_online(client: &Client) -> Result<HashMap<String, Value>> {
    let local_recipe = Path::new("/manifest/recipe-i18n.json");
    if local_recipe.is_file() {
        let content = std::fs::read_to_string(local_recipe)?;
        let res: HashMap<String, Value> = serde_json::from_str(&content)?;
        return Ok(res);
    }

    get(client, "https://releases.aosc.io/manifest/recipe-i18n.json").await
}

async fn get<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T> {
    let res: T = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(res)
}

#[derive(Deserialize)]
pub struct InstallConfig {
    pub locale: Locale,
    pub variant: Variant,
    mirror: Option<Mirror>,
    pub partition: Partition,
    pub efi_partition: Option<Partition>,
    pub user: String,
    pub pwd: String,
    pub fullname: Option<String>,
    pub hostname: String,
    pub rtc_as_localtime: bool,
    pub timezone: Timezone,
    pub swapfile: SwapFile,
    pub is_offline_install: bool,
}

#[derive(Deserialize)]
pub struct SwapFile {
    pub size: f64,
}

#[derive(Deserialize)]
pub struct Locale {
    pub locale: String,
}

#[derive(Deserialize)]
pub struct Variant {
    pub name: String,
    #[serde(rename = "dir-name")]
    pub dir_name: Option<String>,
    pub squashfs: Vec<Squashfs>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Squashfs {
    arch: String,
    date: String,
    #[serde(rename = "downloadSize")]
    download_size: u64,
    inodes: u64,
    #[serde(rename = "instSize")]
    inst_size: u64,
    path: String,
    pub sha256sum: String,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Mirror {
    pub name: String,
    #[serde(rename = "name-tr")]
    pub name_tr: String,
    pub loc: String,
    #[serde(rename = "loc-tr")]
    pub loc_tr: String,
    pub url: String,
    pub score: Option<f32>,
    pub downloaded_size: Option<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct Partition {
    fs_type: String,
    pub parent_path: Option<String>,
    path: Option<String>,
    size: u64,
}

#[derive(Deserialize)]
pub struct Timezone {
    pub data: String,
}

pub struct DownloadInfo<'a> {
    pub url: Option<String>,
    pub checksum: &'a str,
    pub name: &'a str,
}

pub fn get_download_info(config: &InstallConfig) -> Result<DownloadInfo<'_>> {
    let sqfs = config
        .variant
        .squashfs
        .iter()
        .filter(|x| get_arch_name().map(|arch| arch == x.arch).unwrap_or(false))
        .collect::<Vec<_>>();

    let (candidate, url) = candidate_sqfs(sqfs, config.mirror.as_ref().map(|x| x.url.as_str()))?;

    Ok(DownloadInfo {
        url,
        checksum: candidate.sha256sum.as_str(),
        name: config.variant.name.as_str(),
    })
}

pub fn candidate_sqfs<'a>(
    mut sqfs: Vec<&'a Squashfs>,
    url: Option<&str>,
) -> Result<(&'a Squashfs, Option<String>)> {
    sqfs.sort_by(|a, b| b.date.cmp(&a.date));
    let candidate = sqfs.first().ok_or_eyre("Variant squashfs is empty")?;

    let mut res_url = None;
    if let Some(url) = url {
        res_url = Some(format!("{}{}", url, candidate.path));
    }

    Ok((candidate, res_url))
}

pub fn handle_serde_config(s: &str) -> Result<InstallConfig> {
    Ok(serde_json::from_str(s)?)
}

pub async fn get_mirror_speed_score(mirror_url: &str, client: &Client) -> Result<(f32, usize)> {
    let download_url = Url::parse(mirror_url)?.join("../.repotest")?;
    let timer = Instant::now();
    let mut resp = client.get(download_url).send().await?;
    resp.error_for_status_ref()?;

    let mut download_len = 0;

    while let Ok(Some(chunk)) = resp.chunk().await {
        download_len += chunk.len();
    }

    let result_time = timer.elapsed().as_secs_f32();

    Ok((result_time, download_len))
}

// AOSC OS specific architecture mapping for ppc64
#[cfg(target_arch = "powerpc64")]
#[inline]
pub(crate) fn get_arch_name() -> Option<&'static str> {
    let mut endian: libc::c_int = -1;
    let result;
    unsafe {
        result = libc::prctl(libc::PR_GET_ENDIAN, &mut endian as *mut libc::c_int);
    }
    if result < 0 {
        return None;
    }
    match endian {
        libc::PR_ENDIAN_LITTLE | libc::PR_ENDIAN_PPC_LITTLE => Some("ppc64el"),
        libc::PR_ENDIAN_BIG => Some("ppc64"),
        _ => None,
    }
}

/// AOSC OS specific architecture mapping table
#[cfg(not(target_arch = "powerpc64"))]
#[inline]
pub(crate) fn get_arch_name() -> Option<&'static str> {
    use std::env::consts::ARCH;
    match ARCH {
        "x86_64" => Some("amd64"),
        "x86" => Some("i486"),
        "powerpc" => Some("powerpc"),
        "aarch64" => Some("arm64"),
        "mips64" => Some("loongson3"),
        "riscv64" => Some("riscv64"),
        "loongarch64" => Some("loongarch64"),
        _ => None,
    }
}

pub fn control_window_above(pin_pids: &[u32], enable: bool) -> Result<()> {
    let mut fined = false;

    while !fined {
        let (conn, screen_num) = x11rb::connect(None).unwrap();
        let screen = &conn.setup().roots[screen_num];
        let root_id = screen.root;
        let cookie = conn.intern_atom(true, b"_NET_CLIENT_LIST")?;
        let atom = cookie.reply()?.atom;

        let reply = conn
            .get_property(false, root_id, atom, AtomEnum::ANY, 0, u32::MAX)?
            .reply()?;

        let windows = reply
            .value32()
            .ok_or_eyre("illage reply")?
            .collect::<Vec<_>>();

        let cookie = conn.intern_atom(true, b"_NET_WM_PID")?;
        let atom = cookie.reply()?.atom;

        let pin_window_cookie = conn.intern_atom(true, b"_NET_WM_STATE_ABOVE")?;
        let pin_window_atom = pin_window_cookie.reply()?.atom;

        let window_state_cookie = conn.intern_atom(true, b"_NET_WM_STATE")?;
        let window_state_atom = window_state_cookie.reply()?.atom;

        for window in windows {
            let pid = conn
                .get_property(false, window, atom, AtomEnum::ANY, 0, u32::MAX)?
                .reply();

            if let Ok(pid) = pid {
                let pids = pid
                    .value32()
                    .ok_or_eyre("illage reply")?
                    .collect::<Vec<_>>();

                if pids.iter().any(|x| pin_pids.contains(x)) {
                    fined = true;

                    let event = ClientMessageEvent::new(
                        32,
                        window,
                        window_state_atom,
                        [if enable { 1 } else { 0 }, pin_window_atom, 0, 0, 0],
                    );

                    // https://github.com/psychon/x11rb/discussions/929
                    // 从 WM 中置顶
                    conn.send_event(
                        false,
                        root_id,
                        EventMask::SUBSTRUCTURE_REDIRECT | EventMask::SUBSTRUCTURE_NOTIFY,
                        event,
                    )?;

                    conn.sync()?;
                }
            }
        }
    }

    Ok(())
}
