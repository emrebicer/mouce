#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub mod nix;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub use crate::nix::NixMouseManager as Mouse;

#[cfg(target_vendor = "apple")]
pub mod darwin;
#[cfg(target_vendor = "apple")]
pub use crate::darwin::DarwinMouseManager as Mouse;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use crate::windows::WindowsMouseManager as Mouse;

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
