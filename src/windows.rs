///
/// This module contains the mouse action functions
/// for the windows opearting system
/// Uses the User32 system library
///
use crate::common::{CallbackId, MouseActions, MouseButton, MouseEvent, ScrollDirection};
use crate::error::Error;
use std::collections::HashMap;
use std::mem::size_of;
use std::os::raw::{c_int, c_long, c_short, c_uint, c_ulong, c_ushort};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::thread;

static mut HOOK: HHook = null_mut();
static mut CALLBACKS: Option<Mutex<HashMap<CallbackId, Box<dyn Fn(&MouseEvent) + Send>>>> = None;

pub struct WindowsMouseManager {
    callback_counter: CallbackId,
    is_listening: bool,
}

impl WindowsMouseManager {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn MouseActions> {
        Box::new(WindowsMouseManager {
            callback_counter: 0,
            is_listening: false,
        })
    }

    fn send_input(&self, event: WindowsMouseEvent, mouse_data: i32) -> Result<(), Error> {
        let (x, y) = self.get_position_raw()?;
        let mut input = Input {
            r#type: INPUT_MOUSE,
            mi: MouseInput {
                dx: x,
                dy: y,
                mouse_data,
                dw_flags: event as DWord,
                time: 0,
                dw_extra_info: unsafe { GetMessageExtraInfo() as *mut c_ulong },
            },
        };

        unsafe {
            let result = SendInput(1, &mut input, size_of::<Input>() as i32);
            // If the function returns 0, it means the input was blocked by another thread
            if result == 0 {
                return Err(Error::InputIsBlocked);
            }
        }
        Ok(())
    }

    fn start_listener(&mut self) -> Result<(), Error> {
        thread::spawn(move || {
            unsafe extern "system" fn low_level_mouse_handler(
                code: c_int,
                param: WParam,
                lpdata: LParam,
            ) -> LResult {
                // Construct the library's MouseEvent
                let w_param = param as u32;

                let mouse_event = match w_param {
                    WM_MOUSEMOVE => {
                        let (x, y) = get_point(lpdata);
                        Some(MouseEvent::AbsoluteMove(
                            x.try_into().expect("Can't fit i64 into i32"),
                            y.try_into().expect("Can't fit i64 into i32"),
                        ))
                    }
                    WM_LBUTTONDOWN => Some(MouseEvent::Press(MouseButton::Left)),
                    WM_MBUTTONDOWN => Some(MouseEvent::Press(MouseButton::Middle)),
                    WM_RBUTTONDOWN => Some(MouseEvent::Press(MouseButton::Right)),
                    WM_LBUTTONUP => Some(MouseEvent::Release(MouseButton::Left)),
                    WM_MBUTTONUP => Some(MouseEvent::Release(MouseButton::Middle)),
                    WM_RBUTTONUP => Some(MouseEvent::Release(MouseButton::Right)),
                    WM_MOUSEWHEEL => {
                        let delta = get_delta(lpdata) / WHEEL_DELTA as u16;
                        match delta {
                            1 => Some(MouseEvent::Scroll(ScrollDirection::Up)),
                            _ => Some(MouseEvent::Scroll(ScrollDirection::Down)),
                        }
                    }
                    WM_MOUSEHWHEEL => {
                        let delta = get_delta(lpdata) / WHEEL_DELTA as u16;
                        match delta {
                            1 => Some(MouseEvent::Scroll(ScrollDirection::Right)),
                            _ => Some(MouseEvent::Scroll(ScrollDirection::Left)),
                        }
                    }
                    _ => None,
                };

                match (mouse_event, &mut CALLBACKS) {
                    (Some(event), Some(callbacks)) => {
                        for callback in callbacks.lock().unwrap().values() {
                            callback(&event);
                        }
                    }
                    _ => {}
                }

                CallNextHookEx(HOOK, code, param, lpdata)
            }
            unsafe {
                HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(low_level_mouse_handler), null_mut(), 0);
                GetMessageA(null_mut(), null_mut(), 0, 0);
            }
        });

        Ok(())
    }

    // Return the mouse position (c_long, c_long), but it does not directly
    // comply with mouce interface, so we first fetch the positions here
    // then try to convert it to (i32, i32) within the trait implementation
    fn get_position_raw(&self) -> Result<(c_long, c_long), Error> {
        let mut out = Point { x: 0, y: 0 };
        unsafe {
            let result = GetCursorPos(&mut out);
            if result == 0 {
                return Err(Error::CustomError("failed to get the cursor position"));
            }
        }
        return Ok((out.x, out.y));
    }
}

impl Drop for WindowsMouseManager {
    fn drop(&mut self) {
        unsafe {
            if HOOK.is_null() {
                // Remove the procedure installed in the hook chain
                UnhookWindowsHookEx(HOOK);
            }
        }
    }
}

impl MouseActions for WindowsMouseManager {
    fn move_to(&self, x: usize, y: usize) -> Result<(), Error> {
        unsafe {
            let result = SetCursorPos(x as c_int, y as c_int);
            if result == 0 {
                return Err(Error::CustomError("failed to set the cursor position"));
            }
        }
        Ok(())
    }

    fn get_position(&self) -> Result<(i32, i32), Error> {
        match self.get_position_raw() {
            Ok((x, y)) => Ok((
                x.try_into().expect("Can't fit i64 into i32"),
                y.try_into().expect("Can't fit i64 into i32"),
            )),
            Err(e) => Err(e),
        }
    }

    fn press_button(&self, button: &MouseButton) -> Result<(), Error> {
        let event = match button {
            MouseButton::Left => WindowsMouseEvent::LeftDown,
            MouseButton::Middle => WindowsMouseEvent::MiddleDown,
            MouseButton::Right => WindowsMouseEvent::RightDown,
        };

        self.send_input(event, 0)
    }

