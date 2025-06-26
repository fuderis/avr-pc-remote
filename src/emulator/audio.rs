use crate::prelude::*;
use std::process::Command;
use csv::Reader;

/// The device kind
#[derive(Debug, Display, Clone, Eq, PartialEq)]
pub enum DeviceKind {
    Media,
    Micro
}

impl DeviceKind {
    /// check for media device type
    pub fn is_media_device(&self) -> bool {
        if let Self::Media = self { true }else{ false }
    }

    /// check for microphone device type
    pub fn is_micro_device(&self) -> bool {
        if let Self::Micro = self { true }else{ false }
    }
}

/// The audio device
#[derive(Debug, Display, Clone)]
pub struct Device {
    pub name: String,
    pub kind: DeviceKind,
    pub is_active: bool,
}

/// The audio controller
#[derive(Debug, Display, Clone)]
pub struct Audio {
    svv_path: PathBuf,
    svcl_path: PathBuf,
}

impl Audio {
    /// Creates a new audio controller
    pub fn new<P: AsRef<Path>>(svv_path: P, svcl_path: P) -> Result<Self> {
        Ok(Self {
            svv_path: svv_path.as_ref().to_path_buf(),
            svcl_path: svcl_path.as_ref().to_path_buf(),
        })
    }

    /// Get devices list
    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let output = Command::new(&self.svv_path)
            .arg("/scomma")
            .output()
            .map_err(|_| Error::FailedReadDevicesList)?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stdout_clean = stdout.trim_start_matches('\u{feff}');
        
        let mut reader = Reader::from_reader(stdout_clean.as_bytes());
        let mut audio_devices = vec![];
        
        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    err!("CSV parsing error: {}", e);
                    continue;
                }
            };

            if record.get(1) != Some("Device") { continue }
            
            let kind = match &record.get(2).map(|s| s.to_string()).unwrap_or_default()[..] {
                "Render" => DeviceKind::Media,
                "Capture" => DeviceKind::Micro,
                _ => continue
            };
            
            let is_active = record.get(7) == Some("Active") && (record.get(5) == Some("Render") || record.get(5) == Some("Capture"));

            if let Some(name) = record.get(0) {
                // skip empty names
                if name.is_empty() { continue; }
                
                audio_devices.push(Device {
                    name: name.to_string(),
                    kind,
                    is_active,
                });
            }
        }

        if audio_devices.is_empty() {
            return Err(Error::FoundNoDevices.into());
        }

        Ok(audio_devices)
    }

    /// Get audio devices list
    pub async fn get_media_devices(&self) -> Result<Vec<Device>> {
        Ok(self.get_devices().await?.into_iter().filter(|device| device.kind.is_media_device()).collect::<Vec<_>>())
    }

    /// Get microphone devices list
    pub async fn get_micro_devices(&self) -> Result<Vec<Device>> {
        Ok(self.get_devices().await?.into_iter().filter(|device| device.kind.is_micro_device()).collect::<Vec<_>>())
    }
    
    /// Get a current media device
    pub async fn get_active_media_device(&self) -> Result<Device> {
        for device in self.get_media_devices().await? {
            if device.is_active {
                return Ok(device);
            }
        }

        Err(Error::FoundNoActiveDevice.into())
    }

    /// Get a current microphone device
    pub async fn get_active_micro_device(&self) -> Result<Device> {
        for device in self.get_micro_devices().await? {
            if device.is_active {
                return Ok(device);
            }
        }

        Err(Error::FoundNoActiveDevice.into())
    }

    /// Internal helper to set current device
    async fn set_media_device(&self, name: &str) -> Result<()> {
        for device in &self.get_media_devices().await? {
            if device.name == name {
                let status = Command::new(&self.svv_path)
                    .arg("/SetDefault")
                    .arg(name)
                    .arg("all")  // all = Console, Multimedia, Communications
                    .status()?;
                
                if status.success() {
                    info!("Switched to '{}' audio device", name);
                } else {
                    err!("Failed to switch to '{}' audio device", name);
                }

                return Ok(());
            }
        }

        Err(Error::InvalidDeviceName(name.to_owned()).into())
    }

    /// Get current audio volume (0-100)
    pub async fn get_media_volume(&self) -> Result<usize> {
        let device = self.get_active_media_device().await?;

        let output = Command::new(&self.svcl_path)
            .arg("/GetPercent")
            .arg(device.name)
            .output()?;
        
        dbg!(&output);
        
        if !output.status.success() {
            return Err("Failed to get volume".into());
        }
        
        // parse volume (multiplied by 10)
        let stdout = String::from_utf8_lossy(&output.stdout);
        let volume_times10: usize = stdout.trim().parse().unwrap_or(0);

        Ok(volume_times10 / 10)  // Convert to 0-100 scale
    }

    /// Set audio volume (0-100)
    pub async fn set_media_volume(&self, volume: usize) -> Result<()> {
        let device = self.get_active_media_device().await?;

        let status = Command::new(&self.svv_path)
            .arg("/SetVolume")
            .arg(device.name)
            .arg(volume.to_string())
            .status()?;
        
        if status.success() {
            info!("Set audio volume to {}%", volume);
        } else {
            err!("Failed to set audio volume");
        }
        Ok(())
    }

    /// Increase volume by delta
    pub async fn plus_media_volume(&self, delta: usize) -> Result<()> {
        let current = self.get_media_volume().await?;
        let new_volume = (current + delta).min(100);
        self.set_media_volume(new_volume).await
    }

    /// Decrease volume by delta
    pub async fn minus_media_volume(&self, delta: usize) -> Result<()> {
        let current = self.get_media_volume().await?;
        let new_volume = current.saturating_sub(delta);
        self.set_media_volume(new_volume).await
    }

    /// Toggle mute/unmute
    pub async fn toggle_media_mute(&self) -> Result<()> {
        let device = self.get_active_media_device().await?;

        let status = Command::new(&self.svv_path)
            .arg("/Switch")
            .arg(&device.name)
            .status()?;
        
        if status.success() {
            info!("Toggled mute for '{}'", device.name);
        } else {
            err!("Failed to toggle mute");
        }
        Ok(())
    }
}
