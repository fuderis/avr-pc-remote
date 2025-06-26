use crate::prelude::*;
use enigo::{ Enigo, Key as EnigoKey, Keyboard as EnigoKeyboard, Settings, Direction };

/// The keyboard key
#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum Key {
    Add,
    Alt,
    Backspace,
    Break,
    Begin,
    Cancel,
    CapsLock,
    Clear,
    Command,
    Control,
    Decimal,
    Delete,
    Divide,
    DownArrow,
    End,
    Escape,
    Execute,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    Find,
    Hangul,
    Hanja,
    Help,
    Home,
    Insert,
    Kanji,
    LControl,
    LeftArrow,
    Linefeed,
    LMenu,
    LShift,
    MediaNextTrack,
    MediaPlayPause,
    MediaPrevTrack,
    MediaStop,
    Meta,
    ModeChange,
    Multiply,
    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    Option,
    PageDown,
    PageUp,
    Pause,
    Print,
    PrintScr,
    RControl,
    Redo,
    Enter,
    RightArrow,
    RShift,
    ScrollLock,
    Select,
    ScriptSwitch,
    Shift,
    ShiftLock,
    Space,
    Subtract,
    Super,
    SysReq,
    Tab,
    Undo,
    UpArrow,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    MicMute,
    Windows,
    #[serde(untagged)]
    Unicode(char),
    #[serde(untagged)]
    Number(u32),
}

impl ::std::convert::Into<EnigoKey> for Key {
    fn into(self) -> EnigoKey {
        match self {
            Self::Add => EnigoKey::Add,
            Self::Alt => EnigoKey::Alt,
            Self::Backspace => EnigoKey::Backspace,
            Self::Cancel => EnigoKey::Cancel,
            Self::CapsLock => EnigoKey::CapsLock,
            Self::Clear => EnigoKey::Clear,
            Self::Control => EnigoKey::Control,
            Self::Decimal => EnigoKey::Decimal,
            Self::Delete => EnigoKey::Delete,
            Self::Divide => EnigoKey::Divide,
            Self::DownArrow => EnigoKey::DownArrow,
            Self::End => EnigoKey::End,
            Self::Escape => EnigoKey::Escape,
            Self::Execute => EnigoKey::Execute,
            Self::Num0 => EnigoKey::Num0,
            Self::Num1 => EnigoKey::Num1,
            Self::Num2 => EnigoKey::Num2,
            Self::Num3 => EnigoKey::Num3,
            Self::Num4 => EnigoKey::Num4,
            Self::Num5 => EnigoKey::Num5,
            Self::Num6 => EnigoKey::Num6,
            Self::Num7 => EnigoKey::Num7,
            Self::Num8 => EnigoKey::Num8,
            Self::Num9 => EnigoKey::Num9,
            Self::A => EnigoKey::A,
            Self::B => EnigoKey::B,
            Self::C => EnigoKey::C,
            Self::D => EnigoKey::D,
            Self::E => EnigoKey::E,
            Self::F => EnigoKey::F,
            Self::G => EnigoKey::G,
            Self::H => EnigoKey::H,
            Self::I => EnigoKey::I,
            Self::J => EnigoKey::J,
            Self::K => EnigoKey::K,
            Self::L => EnigoKey::L,
            Self::M => EnigoKey::M,
            Self::N => EnigoKey::N,
            Self::O => EnigoKey::O,
            Self::P => EnigoKey::P,
            Self::Q => EnigoKey::Q,
            Self::R => EnigoKey::R,
            Self::S => EnigoKey::S,
            Self::T => EnigoKey::T,
            Self::U => EnigoKey::U,
            Self::V => EnigoKey::V,
            Self::W => EnigoKey::W,
            Self::X => EnigoKey::X,
            Self::Y => EnigoKey::Y,
            Self::Z => EnigoKey::Z,
            Self::F1 => EnigoKey::F1,
            Self::F2 => EnigoKey::F2,
            Self::F3 => EnigoKey::F3,
            Self::F4 => EnigoKey::F4,
            Self::F5 => EnigoKey::F5,
            Self::F6 => EnigoKey::F6,
            Self::F7 => EnigoKey::F7,
            Self::F8 => EnigoKey::F8,
            Self::F9 => EnigoKey::F9,
            Self::F10 => EnigoKey::F10,
            Self::F11 => EnigoKey::F11,
            Self::F12 => EnigoKey::F12,
            Self::F13 => EnigoKey::F13,
            Self::F14 => EnigoKey::F14,
            Self::F15 => EnigoKey::F15,
            Self::F16 => EnigoKey::F16,
            Self::F17 => EnigoKey::F17,
            Self::F18 => EnigoKey::F18,
            Self::F19 => EnigoKey::F19,
            Self::F20 => EnigoKey::F20,
            Self::F21 => EnigoKey::F21,
            Self::F22 => EnigoKey::F22,
            Self::F23 => EnigoKey::F23,
            Self::F24 => EnigoKey::F24,
            Self::Hangul => EnigoKey::Hangul,
            Self::Hanja => EnigoKey::Hanja,
            Self::Help => EnigoKey::Help,
            Self::Home => EnigoKey::Home,
            Self::Insert => EnigoKey::Insert,
            Self::Kanji => EnigoKey::Kanji,
            Self::LControl => EnigoKey::LControl,
            Self::LeftArrow => EnigoKey::LeftArrow,
            Self::LMenu => EnigoKey::LMenu,
            Self::LShift => EnigoKey::LShift,
            Self::MediaNextTrack => EnigoKey::MediaNextTrack,
            Self::MediaPlayPause => EnigoKey::MediaPlayPause,
            Self::MediaPrevTrack => EnigoKey::MediaPrevTrack,
            Self::MediaStop => EnigoKey::MediaStop,
            Self::Meta => EnigoKey::Meta,
            Self::ModeChange => EnigoKey::ModeChange,
            Self::Multiply => EnigoKey::Multiply,
            Self::Numlock => EnigoKey::Numlock,
            Self::Numpad0 => EnigoKey::Numpad0,
            Self::Numpad1 => EnigoKey::Numpad1,
            Self::Numpad2 => EnigoKey::Numpad2,
            Self::Numpad3 => EnigoKey::Numpad3,
            Self::Numpad4 => EnigoKey::Numpad4,
            Self::Numpad5 => EnigoKey::Numpad5,
            Self::Numpad6 => EnigoKey::Numpad6,
            Self::Numpad7 => EnigoKey::Numpad7,
            Self::Numpad8 => EnigoKey::Numpad8,
            Self::Numpad9 => EnigoKey::Numpad9,
            Self::Option => EnigoKey::Option,
            Self::PageDown => EnigoKey::PageDown,
            Self::PageUp => EnigoKey::PageUp,
            Self::Pause => EnigoKey::Pause,
            Self::PrintScr => EnigoKey::PrintScr,
            Self::RControl => EnigoKey::RControl,
            Self::Enter => EnigoKey::Return,
            Self::RightArrow => EnigoKey::RightArrow,
            Self::RShift => EnigoKey::RShift,
            Self::Select => EnigoKey::Select,
            Self::Shift => EnigoKey::Shift,
            Self::Space => EnigoKey::Space,
            Self::Subtract => EnigoKey::Subtract,
            Self::Tab => EnigoKey::Tab,
            Self::UpArrow => EnigoKey::UpArrow,
            Self::VolumeDown => EnigoKey::VolumeDown,
            Self::VolumeMute => EnigoKey::VolumeMute,
            Self::VolumeUp => EnigoKey::VolumeUp,
            Self::Unicode(ch) => EnigoKey::Unicode(ch),
            Self::Number(num) => EnigoKey::Other(num),
            _ => todo!()
        }
    }
}

