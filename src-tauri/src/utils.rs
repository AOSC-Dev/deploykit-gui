use eyre::{eyre, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    variants: Value,
    mirrors: Value,
}

pub async fn get_recpie() -> Result<Recipe> {
    let client = Client::builder().user_agent("deploykit").build()?;
    let recipe = client
        .get("https://releases.aosc.io/manifest/recipe.json")
        .send()
        .await?
        .error_for_status()?
        .json::<Recipe>()
        .await?;

    Ok(recipe)
}

#[derive(Deserialize)]
pub struct InstallConfig {
    pub locale: Locale,
    pub variant: Variant,
    mirror: Mirror,
    partition: Partition,
    pub user: String,
    pub pwd: String,
    pub hostname: String,
    pub rtc_utc: bool,
    pub timezone: Timezone,
}

#[derive(Deserialize)]
pub struct Locale {
    pub locale: String,
}

#[derive(Deserialize)]
pub struct Variant {
    pub squashfs: Vec<Squashfs>,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
struct Mirror {
    url: String,
}

#[derive(Deserialize)]
struct Partition {
    fs_type: String,
    parent_path: Option<String>,
    path: Option<String>,
    size: u64,
}

#[derive(Deserialize)]
pub struct Timezone {
    pub data: String,
}

pub fn get_download_info(config: &InstallConfig) -> Result<(String, String)> {
    let mut sqfs = config
        .variant
        .squashfs
        .iter()
        .filter(|x| get_arch_name().map(|arch| arch == x.arch).unwrap_or(false))
        .collect::<Vec<_>>();

    sqfs.sort_by(|a, b| b.date.cmp(&a.date));
    let candidate = sqfs
        .first()
        .ok_or_else(|| eyre!("Variant squashfs is empty"))?;

    let mirror = &config.mirror.url;
    let url = format!("{}{}", mirror, candidate.path);

    Ok((url, candidate.sha256sum.to_owned()))
}

pub fn handle_serde_config(s: &str) -> Result<InstallConfig> {
    Ok(serde_json::from_str(s)?)
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
