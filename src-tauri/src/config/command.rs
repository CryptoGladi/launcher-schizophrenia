use bytesize::ByteSize;

use super::Config;
use crate::exit_unwrap::ExitUnwrap;

#[tauri::command]
pub fn config_save_nickname(nickname: String) {
    let mut config = Config::load().exit_unwrap();
    config.nickname = nickname;
    config.save().exit_unwrap();
}

#[tauri::command]
pub fn config_save_max_use_memory(max_use_memory: u64) {
    let mut config = Config::load().exit_unwrap();
    config.max_use_memory = ByteSize::mib(max_use_memory);
    config.save().exit_unwrap();
}

#[tauri::command]
pub fn config_load() -> Config {
    Config::load().exit_unwrap()
}
