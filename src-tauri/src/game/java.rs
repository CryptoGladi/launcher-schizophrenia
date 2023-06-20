//! Уставнока java

use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

macro_rules! impl_folder_name {
    ($name:tt, $os:tt) => {
        #[cfg(target_os = $os)]
        static FOLDER_NAME: &str = $name;
    };
}

impl_folder_name!("linux", "linux");
impl_folder_name!("windows", "windows");

#[derive(Debug)]
pub struct JavaManager {}

impl JavaManager {
    pub fn get_exec(&self) -> PathBuf {
        crate::path::get_app_folder()
            .join("tlauncher-libs")
            .join("mojang_jre")
            .join("java-runtime-beta")
            .join(FOLDER_NAME)
            .join("java-runtime-beta")
            .join("bin")
            .join("java")
    }

    pub fn init(&self) {
        #[cfg(target_os = "linux")]
        {
            // https://stackoverflow.com/questions/28670683/how-are-permissions-applied-to-a-file-using-set-mode
            fs::set_permissions(self.get_exec(), fs::Permissions::from_mode(0o655)).unwrap();
        }
    }
}
