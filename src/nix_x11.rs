///
/// This module contains the mouse action functions
/// for the unix-like systems that use X11
///
use crate::common::{MouseActions, MouseButton};
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

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

    fn button_event(&self, button: MouseButton, is_press: bool) {
        let btn = match button {
            MouseButton::LeftClick => 1,
            MouseButton::MiddleClick => 2,
            MouseButton::RightClick => 3,
            MouseButton::ScrollUp => 4,
            MouseButton::ScrollDown => 5,
        };
        unsafe {
            XTestFakeButtonEvent(self.display, btn, is_press, 0);
            XFlush(self.display);
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

    fn get_position(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut void = 0;
        let mut mask = 0;

        unsafe {
            XQueryPointer(
                self.display,
                self.window,
                &mut void,
                &mut void,
                &mut x,
                &mut y,
                &mut x,
                &mut y,
                &mut mask,
            );
        }

        return (x, y);
    }

    fn press_button(&self, button: MouseButton) {
        self.button_event(button, true);
    }

    fn release_button(&self, button: MouseButton) {
        self.button_event(button, false);
    }
}

/// Xlib type definitions
enum _XDisplay {}
type Display = _XDisplay;
type Window = c_ulong;

#[derive(Debug)]
#[repr(C)]
struct XEvent {
    r#type: c_int,
    xbutton: XButtonEvent,
}

#[derive(Debug)]
#[repr(C)]
struct XButtonEvent {
    r#type: c_int,
    window: Window,
    root: Window,
    subwindow: Window,
    x: c_int,
    y: c_int,
    x_root: c_int,
    y_root: c_int,
    state: c_uint,
    button: c_uint,
    same_screen: bool,
}

/// Xlib function definitions
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
    fn XQueryPointer(
        display: *mut Display,
        window: Window,
        root_return: *mut Window,
        child_return: *mut Window,
        root_x_return: *mut c_int,
        root_y_return: *mut c_int,
        win_x_return: *mut c_int,
        win_y_return: *mut c_int,
        mask_return: *mut c_uint,
    ) -> bool;
}

/// XTest function definitions
#[link(name = "Xtst")]
extern "C" {
    fn XTestFakeButtonEvent(
        dpy: *mut Display,
        button: c_uint,
        is_press: bool,
        delay: c_ulong,
    ) -> c_int;
}

#[cfg(test)]
mod tests {
    use crate::{common::MouseActions, common::MouseButton, nix_x11::X11MouseManager};
    use std::{thread, time};

    #[test]
    #[ignore]
    fn x11_move_to_right_bottom() {
        let manager = X11MouseManager::new();
        manager.move_to(1920, 1080);
    }

    #[test]
    #[ignore]
    fn x11_move_to_left_to_right() {
        let manager = X11MouseManager::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut x = 0;
        while x < 1920 {
            manager.move_to(x, 540);
            x += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn x11_move_to_top_to_bottom() {
        let manager = X11MouseManager::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut y = 0;
        while y < 1080 {
            manager.move_to(960, y);
            y += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn x11_get_position() {
        let manager = X11MouseManager::new();
        let positions = vec![
            (0, 0),
            (100, 100),
            (250, 250),
            (325, 325),
            (400, 100),
            (100, 400),
        ];

        let mut x;
        let mut y;
        for position in positions.iter() {
            manager.move_to(position.0, position.1);
            (x, y) = manager.get_position();
            assert_eq!(x, position.0 as i32);
            assert_eq!(y, position.1 as i32);
        }
    }

    #[test]
    #[ignore]
    fn x11_left_click() {
        let manager = X11MouseManager::new();
        manager.press_button(MouseButton::LeftClick);
        manager.release_button(MouseButton::LeftClick);
    }

    #[test]
    #[ignore]
    fn x11_scroll_down() {
        let manager = X11MouseManager::new();
        for _ in 0..10 {
            manager.press_button(MouseButton::ScrollDown);
            manager.release_button(MouseButton::ScrollDown);
            let sleep_duration = time::Duration::from_millis(250);
            thread::sleep(sleep_duration);
        }
    }
}
