#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub mod nix;

#[cfg(target_vendor = "apple")]
pub mod darwin;

#[cfg(target_os = "windows")]
pub mod windows;


/// The `Mouse` struct that implements the `MouseActions`
///
/// # Example usage
///
/// ```rust,no_run
/// use std::thread;
/// use std::time::Duration;
/// 
/// use mouce::{Mouse, MouseActions};
/// 
/// fn main() {
///     let mouse_manager = Mouse::new();
/// 
///     let mut x = 0;
///     while x < 1920 {
///         let _ = mouse_manager.move_to(x, 540);
///         x += 1;
///         thread::sleep(Duration::from_millis(2));
///     }
/// }
/// ```
#[derive(Clone)]
pub struct Mouse {
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ),
        feature = "x11"
    ))]
    inner: crate::nix::x11::X11MouseManager,
    #[cfg(all(
        any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ),
        not(feature = "x11")
    ))]
    inner: crate::nix::uinput::UInputMouseManager,
    #[cfg(target_vendor = "apple")]
    inner: crate::darwin::DarwinMouseManager,
    #[cfg(target_os = "windows")]
    inner: crate::windows::WindowsMouseManager,
}

impl Mouse {
    pub fn new() -> Self {
        #[cfg(all(
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ),
            feature = "x11"
        ))]
        let inner = crate::nix::x11::X11MouseManager::new();
        #[cfg(all(
            any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ),
            not(feature = "x11")
        ))]
        let inner = crate::nix::uinput::UInputMouseManager::new();
        #[cfg(target_vendor = "apple")]
        let inner = crate::darwin::DarwinMouseManager::new();
        #[cfg(target_os = "windows")]
        let inner = crate::windows::WindowsMouseManager::new();

        Self { inner }
    }
}

impl Default for Mouse {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseActions for Mouse {
    fn move_to(&self, x: i32, y: i32) -> Result<(), error::Error> {
        self.inner.move_to(x, y)
    }

    fn move_relative(&self, x_offset: i32, y_offset: i32) -> Result<(), error::Error> {
        self.inner.move_relative(x_offset, y_offset)
    }

    fn get_position(&self) -> Result<(i32, i32), error::Error> {
        self.inner.get_position()
    }

    fn press_button(&self, button: &common::MouseButton) -> Result<(), error::Error> {
        self.inner.press_button(button)
    }

    fn release_button(&self, button: &common::MouseButton) -> Result<(), error::Error> {
        self.inner.release_button(button)
    }

    fn click_button(&self, button: &common::MouseButton) -> Result<(), error::Error> {
        self.inner.click_button(button)
    }

    fn scroll_wheel(&self, direction: &common::ScrollDirection) -> Result<(), error::Error> {
        self.inner.scroll_wheel(direction)
    }

    fn hook(
        &mut self,
        callback: Box<dyn Fn(&common::MouseEvent) + Send>,
    ) -> Result<common::CallbackId, error::Error> {
        self.inner.hook(callback)
    }

    fn unhook(&mut self, callback_id: common::CallbackId) -> Result<(), error::Error> {
        self.inner.unhook(callback_id)
    }

    fn unhook_all(&mut self) -> Result<(), error::Error> {
        self.inner.unhook_all()
    }
}

pub mod common;
pub mod error;

pub use common::MouseActions;

#[cfg(test)]
mod tests {
    use crate::Mouse;

    #[test]
    fn supported_platform() {
        // Mouse should be visible and successfully return a new instance
        // if the current platform is supported
        Mouse::new();
    }
}