/// The keyboard emulator
#[derive(Debug, Clone)]
pub struct Keyboard {
    enigo: Arc<Mutex<Enigo>>,
}

impl Keyboard {
    /// Creates new keyboard emulator
    pub fn new() -> Result<Self> {
        let settings = Settings::default();
        
        Ok(Self {
            enigo: Arc::new(Mutex::new(Enigo::new(&settings)?)),
        })
    }

    /// Press a keyboard key
    pub async fn press(&self, key: &Key) -> Result<()> {
        self.enigo.lock().await.key(key.clone().into(), Direction::Click)?;

        Ok(())
    }

    /// Press a 2 keyboard keys
    pub async fn press2(&self, key1: &Key, key2: &Key) -> Result<()> {
        let key1: EnigoKey = key1.clone().into();
        let key2: EnigoKey = key2.clone().into();
        
        self.enigo.lock().await.key(key1, Direction::Press)?;
        self.enigo.lock().await.key(key1, Direction::Press)?;

        sleep(Duration::from_millis(100)).await;

        self.enigo.lock().await.key(key1, Direction::Release)?;
        self.enigo.lock().await.key(key2, Direction::Release)?;

        Ok(())
    }

    /// Press a 3 keyboard keys
    pub async fn press3(&self, key1: &Key, key2: &Key, key3: &Key) -> Result<()> {
        let key1: EnigoKey = key1.clone().into();
        let key2: EnigoKey = key2.clone().into();
        let key3: EnigoKey = key3.clone().into();
        
        self.enigo.lock().await.key(key1, Direction::Press)?;
        self.enigo.lock().await.key(key2, Direction::Press)?;
        self.enigo.lock().await.key(key3, Direction::Press)?;

        sleep(Duration::from_millis(100)).await;

        self.enigo.lock().await.key(key1, Direction::Release)?;
        self.enigo.lock().await.key(key2, Direction::Release)?;
        self.enigo.lock().await.key(key3, Direction::Release)?;

        Ok(())
    }
    
    /// Press hold a keyboard key
    pub async fn hold(&self, key: &Key) -> Result<()> {
        self.enigo.lock().await.key(key.clone().into(), Direction::Press)?;

        Ok(())
    }

    /// Release a keyboard key
    pub async fn release(&self, key: &Key) -> Result<()> {
        self.enigo.lock().await.key(key.clone().into(), Direction::Release)?;

        Ok(())
    }
}
