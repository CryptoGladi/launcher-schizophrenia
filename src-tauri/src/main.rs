// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod command;
pub mod config;
pub mod discord_rpc;
pub mod exit_unwrap;
mod game;
mod logger;
pub mod path;

use color_eyre::eyre::Result;
use log::*;

fn main() -> Result<()> {
    color_eyre::install()?;

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    logger::init_logger().unwrap();
    let _drpc = discord_rpc::run().unwrap();

    info!("running done!");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            game::command::run_game,
            game::command::game_is_installed,
            game::command::install_game,
            config::command::config_load,
            config::command::config_save_nickname,
            command::open_url_discord
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
