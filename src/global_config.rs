use crate::cmd::Cmd;
use std::env::consts::OS;

pub struct GlobalConfig {
    boot_lib_path: String,
    os: &'static str,
    verbose: bool,
}

pub static mut GLOBAL_CONFIG: Option<GlobalConfig> = None;

impl GlobalConfig {
    fn new(cmd: &Cmd) -> GlobalConfig {
        return GlobalConfig {
            boot_lib_path: "".to_string(),
            os: OS,
            verbose: cmd.verbose_class,
        };
    }

    pub fn init(cmd: &Cmd) {
        unsafe {
            if GLOBAL_CONFIG.is_none() {
                GLOBAL_CONFIG = Some(Self::new(cmd))
            }
        }
    }

    pub fn instance() -> &'static GlobalConfig {
        unsafe {
            return GLOBAL_CONFIG.as_ref().unwrap();
        }
    }
}
