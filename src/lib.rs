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
    use crate::Mouse;

    #[test]
    fn supported_platform() {
        // Mouse should be visible if the current platform is supported
        Mouse::new();
    }
}
