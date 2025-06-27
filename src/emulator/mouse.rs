use crate::prelude::*;
use std::sync::{Arc, Mutex};
use enigo::{ Axis, Button, Coordinate, Direction, Enigo, Mouse as EnigoMouse, Settings, InputError, };

#[derive(Debug, Clone)]
pub struct Mouse {
    enigo: Arc<Mutex<Enigo>>,
}

impl Mouse {
    /// Creates a new mouse emulator
    pub fn new() -> Result<Self> {
        let settings = Settings::default();
        let enigo = Enigo::new(&settings).unwrap();
        Ok(Self {
            enigo: Arc::new(Mutex::new(enigo)),
        })
    }

    /// Returns current mouse coordinates
    pub fn get_coords(&self) -> Result<(i32, i32)> {
        let enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.location().map_err(From::from)
    }

    /// Returns screen resolution (width, height)
    pub fn get_display_size(&self) -> Result<(i32, i32)> {
        let enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.main_display().map_err(From::from)
    }

    /// Move mouse horizontally (relative)
    pub fn move_x(&self, dx: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.move_mouse(dx, 0, Coordinate::Rel).map_err(From::from)
    }

    /// Move mouse vertically (relative)
    pub fn move_y(&self, dy: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.move_mouse(0, dy, Coordinate::Rel).map_err(From::from)
    }

    /// Move mouse to center
    pub fn move_center(&self) -> Result<()> {
        let (width, height) = self.get_display_size()?;
        let center_x = width / 2;
        let center_y = height / 2;
        
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.move_mouse(center_x, center_y, Coordinate::Abs).map_err(From::from)
    }

    /// Press left mouse button
    pub fn press_left(&self, hold: bool) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;

        enigo.button(Button::Left, Direction::Press)?;

        if !hold {
            enigo.button(Button::Left, Direction::Release)?;
        }

        Ok(())
    }

    /// Release left mouse button
    pub fn release_left(&self) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.button(Button::Left, Direction::Release).map_err(From::from)
    }

    /// Press right mouse button
    pub fn press_right(&self, hold: bool) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;

        enigo.button(Button::Right, Direction::Press)?;

        if !hold {
            enigo.button(Button::Right, Direction::Release)?;
        }

        Ok(())
    }

    /// Release right mouse button
    pub fn release_right(&self) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.button(Button::Right, Direction::Release).map_err(From::from)
    }

    /// Scroll horizontally
    pub fn scroll_x(&self, delta: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.scroll(delta, Axis::Horizontal).map_err(From::from)
    }

    /// Scroll vertically
    pub fn scroll_y(&self, delta: i32) -> Result<()> {
        let mut enigo = self.enigo.lock().map_err(|_| InputError::Simulate("Mutex poisoned"))?;
        enigo.scroll(delta, Axis::Vertical).map_err(From::from)
    }
}
