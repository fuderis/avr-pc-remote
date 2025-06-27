use app::{ prelude::*, Bind, Action, Keyboard, Key, Mouse, Media, Device };
use core::time::Duration;
use std::{ io::{ BufReader, BufRead }, process::Command };

#[tokio::main]
async fn main() -> Result<()> {
    run().await?;
    Ok(())
}

async fn run() -> Result<()> {
    // init logger:
    log::set_logger(&*LOGGER).map_err(Error::from)?;
    log::set_max_level(log::LevelFilter::Info);
    
    // init config:
    CONFIG.lock().await.init();
    let cfg = CONFIG.lock().await.clone();

    // init media controller:
    let mut media = Media::new(root_path("/bin")?, Some(|name| !name.contains("SteelSeries"))).await?;

    // init keyboard:
    let mut keyboard = Keyboard::new()?;

    // init mouse:
    let mut mouse = Mouse::new()?;

    // print audio device list:
    info!("Audio device list: \n{}",
        media.get_devices().iter()
            .enumerate()
            .map(|(i, Device { name, kind, is_active })|
                fmt!("{i}. '{name}' — {kind}{}", if *is_active {" (active)"}else{""})
            )
            .collect::<Vec<_>>()
            .join("\n")
    );

    let port_name = fmt!("COM{}", cfg.com_port);
    let baud_rate = cfg.baud_rate as u32;

    let port = serialport::new(&port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut com_reader = BufReader::new(port);
    let mut line = String::new();

    let binds = cfg.binds;
    let mut last_code = str!();
    let mut last_action = Instant::now();
    let mut last_update = Instant::now();
    let action_interval = Duration::from_millis(1000);
    let update_interval = Duration::from_millis(2000);
    let repeat_timeout = Duration::from_millis(25);

    info!("Reading remote inputs..");
    
    loop {
        line.clear();

        if last_action.elapsed() >= action_interval {
            if last_update.elapsed() >= update_interval {
                media.update_info().await?;
                last_update = Instant::now();
            }
        } else {
            last_update = Instant::now();
        }

        match com_reader.read_line(&mut line) {
            Ok(0) => continue,
            Ok(_) => {
                let code = line.trim().to_uppercase();
                if code.is_empty() { continue }

                // repeat last bind:
                if code == "FFFFFFFF" {
                    if last_action.elapsed() < repeat_timeout {
                        continue;
                    }
                    
                    if last_code.is_empty() { continue }

                    if let Err(e) = execute_bind(binds.get(&last_code).unwrap(), &mut media, &mut keyboard, &mut mouse, true).await {
                        err!("Error with executing bind: {e}");
                    }
                } else {
                    // execute exists bind:
                    if let Some(bind) = binds.get(&code) {
                        info!("Pressed '{code}', bind '{}'.", bind.name);

                        last_code = if bind.repeat { code }else{ str!() };
                        
                        if let Err(e) = execute_bind(bind, &mut media, &mut keyboard, &mut mouse, false).await {
                            err!("Error with executing bind: {e}");
                        }
                    }
                    // unbinded code:
                    else {
                        info!("Pressed '{code}', no binds exists..");
                        last_code = str!();
                    }
                }

                last_action = Instant::now();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                err!("Error with reading '{port_name}' port: {e}");
                return Err(e.into());
            }
        }
    }
}

static MENU_MODE: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(0));
const MOUSE_STEPS: (i32, i32) = (30, 100);
const SCROLL_STEPS: (i32, i32) = (2, 5);
const VOLUME_STEPS: (i32, i32) = (2, 5);

