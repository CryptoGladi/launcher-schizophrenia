// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod command;
pub mod config;
pub mod discord_rpc;
pub mod exit_unwrap;
mod game;
mod logger;
pub mod path;

use crate::game::CHILD_PROCESS_GAME;
use color_eyre::eyre::Result;
use log::info;

fn main() -> Result<()> {
    color_eyre::install()?;

    if !crate::path::get_app_folder().is_dir() {
        std::fs::create_dir_all(crate::path::get_app_folder()).unwrap();
    }

    logger::init_logger().unwrap();
    let _drpc = std::thread::spawn(|| discord_rpc::run().unwrap());

    info!("running done! version 1.0");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            game::command::run_game,
            game::command::game_is_installed,
            game::command::install_game,
            config::command::config_load,
            config::command::config_save_nickname,
            config::command::config_save_max_use_memory,
            command::open_url_discord,
            command::open_folder_app,
            command::open_url_repository,
            command::get_total_memory,
        ])
        .on_window_event(|event| {
            if let tauri::WindowEvent::Destroyed = event.event() {
                if let Some(child) = CHILD_PROCESS_GAME.lock().unwrap().as_mut() {
                    log::warn!("killing process");
                    child.kill().unwrap();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
