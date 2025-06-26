use app::{ prelude::*, Keyboard, Audio, Device };

#[tokio::main]
async fn main() -> Result<()> {
    // init logger:
    log::set_logger(&*LOGGER).map_err(Error::from)?;
    log::set_max_level(log::LevelFilter::Info);
    
    // init config:
    CONFIG.lock().await.init();
    // let cfg = CONFIG.lock().await.clone();
    
    // init volume controller:
    let audio = Audio::new(
        root_path("/bin/svv/svv.exe")?,
        root_path("/bin/svcl/svcl.exe")?
    )?;
    // init keyboard:
    // let mut keyboard = Keyboard::new()?;

    info!("Found audio device list: \n{}",
        audio.get_devices().await?.into_iter()
            .enumerate()
            .map(|(i, Device { name, kind, is_active })|
                fmt!("{i}. '{name}' — {kind}{}", if is_active {" (active)"}else{""})
            )
            .collect::<Vec<_>>()
            .join("\n")
    );

    dbg!(audio.get_media_volume().await?);
    dbg!(audio.set_media_volume(50).await?);

    Ok(())
}
