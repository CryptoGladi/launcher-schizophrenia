use super::Config;
use crate::exit_unwrap::ExitUnwrap;

#[tauri::command]
pub fn config_save(config_for_save: Config) {
    config_for_save.save().exit_unwrap();
}

#[tauri::command]
pub fn config_load() -> Config {
    Config::load().exit_unwrap()
}