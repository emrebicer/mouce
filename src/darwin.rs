///
/// This module contains the mouse action functions
/// for the darwin systems (MacOS)
/// Uses the CoreGraphics (a.k.a Quartz) framework
///
use crate::common::{CallbackId, MouseActions, MouseButton, MouseEvent, ScrollDirection};
use crate::error::Error;
use std::collections::HashMap;
use std::os::raw::{c_double, c_int, c_long, c_uint, c_ulong, c_void};
use std::ptr::null_mut;
use std::sync::Mutex;
use std::thread;

static mut TAP_EVENT_REF: Option<CFTypeRef> = None;
static mut CALLBACKS: Option<Mutex<HashMap<CallbackId, Box<dyn Fn(&MouseEvent) + Send>>>> = None;

pub struct DarwinMouseManager {
    callback_counter: CallbackId,
    is_listening: bool,
}

impl DarwinMouseManager {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Box<dyn MouseActions> {
        Box::new(DarwinMouseManager {
            callback_counter: 0,
            is_listening: false,
        })
    }

    fn create_mouse_event(
        &self,
        event_type: CGEventType,
        mouse_button: CGMouseButton,
    ) -> Result<(), Error> {
        let (pos_x, pos_y) = self.get_position()?;
        let position = CGPoint {
            x: pos_x as c_double,
            y: pos_y as c_double,
        };

        unsafe {
            let event = CGEventCreateMouseEvent(null_mut(), event_type, position, mouse_button);
            if event == null_mut() {
                return Err(Error::CGCouldNotCreateEvent);
            }
            CGEventPost(CGEventTapLocation::CGHIDEventTap, event);
            CFRelease(event as CFTypeRef);
        }

        Ok(())
    }

    fn create_scroll_wheel_event(
        &self,
        distance: c_int,
        direction: &ScrollDirection,
    ) -> Result<(), Error> {
        unsafe {
            let event = match direction {
                ScrollDirection::Up | ScrollDirection::Down => CGEventCreateScrollWheelEvent(
                    null_mut(),
                    CGScrollEventUnit::Line,
                    2,
                    distance,
                    0,
                ),
                ScrollDirection::Right | ScrollDirection::Left => CGEventCreateScrollWheelEvent(
                    null_mut(),
                    CGScrollEventUnit::Line,
                    2,
                    0,
                    distance,
                ),
            };

            if event == null_mut() {
                return Err(Error::CGCouldNotCreateEvent);
            }
            CGEventPost(CGEventTapLocation::CGHIDEventTap, event);
            CFRelease(event as CFTypeRef);
        }
        Ok(())
    }

