use crate::nix_x11;

use std::env::consts::OS;

pub enum MouseButton {
    LeftClick,
    MiddleClick,
    RightClick,
    ScrollUp,
    ScrollDown
}

pub fn new_mouse_manager() -> Box<dyn MouseActions> {
    match OS {
        "linux" => {
            Box::new(nix_x11::X11MouseManager::new())
        }
        _ => {
            unimplemented!();
        }
    }

}

pub trait MouseActions {
    /// Move the mouse the the given `x`, `y` coordinates
    fn move_to(&self, x: usize, y: usize);
    /// Get the current position of the mouse
    fn get_position(&self) -> (i32, i32);
    /// Press down the given mouse button
    fn press_button(&self, button: MouseButton);
    /// Release the given mouse button
    fn release_button(&self, button: MouseButton);
}
