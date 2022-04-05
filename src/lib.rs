#[cfg(target_os = "linux")]
pub mod nix_x11;
#[cfg(target_os = "linux")]
pub use crate::nix_x11::X11MouseManager as Mouse;

pub mod common;

#[cfg(test)]
mod tests {
    use crate::{Mouse, common::MouseActions};

    #[test]
    fn new_mouse_manager_test() {
        let manager = Mouse::new();
        manager.move_to(1920, 1080);
    }
}
