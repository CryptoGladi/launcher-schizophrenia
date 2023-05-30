//! Главный модуль, который отвечает за запуск, настройку игры

use self::downloader::{Downloader, Progress};
use bytesize::ByteSize;
use downloader::Progress::*;
use log::debug;
use sysinfo::{System, SystemExt};

pub mod command;
mod downloader;
mod flags;

/// Главная структура для управления процессом игры
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
    /// Запустить игру
    ///
    /// # Внимание
    ///
    /// Для успешного запуска нужно сперва установать игру
    pub fn run(&self) -> anyhow::Result<()> {
        let flags = flags::get_flags(self);
        log::error!("flags: {}", flags);

        let args = vec![];
        let options = run_script::ScriptOptions::new();
        run_script::spawn(&flags, &args, &options)?;

        Ok(())
    }

    pub fn game_is_installed(&self) -> anyhow::Result<bool> {
        let path = downloader::get_path().join("Шизофрения Ретёрн.jar");

        Ok(path.is_file())
    }

    pub async fn download_game<'a>(
        &self,
        callback: impl FnMut(Progress) + Send + Sync + 'a,
    ) -> anyhow::Result<()> {
        let mut dowloader = Downloader::default();

        dowloader.set_callback(callback);
        dowloader.download().await?;

        Ok(())
    }
}