    fn start_listener(&mut self) -> Result<(), Error> {
        thread::spawn(move || {
            unsafe extern "C" fn mouse_on_event_callback(
                _proxy: *const c_void,
                event_type: CGEventType,
                cg_event: CGEventRef,
                _user_info: *mut c_void,
            ) -> CGEventRef {
                // Construct the library's MouseEvent
                let mouse_event = match event_type {
                    CGEventType::LeftMouseDown => {
                        let point = CGEventGetLocation(cg_event);
                        Some(MouseEvent::Press(MouseButton::Left, point.x as i32, point.y as i32))
                    }
                    CGEventType::LeftMouseUp => Some(MouseEvent::Release(MouseButton::Left)),
                    CGEventType::RightMouseDown => {
                        let point = CGEventGetLocation(cg_event);
                        Some(MouseEvent::Press(MouseButton::Right, point.x as i32, point.y as i32))
                    }
                    CGEventType::RightMouseUp => Some(MouseEvent::Release(MouseButton::Right)),
                    CGEventType::OtherMouseDown => {
                        let point = CGEventGetLocation(cg_event);
                        Some(MouseEvent::Press(MouseButton::Middle, point.x as i32, point.y as i32))
                    }
                    CGEventType::OtherMouseUp => Some(MouseEvent::Release(MouseButton::Middle)),
                    CGEventType::MouseMoved => {
                        let point = CGEventGetLocation(cg_event);
                        Some(MouseEvent::AbsoluteMove(point.x as i32, point.y as i32))
                    }
                    CGEventType::ScrollWheel => {
                        // CGEventField::scrollWheelEventPointDeltaAxis1 = 96
                        // CGEventField::scrollWheelEventPointDeltaAxis2 = 97
                        let delta_y = CGEventGetIntegerValueField(cg_event, 96);
                        let delta_x = CGEventGetIntegerValueField(cg_event, 97);
                        if delta_y > 0 {
                            Some(MouseEvent::Scroll(ScrollDirection::Up))
                        } else if delta_y < 0 {
                            Some(MouseEvent::Scroll(ScrollDirection::Down))
                        } else if delta_x < 0 {
                            Some(MouseEvent::Scroll(ScrollDirection::Right))
                        } else if delta_x > 0 {
                            Some(MouseEvent::Scroll(ScrollDirection::Left))
                        } else {
                            // Probably axis3 wheel scrolled
                            None
                        }
                    }
                    CGEventType::KeyDown => Some(MouseEvent::KeyDown()),
                    CGEventType::KeyUp => Some(MouseEvent::KeyUp()),
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

                cg_event
            }

            unsafe {
                // Create the mouse listener hook
                TAP_EVENT_REF = Some(CGEventTapCreate(
                    CGEventTapLocation::CGHIDEventTap,
                    CGEventTapPlacement::HeadInsertEventTap,
                    CGEventTapOption::ListenOnly as u32,
                    (1 << CGEventType::LeftMouseDown as u64)
                        + (1 << CGEventType::LeftMouseUp as u64)
                        + (1 << CGEventType::RightMouseDown as u64)
                        + (1 << CGEventType::RightMouseUp as u64)
                        + (1 << CGEventType::OtherMouseDown as u64)
                        + (1 << CGEventType::OtherMouseUp as u64)
                        + (1 << CGEventType::MouseMoved as u64)
                        + (1 << CGEventType::ScrollWheel as u64)
                        + (1 << CGEventType::KeyDown as u64)
                        + (1 << CGEventType::KeyUp as u64),
                    Some(mouse_on_event_callback),
                    null_mut(),
                ));

                let loop_source =
                    CFMachPortCreateRunLoopSource(null_mut(), TAP_EVENT_REF.unwrap(), 0);
                let current_loop = CFRunLoopGetCurrent();
                CFRunLoopAddSource(current_loop, loop_source, kCFRunLoopDefaultMode);
                CGEventTapEnable(TAP_EVENT_REF.unwrap(), true);
                CFRunLoopRun();
            }
        });

        Ok(())
    }
}

impl Drop for DarwinMouseManager {
    fn drop(&mut self) {
        unsafe {
            match TAP_EVENT_REF {
                Some(event_ref) => {
                    // Release the tap event
                    CFRelease(event_ref);
                    TAP_EVENT_REF = None;
                }
                None => {}
            }
        }
    }
}

impl MouseActions for DarwinMouseManager {
    fn move_to(&self, x: usize, y: usize) -> Result<(), Error> {
        let cg_point = CGPoint {
            x: x as f64,
            y: y as f64,
        };
        unsafe {
            let result = CGWarpMouseCursorPosition(cg_point);
            if result != CGError::Success {
                return Err(Error::CustomError(
                    "Failed to move the mouse, CGError is not Success",
                ));
            }
        };

        Ok(())
    }

    fn get_position(&self) -> Result<(i32, i32), Error> {
        unsafe {
            let event = CGEventCreate(null_mut());
            if event == null_mut() {
                return Err(Error::CGCouldNotCreateEvent);
            }
            let cursor = CGEventGetLocation(event);
            CFRelease(event as CFTypeRef);
            return Ok((cursor.x as i32, cursor.y as i32));
        }
    }

    fn press_button(&self, button: &MouseButton) -> Result<(), Error> {
        let (event_type, mouse_button) = match button {
            MouseButton::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
            MouseButton::Middle => (CGEventType::OtherMouseDown, CGMouseButton::Center),
            MouseButton::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        };
        self.create_mouse_event(event_type, mouse_button)?;
        Ok(())
    }

    fn release_button(&self, button: &MouseButton) -> Result<(), Error> {
        let (event_type, mouse_button) = match button {
            MouseButton::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
            MouseButton::Middle => (CGEventType::OtherMouseUp, CGMouseButton::Center),
            MouseButton::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        };
        self.create_mouse_event(event_type, mouse_button)
    }

