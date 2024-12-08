# mouce
Mouce is a library written in Rust that aims to help simulating and listening mouse actions across different platforms.
## Supported platforms
- **Windows** ✅
  - Tested on Windows 10
  - Uses User32 system library
- **MacOS** ✅
  - Tested on a MacBook Pro (Retina, 13-inch, Mid 2014) with Big Sur installed on it
  - Uses CoreGraphics and CoreFoundation frameworks
- **Unix-like systems**
  - **X11** ✅
    - Tested on i3wm Arch Linux
    - Uses X11 and XTest libraries
  - **Others (partially supported)** ❌
    - For other systems, you can disable the x11 feature and the library will use **uinput**
      - Use `--no-default-features` argument with cargo
      - Or disable default features in `Cargo.toml`
        ```toml
        [dependencies]
        mouce = { version = "x.y.z", default-features = false }
        ```
    - While using **uinput** there are some limitations for the library
      - ```get_position``` function is not implemented as **uinput** does not provide such a feature
      - The rest of the actions work and tested on KDE Wayland and sway
## Library interface
```rust
/// Move the mouse to the given `x`, `y` coordinates
fn move_to(&self, x: usize, y: usize) -> Result<(), Error>;
/// Move the mouse relative to the current position
fn move_relative(&self, x_offset: i32, y_offset: i32) -> Result<(), Error>;
/// Get the current position of the mouse
fn get_position(&self) -> Result<(i32, i32), Error>;
/// Press down the given mouse button
fn press_button(&self, button: &MouseButton) -> Result<(), Error>;
/// Release the given mouse button
fn release_button(&self, button: &MouseButton) -> Result<(), Error>;
/// Click the given mouse button
fn click_button(&self, button: &MouseButton) -> Result<(), Error>;
/// Scroll the mouse wheel towards to the given direction
fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error>;
/// Attach a callback function to mouse events
fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId, Error>;
/// Remove the callback function with the given `CallbackId`
fn unhook(&mut self, callback_id: CallbackId) -> Result<(), Error>;
/// Remove all callback functions
fn unhook_all(&mut self) -> Result<(), Error>;
```
## Example
This example program moves the mouse from left to right;
```rust
use std::thread;
use std::time::Duration;

use mouce::{Mouse, MouseActions};

fn main() {
    let mouse_manager = Mouse::new();

    let mut x = 0;
    while x < 1920 {
        let _ = mouse_manager.move_to(x, 540);
        x += 1;
        thread::sleep(Duration::from_millis(2));
    }
}
```
To see more examples, you can look at the documentation by running;
```fish
cargo doc --open
```
## CLI binary
mouce comes with an example CLI program that uses mouce library functions.
You can install the binary with;
```fish
cargo install mouce --features="cli"
```
and see ```mouce --help``` for further details.
