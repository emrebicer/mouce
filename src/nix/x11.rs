///
/// This module contains the mouse action functions
/// for the unix-like systems that use X11
///
use crate::common::{CallbackId, MouseActions, MouseButton, MouseEvent, ScrollDirection};
use crate::error::Error;
use crate::nix::Callbacks;
use std::collections::HashMap;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::sync::{Arc, Mutex};

pub struct X11MouseManager {
    display: *mut Display,
    window: Window,
    callbacks: Callbacks,
    callback_counter: CallbackId,
    is_listening: bool,
}

impl X11MouseManager {
    pub fn new() -> Self {
        unsafe {
            let display = XOpenDisplay(&0);
            let window = XDefaultRootWindow(display);
            X11MouseManager {
                display,
                window,
                callbacks: Arc::new(Mutex::new(HashMap::new())),
                callback_counter: 0,
                is_listening: false,
            }
        }
    }

    fn button_event(&self, button: &MouseButton, is_press: bool) -> Result<(), Error> {
        let btn = match button {
            MouseButton::Left => 1,
            MouseButton::Middle => 2,
            MouseButton::Right => 3,
        };
        unsafe {
            XTestFakeButtonEvent(self.display, btn, is_press, 0);
            XFlush(self.display);
        }
        Ok(())
    }
}

impl MouseActions for X11MouseManager {
    fn move_to(&self, x: usize, y: usize) -> Result<(), Error> {
        unsafe {
            XWarpPointer(self.display, 0, self.window, 0, 0, 0, 0, x as i32, y as i32);
            XFlush(self.display);
        }
        Ok(())
    }

    fn get_position(&self) -> Result<(i32, i32), Error> {
        let mut x = 0;
        let mut y = 0;
        let mut void = 0;
        let mut mask = 0;

        unsafe {
            let out = XQueryPointer(
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

            // If XQueryPointer returns False (which is an enum value that corresponds to 0)
            // that means the pointer is not on the same screen as the specified window
            if out == 0 {
                return Err(Error::X11PointerWindowMismatch);
            }
        }

        Ok((x, y))
    }

    fn press_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.button_event(button, true)
    }

    fn release_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.button_event(button, false)
    }

    fn click_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.press_button(button)?;
        self.release_button(button)
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error> {
        let btn = match direction {
            ScrollDirection::Up => 4,
            ScrollDirection::Down => 5,
            ScrollDirection::Left => 6,
            ScrollDirection::Right => 7,
        };
        unsafe {
            XTestFakeButtonEvent(self.display, btn, true, 0);
            XTestFakeButtonEvent(self.display, btn, false, 0);
            XFlush(self.display);
        }
        Ok(())
    }

    fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId, Error> {
        if !self.is_listening {
            super::start_nix_listener(&self.callbacks)?;
            self.is_listening = true;
        }

        let id = self.callback_counter;
        self.callbacks.lock().unwrap().insert(id, callback);
        self.callback_counter += 1;
        Ok(id)
    }

    fn unhook(&mut self, callback_id: CallbackId) -> Result<(), Error> {
        match self.callbacks.lock().unwrap().remove(&callback_id) {
            Some(_) => Ok(()),
            None => Err(Error::UnhookFailed),
        }
    }

    fn unhook_all(&mut self) -> Result<(), Error> {
        self.callbacks.lock().unwrap().clear();
        Ok(())
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

// Xlib function definitions
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
    ) -> c_int;
}

// XTest function definitions
#[link(name = "Xtst")]
extern "C" {
    fn XTestFakeButtonEvent(
        dpy: *mut Display,
        button: c_uint,
        is_press: bool,
        delay: c_ulong,
    ) -> c_int;
}
