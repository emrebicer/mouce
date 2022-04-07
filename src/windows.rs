///
/// This module contains the mouse action functions
/// for the windows opearting system
/// Uses the User32 system library
///
use crate::common::{MouseActions, MouseButton, ScrollDirection};
use std::mem::size_of;
use std::os::raw::{c_int, c_long, c_uint, c_ulong};

pub struct WindowsMouseManager {}

impl WindowsMouseManager {
    pub fn new() -> Self {
        WindowsMouseManager {}
    }

    fn send_input(&self, event: MouseEvent, mouse_data: i32) {
        let (x, y) = self.get_position();
        let mut input = Input {
            r#type: INPUT_MOUSE,
            mi: MouseInput {
                dx: x,
                dy: y,
                mouse_data,
                dw_flags: event as u32,
                time: 0,
                dw_extra_info: unsafe { GetMessageExtraInfo() as *mut c_ulong },
            },
        };

        unsafe {
            SendInput(1, &mut input, size_of::<Input>() as i32);
        }
    }
}

impl MouseActions for WindowsMouseManager {
    fn move_to(&self, x: usize, y: usize) {
        unsafe {
            SetCursorPos(x as c_int, y as c_int);
        }
    }

    fn get_position(&self) -> (i32, i32) {
        let mut out = Point { x: 0, y: 0 };
        unsafe {
            GetCursorPos(&mut out);
        }
        return (out.x, out.y);
    }

    fn press_button(&self, button: &MouseButton) {
        let event = match button {
            MouseButton::Left => MouseEvent::MouseEventFLeftDown,
            MouseButton::Middle => MouseEvent::MouseEventFMiddleDown,
            MouseButton::Right => MouseEvent::MouseEventFRightDown,
        };

        self.send_input(event, 0);
    }

    fn release_button(&self, button: &MouseButton) {
        let event = match button {
            MouseButton::Left => MouseEvent::MouseEventFLeftUp,
            MouseButton::Middle => MouseEvent::MouseEventFMiddleUp,
            MouseButton::Right => MouseEvent::MouseEventFRightUp,
        };

        self.send_input(event, 0);
    }

    fn click_button(&self, button: &MouseButton) {
        self.press_button(button);
        self.release_button(button);
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) {
        let scroll_amount = match direction {
            ScrollDirection::Up => 150,
            ScrollDirection::Down => -150,
        };
        self.send_input(MouseEvent::MouseEventFWheel, scroll_amount);
    }
}

/// User32 type definitions
type LParam = *mut c_long;
type LPInput = *mut Input;
type DWord = c_ulong;
const INPUT_MOUSE: DWord = 0;
#[repr(C)]
struct MouseInput {
    dx: c_long,
    dy: c_long,
    mouse_data: c_int,
    dw_flags: DWord,
    time: DWord,
    dw_extra_info: *mut c_ulong,
}
#[repr(C)]
struct Input {
    r#type: DWord,
    mi: MouseInput,
}
#[repr(C)]
struct Point {
    x: c_long,
    y: c_long,
}
#[repr(C)]
enum MouseEvent {
    MouseEventFLeftDown = 0x0002,
    MouseEventFLeftUp = 0x0004,
    MouseEventFRightDown = 0x0008,
    MouseEventFRightUp = 0x0010,
    MouseEventFMiddleDown = 0x0020,
    MouseEventFMiddleUp = 0x0040,
    MouseEventFWheel = 0x0800,
}

/// User32 function definitions
#[link(name = "user32")]
extern "system" {
    fn SetCursorPos(x: c_int, y: c_int) -> c_int;
    fn GetCursorPos(lppoint: *mut Point) -> bool;
    fn SendInput(cInputs: c_uint, pInputs: LPInput, cbSize: c_int) -> c_uint;
    fn GetMessageExtraInfo() -> LParam;
}