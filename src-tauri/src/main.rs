// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;

use color_eyre::eyre::Result;
use log::*;

#[tauri::command]
fn my_custom_command() {
    let game = game::Game::default();

    info!("{:?}", game.run());
}

fn main() -> Result<()> {
    color_eyre::install()?;
    simple_logger::SimpleLogger::default()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
    info!("running done!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
