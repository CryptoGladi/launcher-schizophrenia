use std::fs::read_dir;

use self::downloader::Downloader;
use bytesize::ByteSize;
use downloader::Progress::*;
use sysinfo::{System, SystemExt};

pub mod command;
mod downloader;
mod flags;

#[derive(Debug)]
pub struct Game {
    min_use_memory: ByteSize,
    max_use_memory: ByteSize,
    username: String,
}

impl Default for Game {
    fn default() -> Self {
        let system_info = System::new_all();
        let max_use_memory = ByteSize::b(system_info.total_memory() / 2);

        Self {
            min_use_memory: ByteSize::gib(1),
            max_use_memory,
            username: "test_player".to_string(),
        }
    }
}

impl Game {
    pub fn run(&self) {
        let flags = flags::get_flags(&self);
        log::error!("flags: {}", flags);
    }

    pub fn game_is_installed(&self) -> anyhow::Result<bool> {
        let path = downloader::get_path();

        if !path.is_dir() {
            return Ok(false);
        }

        Ok(downloader::CHECKSUM_FOR_UNPACKED_ARCHIVE.check(&mut read_dir(path)?)?)
    }

    pub async fn download_game(&self) -> anyhow::Result<()> {
        let mut dowloader = Downloader::default();

        dowloader.set_callback(|progress| {
            match progress {
                Downloading(e) => {},
                Decompressing => log::warn!("decompess"),
            }

            // TODO https://github.com/tauri-apps/tauri-plugin-upload/blob/dev/src/lib.rs#L73
        });

        dowloader.download().await?;

        Ok(())
    }
}
