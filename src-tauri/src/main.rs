// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod command;
pub mod config;
pub mod exit_unwrap;
mod game;
pub mod path;

use color_eyre::eyre::Result;
use log::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    simple_logger::SimpleLogger::default()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    info!("running done!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            game::command::run_game,
            game::command::game_is_installed,
            game::command::install_game,
            config::command::config_load,
            config::command::config_save,
            command::open_url_discord
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
