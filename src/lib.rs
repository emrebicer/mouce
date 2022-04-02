pub mod linux_x11;
pub mod common;
pub use common::new_mouse_manager;

#[cfg(test)]
mod tests {
    use crate::new_mouse_manager;
    #[test]
    fn new_mouse_manager_test() {
        let manager = new_mouse_manager();
        manager.move_to(1920, 1080);
    }
}
