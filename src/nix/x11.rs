///
/// This module contains the mouse action functions
/// for the unix-like systems that use X11
///
use crate::common::{MouseActions, MouseButton, ScrollDirection};
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

    fn button_event(&self, button: &MouseButton, is_press: bool) {
        let btn = match button {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
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

    fn press_button(&self, button: &MouseButton) {
        self.button_event(button, true);
    }

    fn release_button(&self, button: &MouseButton) {
        self.button_event(button, false);
    }

    fn click_button(&self, button: &MouseButton) {
        self.press_button(button);
        self.release_button(button);
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) {
        let btn = match direction {
            ScrollDirection::Up => 4,
            ScrollDirection::Down => 5,
        };
        unsafe {
            XTestFakeButtonEvent(self.display, btn, true, 0);
            XTestFakeButtonEvent(self.display, btn, false, 0);
            XFlush(self.display);
        }
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
