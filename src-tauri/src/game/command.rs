use crate::game::Game;
use log::*;

#[tauri::command]
pub async fn run_game() {
    let game = Game::default();

    info!("{:?}", game.run());
}

#[tauri::command]
pub fn game_is_installed() -> bool {
    let game = Game::default();
    let is_installed = game.game_is_installed().unwrap();

    info!("game is installed?: {}", is_installed);
    is_installed
}

#[tauri::command]
pub async fn install_game() {
    let game = Game::default();
    info!("run install game");
    
    if game_is_installed() {
        return ();
    }

    game.download_game().await.unwrap();
    
    info!("done download game!");
}
