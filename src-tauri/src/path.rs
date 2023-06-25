use std::path::PathBuf;

pub fn get_app_folder() -> anyhow::Result<PathBuf> {
    if let Some(path) = dirs::config_local_dir() {
        return Ok(path);
    }

    anyhow::bail!("get app folder");
}
