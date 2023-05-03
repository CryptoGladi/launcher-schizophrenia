use bytesize::ByteSize;
use sysinfo::{System, SystemExt};

mod error;

#[derive(Debug)]
enum GCJava {
    ConcMarkSweepGC,
    G1GC,
}

#[derive(Debug)]
pub struct Game {
    min_use_memory: u64,
    max_use_memory: u64,
    gc_java: GCJava,
    username: String,
}

impl Default for Game {
    fn default() -> Self {
        let system_info = System::new_all();
        let max_use_memory = system_info.total_memory() / 2;

        let gc_java = if max_use_memory > ByteSize::kib(2).as_u64() {
            GCJava::G1GC
        } else {
            GCJava::ConcMarkSweepGC
        };

        Self {
            min_use_memory: ByteSize::gib(1).as_u64(),
            max_use_memory,
            gc_java,
            username: "test_player".to_string(),
        }
    }
}

impl Game {
    
}