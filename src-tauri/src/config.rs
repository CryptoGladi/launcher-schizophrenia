use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use std::{
    fs::{remove_file, OpenOptions},
    io::Write,
    path::PathBuf,
};

pub mod command;

const FILENAME: &str = "config_launcher.json";

#[derive(Serialize, Deserialize)]
#[serde(remote = "ByteSize")]
pub struct ByteSizeDef(pub u64);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub nickname: String,

    #[serde(with = "ByteSizeDef")]
    pub max_use_memory: ByteSize,

    #[serde(with = "ByteSizeDef")]
    pub min_use_memory: ByteSize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            nickname: "".to_string(),
            max_use_memory: ByteSize::gb(4),
            min_use_memory: ByteSize::gb(3),
        }
    }
}

impl Config {
    pub fn save(&self) -> anyhow::Result<()> {
        if Config::get_path().is_file() {
            remove_file(Config::get_path())?;
        }

        let json = serde_json::to_string_pretty(&self)?;

        std::fs::create_dir_all(crate::path::get_app_folder())?;
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Config::get_path())?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn get_path() -> PathBuf {
        crate::path::get_app_folder().join(FILENAME)
    }

    pub fn load() -> anyhow::Result<Config> {
        if !Config::get_path().is_file() {
            return Ok(Config::default());
        }

        let raw_json = std::fs::read_to_string(Config::get_path())?;
        Ok(serde_json::from_str(&raw_json)?)
    }
}
