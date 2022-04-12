# mice
Mice is a library written in Rust that aims to help simulating mouse actions across different platforms.
## Supported platforms
- **Windows** ✅
  - Tested on Windows 10
  - Uses User32 system library
- **MacOS** ✅
  - Tested on a MacBook Pro (Retina, 13-inch, Mid 2014) with Big Sur installed on it
  - Uses CoreGraphics framework
- **Unix-like systems**
  - **X11** ✅
    - Tested on i3wm Arch Linux
    - Uses X11 and XTest libraries
  - **Others (partially supported)** ❌
    - For other systems, the library defaults to using **uinput**
    - While using **uinput** there are some limitations for the library
      - ```get_position``` function is not implemented as **uinput** does not provide such a feature
      - press and release actions for ```MouseButton::Middle``` does not work
      - The rest of the actions work and tested on KDE Wayland and sway
## Library interface
```Rust
/// Move the mouse to the given `x`, `y` coordinates
fn move_to(&self, x: usize, y: usize);
/// Get the current position of the mouse
fn get_position(&self) -> (i32, i32);
/// Press down the given mouse button
fn press_button(&self, button: &MouseButton);
/// Release the given mouse button
fn release_button(&self, button: &MouseButton);
/// Click the given mouse button
fn click_button(&self, button: &MouseButton);
/// Scroll the mouse wheel towards to the given direction
fn scroll_wheel(&self, direction: &ScrollDirection);
```
## Example
This example program moves the mouse from left to right;
```Rust
use std::thread;
use std::time::Duration;

use mice::Mouse;

fn main() {
    let mouse_manager = Mouse::new();

    let mut x = 0;
    while x < 1920 {
        mouse_manager.move_to(x, 540);
        x += 1;
        thread::sleep(Duration::from_millis(2));
    }
}
```
