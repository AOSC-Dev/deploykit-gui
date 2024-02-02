use std::{
    io,
    path::Path,
    process::{Command, Stdio},
};

use eyre::{eyre, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub partition: Partition,
    pub efi_partition: Option<Partition>,
    pub user: String,
    pub pwd: String,
    pub hostname: String,
    pub rtc_utc: bool,
    pub timezone: Timezone,
    pub swapfile: SwapFile,
}

#[derive(Deserialize)]
pub struct SwapFile {
    pub size: usize,
}

#[derive(Deserialize)]
pub struct Locale {
    pub locale: String,
}

#[derive(Deserialize)]
pub struct Variant {
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

#[derive(Deserialize)]
struct Mirror {
    url: String,
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

pub fn get_download_info(config: &InstallConfig) -> Result<(String, String)> {
    let sqfs = config
        .variant
        .squashfs
        .clone()
        .into_iter()
        .filter(|x| get_arch_name().map(|arch| arch == x.arch).unwrap_or(false))
        .collect::<Vec<_>>();

    let (candidate, url) = candidate_sqfs(sqfs, &config.mirror.url)?;

    Ok((url, candidate.sha256sum.to_owned()))
}

pub fn candidate_sqfs(mut sqfs: Vec<Squashfs>, url: &str) -> Result<(Squashfs, String), eyre::Error> {
    sqfs.sort_by(|a, b| b.date.cmp(&a.date));

    let candidate = sqfs
        .first()
        .ok_or_else(|| eyre!("Variant squashfs is empty"))?
        .to_owned()
        .to_owned();

    // let mirror = &config.mirror.url;
    let url = format!("{}{}", url, candidate.path);

    Ok((candidate, url))
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

pub fn is_efi() -> bool {
    let efi_path = "/sys/firmware/efi";
    Path::new(efi_path).exists()
}

pub trait CommandExt {
    fn spawn_detached(&mut self) -> io::Result<()>;
}

impl CommandExt for Command {
    fn spawn_detached(&mut self) -> io::Result<()> {
        // This is pretty much lifted from the implementation in Alacritty:
        // https://github.com/alacritty/alacritty/blob/b9c886872d1202fc9302f68a0bedbb17daa35335/alacritty/src/daemon.rs

        self.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        #[cfg(unix)]
        unsafe {
            use std::os::unix::process::CommandExt as _;

            self.pre_exec(move || {
                match libc::fork() {
                    -1 => return Err(io::Error::last_os_error()),
                    0 => (),
                    _ => libc::_exit(0),
                }

                if libc::setsid() == -1 {
                    return Err(io::Error::last_os_error());
                }

                Ok(())
            });
        }

        self.spawn().map(|_| ())
    }
}
