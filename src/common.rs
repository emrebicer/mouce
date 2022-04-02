use crate::nix_x11;

use std::env::consts::OS;

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
}
