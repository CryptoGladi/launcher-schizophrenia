//! Главный модуль, который отвечает за запуск и настройку игры

use self::downloader::{Downloader, Progress};
use self::java::JavaManager;
use bytesize::ByteSize;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub mod command;
mod downloader;
mod flags;
mod java;

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
    nickname: String,
    path_to_minecraft: SPathBuf,
    java: JavaManager,
}

impl Default for GameManager {
    fn default() -> Self {
        Self {
            min_use_memory: ByteSize::gib(3),
            max_use_memory: ByteSize::gib(4),
            nickname: "test_player".to_string(),
            path_to_minecraft: SPathBuf(crate::path::get_app_folder()),
            java: JavaManager {},
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
        log::info!("flags: {:?}", flags);

        let mut command = Command::new(self.java.get_exec())
            .args(flags)
            .current_dir(&self.path_to_minecraft.0)
            .stdout(Stdio::piped())
            .spawn()?;

        command.wait()?;
        log::info!("output command minecraft: {:?}", command);

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
        self.java.init();

        Ok(())
    }
}
