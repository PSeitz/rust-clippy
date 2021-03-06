#![feature(tool_lints)]

use std::env;

#[macro_export]
macro_rules! get_version_info {
    () => {{
        let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u8>().unwrap();
        let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u8>().unwrap();
        let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u16>().unwrap();

        let host_compiler = $crate::get_channel();
        let commit_hash = option_env!("GIT_HASH").map(|s| s.to_string());
        let commit_date = option_env!("COMMIT_DATE").map(|s| s.to_string());

        VersionInfo {
            major,
            minor,
            patch,
            host_compiler,
            commit_hash,
            commit_date,
        }
    }};
}

// some code taken and adapted from RLS and cargo
pub struct VersionInfo {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
    pub host_compiler: Option<String>,
    pub commit_hash: Option<String>,
    pub commit_date: Option<String>,
}

impl std::fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.commit_hash {
            Some(_) => {
                write!(
                    f,
                    "clippy {}.{}.{} ({} {})",
                    self.major,
                    self.minor,
                    self.patch,
                    self.commit_hash.clone().unwrap_or_default().trim(),
                    self.commit_date.clone().unwrap_or_default().trim(),
                )?;
            },
            None => {
                write!(f, "clippy {}.{}.{}", self.major, self.minor, self.patch)?;
            },
        };
        Ok(())
    }
}

pub fn get_channel() -> Option<String> {
    if let Ok(channel) = env::var("CFG_RELEASE_CHANNEL") {
        Some(channel)
    } else {
        // we could ask ${RUSTC} -Vv and do some parsing and find out
        Some(String::from("nightly"))
    }
}

pub fn get_commit_hash() -> Option<String> {
    std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}

pub fn get_commit_date() -> Option<String> {
    std::process::Command::new("git")
        .args(&["log", "-1", "--date=short", "--pretty=format:%cd"])
        .output()
        .ok()
        .and_then(|r| String::from_utf8(r.stdout).ok())
}