    fn click_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.press_button(button)?;
        self.release_button(button)
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error> {
        let distance = match direction {
            ScrollDirection::Up | ScrollDirection::Left => 5,
            ScrollDirection::Down | ScrollDirection::Right => -5,
        };
        self.create_scroll_wheel_event(distance, direction)
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

/// CoreGraphics type definitions
#[allow(dead_code)]
#[derive(PartialEq, Eq)]
#[repr(C)]
enum CGError {
    CannotComplete = 1004,
    Failure = 1000,
    IllegalArgument = 1001,
    InvalidConnection = 1002,
    InvalidContext = 1003,
    InvalidOperation = 1010,
    NoneAvailable = 1011,
    NotImplemented = 1006,
    RangeCheck = 1007,
    Success = 0,
    TypeCheck = 1008,
}
#[repr(C)]
pub struct CGPoint {
    x: c_double,
    y: c_double,
}
enum CGEventSource {}
enum CGEvent {}
type CGEventSourceRef = *mut CGEventSource;
type CGEventRef = *mut CGEvent;
type CFTypeRef = *const c_void;
type CGEventMask = c_ulong;

#[repr(C)]
enum CGEventType {
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    MouseMoved = 5,
    _LeftMouseDragged = 6,
    _RightMouseDragged = 7,
    ScrollWheel = 22,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    _OtherMouseDragged = 27,
    KeyDown = 10,
    KeyUp = 11,
}

#[repr(C)]
enum CGMouseButton {
    Left = 0,
    Right = 1,
    Center = 2,
}

#[repr(C)]
enum CGEventTapLocation {
    CGHIDEventTap = 0,
    _CGSessionEventTap = 1,
    _CGAnnotatedSessionEventTap = 2,
}

#[repr(C)]
enum CGScrollEventUnit {
    _Pixel = 0,
    Line = 1,
}

#[repr(C)]
enum CGEventTapPlacement {
    HeadInsertEventTap = 0,
    _TailAppendEventTap = 1,
}

#[repr(C)]
enum CGEventTapOption {
    _Default = 0,
    ListenOnly = 1,
}

type CGEventTapCallback = Option<
    unsafe extern "C" fn(
        proxy: *const c_void,
        event_type: CGEventType,
        cg_event: CGEventRef,
        user_info: *mut c_void,
    ) -> CGEventRef,
>;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> CGError;
    fn CGEventCreate(source: CGEventSourceRef) -> CGEventRef;
    fn CGEventGetLocation(event: CGEventRef) -> CGPoint;
    fn CGEventCreateMouseEvent(
        source: CGEventSourceRef,
        mouse_type: CGEventType,
        mouse_cursor_position: CGPoint,
        mouse_button: CGMouseButton,
    ) -> CGEventRef;
    fn CGEventCreateScrollWheelEvent(
        source: CGEventSourceRef,
        units: CGScrollEventUnit,
        // Number of scroll directions/wheels, maximum is 3
        wheel_count: c_int,
        // Vertical wheel movement distance
        wheel1: c_int,
        // Horizontal wheel movement distance
        wheel2: c_int,
    ) -> CGEventRef;
    fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
    fn CGEventTapCreate(
        tap: CGEventTapLocation,
        place: CGEventTapPlacement,
        options: c_uint,
        eventsOfInterest: CGEventMask,
        callback: CGEventTapCallback,
        refcon: *mut c_void,
    ) -> CFTypeRef;
    fn CGEventTapEnable(tap: *const c_void, enable: bool);
    fn CGEventGetIntegerValueField(event: CGEventRef, field: c_uint) -> c_long;
}
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFRunLoopDefaultMode: *const c_void;

    fn CFRelease(cf: CFTypeRef);
    fn CFMachPortCreateRunLoopSource(
        allocator: *mut c_void,
        tap: *const c_void,
        order: c_ulong,
    ) -> *mut c_void;
    fn CFRunLoopGetCurrent() -> *mut c_void;
    fn CFRunLoopAddSource(rl: *mut c_void, source: *mut c_void, mode: *const c_void);
    fn CFRunLoopRun();
}
