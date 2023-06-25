use crate::exit_unwrap::ExitUnwrap;
use open::that as open;
use sysinfo::{System, SystemExt};

const DISCORD_URL: &str = "https://discord.gg/JSM8f42Tvx";
const REPOSITORY_URL: &str = "https://github.com/CryptoGladi/launcher-schizophrenia";

#[tauri::command]
pub fn open_url_discord() {
    log::debug!("run open_url_discord");
    open(DISCORD_URL).exit_unwrap();
}

#[tauri::command]
pub fn open_folder_app() {
    log::debug!("run open_folder_app");
    open(crate::path::get_app_folder().exit_unwrap()).exit_unwrap();
}

#[tauri::command]
pub fn get_total_memory() -> u64 {
    log::debug!("run get_total_memory");
    System::new_all().total_memory() / bytesize::MIB
}

#[tauri::command]
pub fn open_url_repository() {
    log::debug!("run open_url_repository");
    open(REPOSITORY_URL).exit_unwrap();
}