/// Execute remote bind
async fn execute_bind(bind: &Bind, media: &mut Media, keyboard: &mut Keyboard, mouse: &mut Mouse, is_repeated: bool) -> Result<()> {
    match &bind.action {
        // execute special handler:
        Action::Handler { handler: name } => {
            match name.as_ref() {
                "switch-audio" => {
                    media.switch_next_audio_device().await?;
                }

                "mute-unmute" => {
                    media.switch_audio_mute().await?;
                    media.switch_micro_mute().await?;
                }

                "navigation" => {
                    let mut mode = MENU_MODE.lock().await;

                    if *mode >= 2 {
                        *mode = 0;
                    } else {
                        *mode += 1;
                    }

                    match *mode {
                        0 => info!("Switched to media mode"),
                        1 => info!("Switched to mouse emulation mode"),
                        2 => info!("Switched to selector mode"),

                        _ => err!("Unknown menu mode")
                    }
                }
                "nav-left" => {
                    match *MENU_MODE.lock().await {
                        0 => {
                            let volume = media.decrease_audio_volume(if is_repeated { VOLUME_STEPS.1 }else{ VOLUME_STEPS.0 }).await?;
                            info!("Set audio volume to {volume}%");
                        }

                        1 => {
                            let step: i32 = if is_repeated { MOUSE_STEPS.1 }else{ MOUSE_STEPS.0 };
                            mouse.move_x(-step)?;
                            info!("Move mouse left by {step}px", );
                        }

                        2 => {
                            keyboard.press_all(&[Key::Shift, Key::Tab], true).await?;
                            sleep(Duration::from_millis(100)).await;
                            keyboard.release_all(&[Key::Shift, Key::Tab]).await?;
                            info!("Selected previous element");
                        }

                        _ => err!("Unknown menu mode")
                    }
                }
                "nav-right" => {
                    match *MENU_MODE.lock().await {
                        0 => {
                            let volume = media.increase_audio_volume(if is_repeated { VOLUME_STEPS.1 }else{ VOLUME_STEPS.0 }).await?;
                            info!("Set audio volume to {volume}%");
                        }

                        1 => {
                            let step: i32 = if is_repeated { MOUSE_STEPS.1 }else{ MOUSE_STEPS.0 };
                            mouse.move_x(step)?;
                            info!("Move mouse right by {step}px", );
                        }

                        2 => {
                            keyboard.press(&Key::Tab, true).await?;
                            info!("Selected next element");
                        }

                        _ => err!("Unknown menu mode")
                    }
                }
                "nav-up" => {
                    match *MENU_MODE.lock().await {
                        0 => {
                            if !is_repeated {
                                keyboard.press(&Key::MediaNextTrack, false).await?;
                                info!("Switched to next track");
                            }
                        }

                        1 => {
                            let step: i32 = if is_repeated { MOUSE_STEPS.1 }else{ MOUSE_STEPS.0 };
                            mouse.move_y(-step)?;
                            info!("Move mouse top by {step}px", );
                        }

                        2 => {
                            keyboard.press_all(&[Key::Shift, Key::Tab], true).await?;
                            sleep(Duration::from_millis(100)).await;
                            keyboard.release_all(&[Key::Shift, Key::Tab]).await?;
                            info!("Selected previous element");
                        }

                        _ => err!("Unknown menu mode")
                    }
                }
                "nav-down" => {
                    match *MENU_MODE.lock().await {
                        0 => {
                            if !is_repeated {
                                keyboard.press(&Key::MediaPrevTrack, false).await?;
                                info!("Switched to previos track");
                            }
                        }

                        1 => {
                            let step: i32 = if is_repeated { MOUSE_STEPS.1 }else{ MOUSE_STEPS.0 };
                            mouse.move_y(step)?;
                            info!("Move mouse bottom by {step}px", );
                        }
                        
                        2 => {
                            keyboard.press(&Key::Tab, true).await?;
                            info!("Selected next element");
                        }

                        _ => err!("Unknown menu mode")
                    }
                }
                "nav-center" => {
                    match *MENU_MODE.lock().await {
                        0 => {
                            keyboard.press(&Key::MediaPlayPause, false).await?;
                            info!("Switched media play/pause");
                        }

                        1 => {
                            mouse.press_left(false)?;
                            info!("Pressed left mouse button");
                        }

                        2 => {
                            keyboard.press(&Key::Enter, false).await?;
                            info!("Pressed enter button");
                        }

                        _ => err!("Unknown menu mode")
                    }
                }

                "scroll-up" => {
                    let step: i32 = if is_repeated { SCROLL_STEPS.1 }else{ SCROLL_STEPS.0 };
                    mouse.scroll_y(-step)?;
                    info!("Scroll up by {step}px");
                }
                "scroll-down" => {
                    let step: i32 = if is_repeated { SCROLL_STEPS.1 }else{ SCROLL_STEPS.0 };
                    mouse.scroll_y(step)?;
                    info!("Scroll down by {step}px");
                }

                "sleep-mode" => {
                    let status = Command::new("rundll32.exe")
                        .arg("powrprof.dll,SetSuspendState")
                        .arg("0")
                        .arg("1")
                        .arg("0")
                        .status()?;

                    if status.success() {
                        info!("PC switched to sleep mode");
                    } else {
                        err!("Failed switch PC to sleep mode");
                    }
                }
                
                _ => err!("Unknown handler with name '{name}'")
            }
        },

        // press keyboard shortcut:
        Action::Shortcut { shortcut: keys } => {
            keyboard.press_all(keys, true).await?;

            sleep(Duration::from_millis(100)).await;

            keyboard.release_all(keys).await?;
        },

        // press keyboard key:
        Action::Press { press: keys } => {
            keyboard.press_all(keys, false).await?;
        },

        // open website:
        Action::Open { open: url } => {
            let url = if url.starts_with("https:") { url }else{ &fmt!("https://{url}") };
            webbrowser::open(url)?;
        },
    }

    Ok(())
}
