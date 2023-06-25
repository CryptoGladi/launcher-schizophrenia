//! Уставнока java

use std::fs;
use std::path::PathBuf;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

macro_rules! impl_folder_name {
    ($name:tt, $os:tt, $filename:tt) => {
        #[cfg(target_os = $os)]
        static FOLDER_NAME: &str = $name;

        #[cfg(target_os = $os)]
        static FILENAME: &str = $filename;
    };
}

impl_folder_name!("linux", "linux", "java");
impl_folder_name!("windows", "windows", "java.exe");

#[derive(Debug, Default)]
pub struct JavaManager {}

impl JavaManager {
    pub fn get_exec(&self) -> PathBuf {
        crate::path::get_app_folder()
            .join("java")
            .join(FOLDER_NAME)
            .join("bin")
            .join(FILENAME)
    }

    pub fn init(&self) {
        #[cfg(target_os = "linux")]
        {
            // https://askubuntu.com/questions/229589/how-to-make-a-file-e-g-a-sh-script-executable-so-it-can-be-run-from-a-termi
            fs::set_permissions(self.get_exec(), fs::Permissions::from_mode(0o770)).unwrap();
        }
    }
}
