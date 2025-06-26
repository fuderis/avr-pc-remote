use serialport;
use std::io::{BufReader, BufRead};
use std::thread::sleep;
use std::time::Duration;
use std::result::Result;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use enigo::{ Enigo, Key, Keyboard, Settings, Direction };


fn press_key(enigo: &mut Enigo, key: Key) -> std::io::Result<()> {
    enigo.key(key, Direction::Press).unwrap();
    Ok(())
}

fn press_2key(enigo: &mut Enigo, key1: Key, key2: Key) -> std::io::Result<()> {
    enigo.key(key1, Direction::Press).unwrap();
    enigo.key(key2, Direction::Press).unwrap();

    sleep(Duration::from_millis(100));

    enigo.key(key1, Direction::Release).unwrap();
    enigo.key(key2, Direction::Release).unwrap();

    Ok(())
}

fn press_3key(enigo: &mut Enigo, key1: Key, key2: Key, key3: Key) -> std::io::Result<()> {
    enigo.key(key1, Direction::Press).unwrap();
    enigo.key(key2, Direction::Press).unwrap();
    enigo.key(key3, Direction::Press).unwrap();

    sleep(Duration::from_millis(100));

    enigo.key(key1, Direction::Release).unwrap();
    enigo.key(key2, Direction::Release).unwrap();
    enigo.key(key3, Direction::Release).unwrap();

    Ok(())
}

fn open_website(url: &str) -> std::io::Result<()> {
    webbrowser::open(url)?;
    Ok(())
}

fn get_nircmd_path() -> PathBuf {
    let exe_path = env::current_exe().expect("Не удалось получить путь к исполняемому файлу");
    let exe_dir = exe_path.parent().expect("Не удалось получить папку с программой");
    exe_dir.join("nircmd").join("nircmd.exe") // nircmd.exe в папке ./nircmd/
}

fn switch_device(device_name: &str) -> std::io::Result<()> {
    let nircmd_path = get_nircmd_path();

    let status = Command::new(nircmd_path)
        .arg("setdefaultsounddevice")
        .arg(device_name)
        .arg("1") // 1 — для мультимедиа и коммуникаций
        .status()?;

    if status.success() {
        println!("Устройство '{}' установлено по умолчанию", device_name);
    } else {
        eprintln!("Не удалось установить устройство '{}'", device_name);
    }

    Ok(())
}

fn change_volume(delta_percent: i32) -> std::io::Result<()> {
    let nircmd_path = get_nircmd_path();

    let volume_change = delta_percent * 65536 / 100;

    let status = Command::new(nircmd_path)
        .arg("changesysvolume")
        .arg(volume_change.to_string())
        .status()?;

    if status.success() {
        println!("Громкость изменена на {}%", delta_percent);
    } else {
        eprintln!("Ошибка при изменении громкости");
    }

    Ok(())
}

fn mute_unmute() -> std::io::Result<()> {
    let nircmd_path = get_nircmd_path();

    let status = Command::new(nircmd_path)
        .arg("mutesysvolume")
        .arg("2") // 2 — переключить состояние мьюта
        .status()?;

    if status.success() {
        println!("Звук переключен (мьют/размьют)");
    } else {
        eprintln!("Ошибка при переключении мьюта");
    }

    Ok(())
}

fn media_play(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaPlayPause, Direction::Click).unwrap();
    Ok(())
}

fn media_pause(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaPlayPause, Direction::Click).unwrap();
    Ok(())
}

fn media_play_pause(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaPlayPause, Direction::Click).unwrap();
    enigo.key(Key::Space, Direction::Click).unwrap();
    Ok(())
}

fn media_stop(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaStop, Direction::Click).unwrap();
    Ok(())
}

fn media_prev(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaPrevTrack, Direction::Click).unwrap();
    Ok(())
}

fn media_next(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::MediaNextTrack, Direction::Click).unwrap();
    Ok(())
}

fn rewind_back(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::LeftArrow, Direction::Click).unwrap();
    enigo.key(Key::J, Direction::Click).unwrap();
    Ok(())
}
fn rewind_forward(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::RightArrow, Direction::Click).unwrap();
    enigo.key(Key::L, Direction::Click).unwrap();
    Ok(())
}

fn rewind_back2(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::LeftArrow, Direction::Click).unwrap();
    enigo.key(Key::K, Direction::Click).unwrap();
    Ok(())
}
fn rewind_forward2(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::RightArrow, Direction::Click).unwrap();
    enigo.key(Key::L, Direction::Click).unwrap();
    Ok(())
}


fn screen_record(enigo: &mut Enigo) -> std::io::Result<()> {
    enigo.key(Key::LControl, Direction::Press).unwrap();
    enigo.key(Key::F9, Direction::Press).unwrap();

    sleep(Duration::from_millis(100));

    enigo.key(Key::LControl, Direction::Release).unwrap();
    enigo.key(Key::F9, Direction::Release).unwrap();
    
    Ok(())
}

