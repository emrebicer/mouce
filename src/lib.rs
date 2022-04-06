#[cfg(target_os = "linux")]
pub mod nix_x11;
#[cfg(target_os = "linux")]
pub use crate::nix_x11::X11MouseManager as Mouse;
#[cfg(target_vendor = "apple")]
pub mod darwin;
#[cfg(target_vendor = "apple")]
pub use crate::darwin::DarwinMouseManager as Mouse;

pub mod common;

#[cfg(test)]
mod tests {
    use crate::{common::MouseActions, Mouse};

    #[test]
    fn new_mouse_manager_test() {
        let manager = Mouse::new();
        manager.move_to(1920, 1080);
    }
}
