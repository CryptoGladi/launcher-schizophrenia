use std::path::PathBuf;

pub fn get_config() -> PathBuf {
    dirs::config_local_dir().unwrap().join("mine-schizophrenia")
}
