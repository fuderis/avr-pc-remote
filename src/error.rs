use macron::{ Display, From, Error };

// The std result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
// The application result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static>>;

// The application error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    String(String),

    #[from]
    Logger(log::SetLoggerError),

    #[display = "Couldn't get the directory of the root path"]
    InvalidRootPath,

    #[display = "The path contains invalid UTF-8 characters"]
    InvalidPath,

    #[display = "No audio devices set"]
    FoundNoDevices,

    #[display = "Failed to get devices list"]
    FailedReadDevicesList,

    #[display = "Found no audio device named as '{0}'"]
    DeviceNotFound(String),

    #[display = "Found no active audio device"]
    ActiveDeviceNotFound,
}