    fn release_button(&self, button: &MouseButton) -> Result<(), Error> {
        let event = match button {
            MouseButton::Left => WindowsMouseEvent::LeftUp,
            MouseButton::Middle => WindowsMouseEvent::MiddleUp,
            MouseButton::Right => WindowsMouseEvent::RightUp,
        };

        self.send_input(event, 0)
    }

    fn click_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.press_button(button)?;
        self.release_button(button)
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error> {
        let (event, scroll_amount) = match direction {
            ScrollDirection::Up => (WindowsMouseEvent::Wheel, 150),
            ScrollDirection::Down => (WindowsMouseEvent::Wheel, -150),
            ScrollDirection::Right => (WindowsMouseEvent::HWheel, 150),
            ScrollDirection::Left => (WindowsMouseEvent::HWheel, -150),
        };
        self.send_input(event, scroll_amount)
    }

    fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId, Error> {
        if !self.is_listening {
            self.start_listener()?;
            self.is_listening = true;
        }

        let id = self.callback_counter;
        unsafe {
            match &mut CALLBACKS {
                Some(callbacks) => {
                    callbacks.lock().unwrap().insert(id, callback);
                }
                None => {
                    initialize_callbacks();
                    return self.hook(callback);
                }
            }
        }
        self.callback_counter += 1;
        Ok(id)
    }

    fn unhook(&mut self, callback_id: CallbackId) -> Result<(), Error> {
        unsafe {
            match &mut CALLBACKS {
                Some(callbacks) => match callbacks.lock().unwrap().remove(&callback_id) {
                    Some(_) => Ok(()),
                    None => Err(Error::UnhookFailed),
                },
                None => {
                    initialize_callbacks();
                    self.unhook(callback_id)
                }
            }
        }
    }

    fn unhook_all(&mut self) -> Result<(), Error> {
        unsafe {
            match &mut CALLBACKS {
                Some(callbacks) => {
                    callbacks.lock().unwrap().clear();
                }
                None => {
                    initialize_callbacks();
                    return self.unhook_all();
                }
            }
        }
        Ok(())
    }
}

fn initialize_callbacks() {
    unsafe {
        match CALLBACKS {
            Some(_) => {}
            None => {
                CALLBACKS = Some(Mutex::new(HashMap::new()));
            }
        }
    }
}

unsafe fn get_point(lpdata: LParam) -> (c_long, c_long) {
    let mouse = *(lpdata as *const MSLLHookStruct);
    (mouse.pt.x, mouse.pt.y)
}

unsafe fn get_delta(lpdata: LParam) -> Word {
    let mouse = *(lpdata as *const MSLLHookStruct);
    ((mouse.mouse_data >> 16) & 0xffff) as Word
}

/// User32 type definitions
type LParam = *mut c_long;
type LPInput = *mut Input;
type DWord = c_ulong;
type LResult = *mut c_int;
type WParam = usize;
type HHook = *mut Hhook__;
type HInstance = *mut HInstance__;
type HookProc =
    Option<unsafe extern "system" fn(code: c_int, w_param: WParam, l_param: LParam) -> LResult>;
type LPMsg = *mut Msg;
type HWND = *mut HWND__;
type Word = c_ushort;
const WM_MOUSEMOVE: c_uint = 0x0200;
const WM_LBUTTONDOWN: c_uint = 0x0201;
const WM_LBUTTONUP: c_uint = 0x0202;
const WM_RBUTTONDOWN: c_uint = 0x0204;
const WM_RBUTTONUP: c_uint = 0x0205;
const WM_MBUTTONDOWN: c_uint = 0x0207;
const WM_MBUTTONUP: c_uint = 0x0208;
const WM_MOUSEWHEEL: c_uint = 0x020A;
const WM_MOUSEHWHEEL: c_uint = 0x020E;
const WHEEL_DELTA: c_short = 120;
const WH_MOUSE_LL: c_int = 14;
enum Hhook__ {}
enum HInstance__ {}
enum HWND__ {}
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
#[derive(Clone, Copy)]
struct Point {
    x: c_long,
    y: c_long,
}
#[repr(C)]
enum WindowsMouseEvent {
    LeftDown = 0x0002,
    LeftUp = 0x0004,
    RightDown = 0x0008,
    RightUp = 0x0010,
    MiddleDown = 0x0020,
    MiddleUp = 0x0040,
    Wheel = 0x0800,
    HWheel = 0x01000,
}

#[repr(C)]
struct Msg {
    hwnd: HWND,
    message: c_uint,
    w_param: WParam,
    l_param: LParam,
    time: DWord,
    pt: Point,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MSLLHookStruct {
    pt: Point,
    mouse_data: DWord,
    flags: DWord,
    time: DWord,
    dw_extra_info: usize,
}

// User32 function definitions
#[link(name = "user32")]
extern "system" {
    fn SetCursorPos(x: c_int, y: c_int) -> c_int;
    fn GetCursorPos(lp_point: *mut Point) -> c_int;
    fn SendInput(c_inputs: c_uint, p_inputs: LPInput, cb_size: c_int) -> c_uint;
    fn GetMessageExtraInfo() -> LParam;
    fn SetWindowsHookExA(
        idHook: c_int,
        lpfn: HookProc,
        hmod: HInstance,
        dwThreadId: DWord,
    ) -> HHook;
    fn CallNextHookEx(hhk: HHook, n_code: c_int, w_param: WParam, l_param: LParam) -> LResult;
    fn GetMessageA(
        lp_msg: LPMsg,
        h_wnd: HWND,
        w_msg_filter_min: c_uint,
        w_msg_filter_max: c_uint,
    ) -> bool;
    fn UnhookWindowsHookEx(hhk: HHook) -> bool;
}
