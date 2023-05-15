// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;

use color_eyre::eyre::Result;
use log::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    simple_logger::SimpleLogger::default()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    info!("running done!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            game::command::run_game,
            game::command::game_is_installed,
            game::command::install_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
