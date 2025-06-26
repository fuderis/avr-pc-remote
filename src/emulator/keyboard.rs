use crate::prelude::*;
use enigo::{ Enigo, Key, Keyboard as EnigoKeyboard, Settings, Direction };

/// The keyboard emulator
#[derive(Debug, Display)]
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
    pub async fn press(&self, key: Key) -> Result<()> {
        self.enigo.lock().await.key(key, Direction::Click)?;

        Ok(())
    }

    /// Press a 2 keyboard keys
    pub async fn press2(&self, key1: Key, key2: Key) -> Result<()> {
        self.enigo.lock().await.key(key1, Direction::Press)?;
        self.enigo.lock().await.key(key2, Direction::Press)?;

        sleep(Duration::from_millis(100)).await;

        self.enigo.lock().await.key(key1, Direction::Release)?;
        self.enigo.lock().await.key(key2, Direction::Release)?;

        Ok(())
    }

    /// Press a 3 keyboard keys
    pub async fn press3(&self, key1: Key, key2: Key, key3: Key) -> Result<()> {
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
    pub async fn hold(&self, key: Key) -> Result<()> {
        self.enigo.lock().await.key(key, Direction::Press)?;

        Ok(())
    }

    /// Release a keyboard key
    pub async fn release(&self, key: Key) -> Result<()> {
        self.enigo.lock().await.key(key, Direction::Release)?;

        Ok(())
    }
}
