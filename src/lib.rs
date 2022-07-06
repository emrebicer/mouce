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

pub use common::MouseActions;

/// SAFETY: T must not have any padding or otherwise uninitialized bytes inside of it
pub(crate) unsafe fn cast_to_bytes<T: ?Sized>(mem: &T) -> &[u8] {
    std::slice::from_raw_parts(mem as *const T as *const u8, std::mem::size_of_val(mem))
}

#[cfg(test)]
mod tests {
    use crate::Mouse;

    #[test]
    fn supported_platform() {
        // Mouse should be visible if the current platform is supported
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        Mouse::new((0, 1920), (0, 1080));
        #[cfg(not(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        )))]
        Mouse::new();
    }
}