fn sleep_mode() -> std::io::Result<()> {
    let status = Command::new("rundll32.exe")
        .arg("powrprof.dll,SetSuspendState")
        .arg("0")
        .arg("1")
        .arg("0")
        .status()?;

    if status.success() {
        println!("Переход в спящий режим выполнен");
    } else {
        eprintln!("Ошибка при переходе в спящий режим");
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut action = "";
    let mut current_device_index = 0;

    // Укажи точные имена устройств, как они отображаются в Windows
    let devices = vec!["Speakers", "Headphones"];

    let port_name = "COM8";
    let baud_rate = 9600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()?;

    let mut reader = BufReader::new(port);
    let mut line = String::new();

    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings).unwrap();

    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => continue,
            Ok(_) => {
                let code = line.trim().to_uppercase();
                if code.is_empty() {
                    continue;
                }

                match code.as_str() {
                    // switch device
                    "FF8877" => {
                        action = "switch_device";
                        current_device_index = (current_device_index + 1) % devices.len();
                        let device_name = devices[current_device_index];
                        let _ = switch_device(device_name).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // volume up
                    "FFD827" => {
                        action = "volume_up";
                        let _ = change_volume(1).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // volume down
                    "FFDA25" => {
                        action = "volume_down";
                        let _ = change_volume(-1).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // volume mute/unmute
                    "FF9867" => {
                        action = "mute_unmute";
                        let _ = mute_unmute().map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // media play
                    "FFD22D" => {
                        action = "media_play";
                        let _ = media_play(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // media pause
                    "FF12ED" => {
                        action = "media_pause";
                        let _ = media_pause(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // media play/pause
                    "FF58A7" => {
                        action = "media_play_pause";
                        let _ = media_play_pause(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // media stop
                    "FF10EF" => {
                        action = "media_stop";
                        let _ = media_stop(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // prev track
                    "FF6897" => {
                        action = "prev_track";
                        let _ = media_prev(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // next track
                    "FF609F" => {
                        action = "next_track";
                        let _ = media_next(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // rewind back
                    "FFE21D" => {
                        action = "rewind_back";
                        let _ = rewind_back(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // rewind back
                    "FF20DF" => {
                        action = "rewind_back2";
                        let _ = rewind_back2(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // rewind forward
                    "FF22DD" => {
                        action = "rewind_forward";
                        let _ = rewind_forward(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // rewind forward
                    "FFE01F" => {
                        action = "rewind_forward2";
                        let _ = rewind_forward2(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // start/stop screen record
                    "FFD02F" => {
                        action = "screen_record";
                        let _ = screen_record(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // tab
                    "FFA05F" => {
                        action = "tab";
                        let _ = press_key(&mut enigo, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // shift-tab
                    "FF629D" => {
                        action = "shift_tab";
                        let _ = press_2key(&mut enigo, Key::LShift, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // click
                    "FF6A95" => {
                        action = "click";
                        let _ = press_key(&mut enigo, Key::Return).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // close
                    "FFA857" => {
                        action = "close";
                        let _ = press_2key(&mut enigo, Key::LControl, Key::W).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // fullscreen
                    "FFB04F" => {
                        action = "fullscreen";
                        let _ = press_key(&mut enigo, Key::F).map_err(|e| eprintln!("[ERROR] {e}"));
                        let _ = press_key(&mut enigo, Key::F11).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // switch_tab
                    "FF4AB5" => {
                        action = "switch_tab";
                        let _ = press_3key(&mut enigo, Key::LControl, Key::LShift, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    
                    // sleep mode
                    "FF5AA5" => {
                        action = "sleep_mode";
                        let _ = sleep_mode().map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // youtube
                    "FFCA35" => {
                        action = "youtube";
                        let _ = open_website("https://www.youtube.com/").map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // youtube music
                    "FF0AF5" => {
                        action = "youtube_music";
                        let _ = open_website("https://music.youtube.com/").map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // animego
                    "FF08F7" => {
                        action = "animego";
                        let _ = open_website("https://animego.me/").map_err(|e| eprintln!("[ERROR] {e}"));
                    }
                    // radio energy
                    "FFE817" => {
                        action = "radio_energy";
                        let _ = open_website("https://www.energyfm.ru/").map_err(|e| eprintln!("[ERROR] {e}"));
                    }

                    // repeat last action
                    "FFFFFFFF" => {
                        match action {
                            "volume_up" => {
                                let _ = change_volume(3).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            "volume_down" => {
                                let _ = change_volume(-3).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            "tab" => {
                                let _ = press_key(&mut enigo, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            "shift_tab" => {
                                let _ = press_2key(&mut enigo, Key::LShift, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            // switch_tab
                            "switch_tab" => {
                                let _ = press_3key(&mut enigo, Key::LControl, Key::LShift, Key::Tab).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            // rewind back
                            "rewind_back" => {
                                let _ = rewind_back(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                            }
                            // rewind back
                            "rewind_back2" => {
                                let _ = rewind_back2(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            // rewind forward
                            "rewind_forward" => {
                                let _ = rewind_forward(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                            }
                            // rewind forward
                            "rewind_forward2" => {
                                let _ = rewind_forward2(&mut enigo).map_err(|e| eprintln!("[ERROR] {e}"));
                            }

                            _ => {}
                        }
                    }

                    _ => {
                        println!("Неизвестный код '{}'", code);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
            Err(e) => {
                eprintln!("Ошибка при чтении порта: {}", e);
                break;
            }
        }
    }

    Ok(())
}
