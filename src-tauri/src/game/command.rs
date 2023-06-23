use crate::config::Config;
use crate::exit_unwrap::ExitUnwrap;
use crate::game::downloader::Progress::*;
use crate::game::GameManager;
use log::*;
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
use tauri::Window;

pub mod event {
    pub const PROGRESS_DOWLOADING: &str = "progress-downloading";
    pub const PROGRESS_DECOMPESSING: &str = "progress-decompressing";
    pub const GAME_STARTED: &str = "game-started";
    pub const GAME_ENDED: &str = "game-ended";
}

#[tauri::command]
pub async fn run_game(window: Window, nickname: String) {
    if nickname.is_empty() {
        MessageDialogBuilder::new("Вы не указали имя игрока", "Ошибка запуска игры")
            .buttons(MessageDialogButtons::Ok)
            .kind(MessageDialogKind::Error)
            .parent(&window)
            .show(|_| {});

        return;
    }

    let config = Config::load().exit_unwrap();

    info!("running game: nickname: {}", nickname);

    window.emit(event::GAME_STARTED, ()).exit_unwrap();
    let game = GameManager {
        nickname,
        max_use_memory: config.max_use_memory,
        min_use_memory: config.min_use_memory,
        ..Default::default()
    };

    info!("{:?}", game.run());
    window.emit(event::GAME_ENDED, ()).exit_unwrap();
}

#[tauri::command]
pub fn game_is_installed() -> bool {
    let game = GameManager::default();
    let is_installed = game.is_installed().exit_unwrap();

    info!("game is installed?: {}", is_installed);
    is_installed
}

#[tauri::command]
pub async fn install_game(window: Window) {
    let game = GameManager::default();

    info!("run install game");

    game.download(move |progress| match progress {
        Downloading(e) => window.emit(event::PROGRESS_DOWLOADING, e).exit_unwrap(),
        Decompressing(e) => window.emit(event::PROGRESS_DECOMPESSING, e).exit_unwrap(),
    })
    .await
    .exit_unwrap();

    info!("done download game!");
}
