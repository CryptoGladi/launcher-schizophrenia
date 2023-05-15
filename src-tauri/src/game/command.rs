use crate::game;
use log::*;

#[tauri::command]
pub async fn run_game() {
    let game = game::Game::default();

    info!("{:?}", game.run());
}

#[tauri::command]
pub fn game_is_installed() -> bool {
    let game = game::Game::default();
    let is_installed = game.game_is_installed().unwrap();

    info!("game is installed?: {}", is_installed);
    is_installed
}