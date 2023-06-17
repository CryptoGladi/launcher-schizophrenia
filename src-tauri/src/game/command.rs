use crate::exit_unwrap::ExitUnwrap;
use crate::game::downloader::Progress::*;
use crate::game::Game;
use log::*;
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};
use tauri::Window;

pub mod event {
    pub const PROGRESS_DOWLOADING: &str = "progress-downloading";
    pub const PROGRESS_DECOMPESSING: &str = "progress-decompressing";
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

    info!("running game: nickname: {}", nickname);
    let game = Game {
        username: nickname,
        ..Default::default()
    };

    info!("{:?}", game.run());
}

#[tauri::command]
pub fn game_is_installed() -> bool {
    return true;
    let game = Game::default();
    let is_installed = game.game_is_installed().exit_unwrap();

    info!("game is installed?: {}", is_installed);
    is_installed
}

#[tauri::command]
pub async fn install_game(window: Window) {
    let game = Game::default();

    info!("run install game");

    game.download_game(move |progress| match progress {
        Downloading(e) => window.emit(event::PROGRESS_DOWLOADING, e).exit_unwrap(),
        Decompressing(e) => window.emit(event::PROGRESS_DECOMPESSING, e).exit_unwrap(),
    })
    .await
    .exit_unwrap();

    info!("done download game!");
}
