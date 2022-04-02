use crate::common::MouseActions;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

pub enum _XDisplay {}
pub type Display = _XDisplay;
pub type Window = c_ulong;

pub struct X11MouseManager {
    display: *mut Display,
    window: Window,
}

impl X11MouseManager {
    pub fn new() -> Self {
        unsafe {
            let display = XOpenDisplay(&0);
            let window = XDefaultRootWindow(display);
            X11MouseManager { display, window }
        }
    }
}

impl MouseActions for X11MouseManager {
    fn move_to(&self, x: usize, y: usize) {
        unsafe {
            XWarpPointer(self.display, 0, self.window, 0, 0, 0, 0, x as i32, y as i32);
            XFlush(self.display);
        }
    }
}

#[link(name = "X11")]
extern "C" {
    fn XOpenDisplay(display: *const c_char) -> *mut Display;
    fn XDefaultRootWindow(display: *mut Display) -> Window;
    fn XWarpPointer(
        display: *mut Display,
        src_w: Window,
        dest_w: Window,
        srx_x: c_int,
        src_y: c_int,
        src_width: c_uint,
        src_height: c_uint,
        dest_x: c_int,
        dest_y: c_int,
    ) -> c_int;

        fn XFlush(display: *mut Display) -> c_int;
}

#[cfg(test)]
mod tests {
    use crate::{linux_x11::X11MouseManager, common::MouseActions};
    use std::{thread, time};

    #[test]
    fn x11_move_to_right_bottom() {
        let manager = X11MouseManager::new();
        manager.move_to(1920, 1080);
    }

    #[test]
    fn x11_move_to_left_to_right() {
        let manager = X11MouseManager::new();
        let sleep_duration = time::Duration::from_millis(5);
        let mut x = 0;
        while x < 1920 {
            manager.move_to(x, 540);
            x += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    fn x11_move_to_top_to_bottom() {
        let manager = X11MouseManager::new();
        let sleep_duration = time::Duration::from_millis(5);
        let mut y = 0;
        while y < 1080 {
            manager.move_to(960, y);
            y += 1;
            thread::sleep(sleep_duration);
        }
    }
}
