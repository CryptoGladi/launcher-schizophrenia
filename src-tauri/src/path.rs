use std::path::PathBuf;

pub fn get_path_to_folder() -> PathBuf {
    dirs::config_local_dir().unwrap().join("mine-schizophrenia")
}
