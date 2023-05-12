use bytesize::ByteSize;
use sysinfo::{System, SystemExt};

mod error;
mod flags;
mod download;

#[derive(Debug)]
pub struct Game {
    min_use_memory: ByteSize,
    max_use_memory: ByteSize,
    username: String,
}

impl Default for Game {
    fn default() -> Self {
        let system_info = System::new_all();
        let max_use_memory = ByteSize::b(system_info.total_memory() / 2);

        Self {
            min_use_memory: ByteSize::gib(1),
            max_use_memory,
            username: "test_player".to_string(),
        }
    }
}

impl Game {
    pub fn run(&self) {
        let flags = flags::get_flags(&self);
        log::error!("flags: {}", flags);
    }
}
