use crate::exit_unwrap::ExitUnwrap;
use bytesize::ByteSize;
use open::that as open_url;
use sysinfo::{System, SystemExt};

const DISCORD_URL: &str = "https://discord.gg/JSM8f42Tvx";

#[tauri::command]
pub fn open_url_discord() {
    open_url(DISCORD_URL).exit_unwrap();
}

#[tauri::command]
pub fn get_total_memory() -> u64 {
    System::new_all().total_memory() / bytesize::MIB
}
