pub mod error;      pub use error::{ StdResult, Result, Error };
pub mod logger;     pub use logger::Logger;
pub mod config;     pub use config::Config;
pub mod prelude;    use prelude::*;

pub static LOGGER: Lazy<Logger> = Lazy::new(|| Logger::new());
pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Config::new("config.json").unwrap_or_default());

pub mod emulator;   pub use emulator::{ Media, Device, DeviceKind, Keyboard, Key, Mouse };
pub mod binds;      pub use binds::{ Bind, Action };

/// Generates path by program root path 
pub fn root_path<P: AsRef<Path>>(relative_path: P) -> Result<PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| Error::InvalidRootPath)?;

    // converting path to string:
    let rel_str = relative_path.as_ref().to_str().ok_or_else(|| Error::InvalidPath)?;
    
    // removing start symbol '/' if it's exists
    let rel_str = if rel_str.starts_with('/') {
        &rel_str[1..]
    } else {
        rel_str
    };

    Ok(exe_dir.join(rel_str))
}

/// Generates an unique ID
pub fn uniq_id() -> String {
    use std::time::{ SystemTime, UNIX_EPOCH };
    
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}
