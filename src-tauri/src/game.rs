//! Главный модуль, который отвечает за запуск и настройку игры

use self::downloader::{Downloader, Progress};
use bytesize::ByteSize;
use std::path::PathBuf;
use std::process::Command;

pub mod command;
mod downloader;
mod flags;

#[derive(Debug, Clone)]
pub struct SPathBuf(PathBuf);

impl ToString for SPathBuf {
    fn to_string(&self) -> String {
        self.0.clone().into_os_string().into_string().unwrap()
    }
}

/// Главная структура для управления процессом игры
#[derive(Debug)]
pub struct GameManager {
    min_use_memory: ByteSize,
    max_use_memory: ByteSize,
    username: String,
    path_to_minecraft: SPathBuf,
    path_to_java: SPathBuf,
}

impl Default for GameManager {
    fn default() -> Self {
        Self {
            min_use_memory: ByteSize::gib(3),
            max_use_memory: ByteSize::gib(4),
            username: "test_player".to_string(),
            path_to_minecraft: SPathBuf(crate::path::get_app_folder()),
            path_to_java: SPathBuf(crate::path::get_app_folder()), // TODO
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
        log::error!("flags: {:?}", flags);
        log::error!("flags size: {}", flags.len());

        let mut command = Command::new(
            "/home/gladi/.tlauncher/mojang_jre/java-runtime-beta/linux/java-runtime-beta/bin/java",
        )
        .args(flags)
        .current_dir(&self.path_to_minecraft.0)
        .spawn()?;
        command.wait()?;
        log::warn!("output: {:?}", command.stdout);
        log::warn!("command: {:?}", command);

        Ok(())
    }

    pub fn is_installed(&self) -> anyhow::Result<bool> {
        let path = crate::path::get_app_folder().join("servers.dat");

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
