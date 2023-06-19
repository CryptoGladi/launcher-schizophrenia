use std::path::PathBuf;

pub fn get_app_folder() -> PathBuf {
    dirs::config_local_dir().unwrap().join("mine-schizophrenia")
}
