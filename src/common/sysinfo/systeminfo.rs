use clap::builder::Str;

#[derive(Clone)]

pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
}

pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
    }
}
