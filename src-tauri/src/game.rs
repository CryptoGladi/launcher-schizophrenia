use std::fs::read_dir;

use bytesize::ByteSize;
use sysinfo::{System, SystemExt};
use chksum::prelude::*;
use anyhow::Result;

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

        let digest = read_dir(path)?.chksum(downloader::CHECKSUM.1)?;
        Ok(format!("{:x}", digest) == downloader::CHECKSUM.0)
    }
}
