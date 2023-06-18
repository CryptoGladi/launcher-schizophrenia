//! Главный модуль, который отвечает за запуск, настройку игры

use self::downloader::{Downloader, Progress};
use bytesize::ByteSize;

pub mod command;
mod downloader;
mod flags;

/// Главная структура для управления процессом игры
#[derive(Debug)]
pub struct GameManager {
    min_use_memory: ByteSize,
    max_use_memory: ByteSize,
    username: String,
}

impl Default for GameManager {
    fn default() -> Self {
        Self {
            min_use_memory: ByteSize::gib(1),
            max_use_memory: ByteSize::gib(4),
            username: "test_player".to_string(),
        }
    }
}

impl GameManager {
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

    pub fn is_installed(&self) -> anyhow::Result<bool> {
        let path = crate::path::get_config().join("Шизофрения Ретёрн.jar");

        Ok(path.is_file())
    }

    pub async fn download<'a>(
        &self,
        callback: impl FnMut(Progress) + Send + Sync + 'a,
    ) -> anyhow::Result<()> {
        let mut dowloader = Downloader::default();

        dowloader.set_callback(callback);
        dowloader.download().await?;

        Ok(())
    }
}
