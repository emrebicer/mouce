///
/// This module contains the mouse action functions
/// for the darwin systems (MacOS)
/// Uses the CoreGraphics (a.k.a Quartz) framework
///
use crate::common::{MouseActions, MouseButton, ScrollDirection};
use std::os::raw::{c_double, c_int, c_void};
use std::ptr::null_mut;

pub struct DarwinMouseManager {}

impl DarwinMouseManager {
    pub fn new() -> Self {
        DarwinMouseManager {}
    }

    fn create_mouse_event(&self, event_type: CGEventType, mouse_button: CGMouseButton) {
        let (pos_x, pos_y) = self.get_position();
        let position = CGPoint {
            x: pos_x as c_double,
            y: pos_y as c_double,
        };

        unsafe {
            let event = CGEventCreateMouseEvent(null_mut(), event_type, position, mouse_button);
            CGEventPost(CGEventTapLocation::CGHIDEventTap, event);
            CFRelease(event as CFTypeRef);
        }
    }

    fn create_scroll_wheel_event(&self, distance: c_int) {
        unsafe {
            let event =
                CGEventCreateScrollWheelEvent(null_mut(), CGScrollEventUnit::Line, 1, distance);
            CGEventPost(CGEventTapLocation::CGHIDEventTap, event);
            CFRelease(event as CFTypeRef);
        }
    }
}

impl MouseActions for DarwinMouseManager {
    fn move_to(&self, x: usize, y: usize) {
        let cg_point = CGPoint {
            x: x as f64,
            y: y as f64,
        };
        unsafe {
            CGWarpMouseCursorPosition(cg_point);
        }
    }

    fn get_position(&self) -> (i32, i32) {
        unsafe {
            let event = CGEventCreate(null_mut());
            let cursor = CGEventGetLocation(event);
            CFRelease(event as CFTypeRef);
            return (cursor.x as i32, cursor.y as i32);
        }
    }

    fn press_button(&self, button: &MouseButton) {
        let (event_type, mouse_button) = match button {
            MouseButton::Left => (CGEventType::LeftMouseDown, CGMouseButton::Left),
            MouseButton::Middle => (CGEventType::OtherMouseDown, CGMouseButton::Center),
            MouseButton::Right => (CGEventType::RightMouseDown, CGMouseButton::Right),
        };
        self.create_mouse_event(event_type, mouse_button);
    }

    fn release_button(&self, button: &MouseButton) {
        let (event_type, mouse_button) = match button {
            MouseButton::Left => (CGEventType::LeftMouseUp, CGMouseButton::Left),
            MouseButton::Middle => (CGEventType::OtherMouseUp, CGMouseButton::Center),
            MouseButton::Right => (CGEventType::RightMouseUp, CGMouseButton::Right),
        };
        self.create_mouse_event(event_type, mouse_button);
    }

    fn click_button(&self, button: &MouseButton) {
        self.press_button(button);
        self.release_button(button);
    }

    fn scroll_wheel(&self, direction: &ScrollDirection) {
        let distance = match direction {
            ScrollDirection::Up => 5,
            ScrollDirection::Down => -5,
        };
        self.create_scroll_wheel_event(distance);
    }
}

/// CoreGraphics type definitions
enum CGError {}
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
#[repr(C)]
enum CGEventType {
    LeftMouseDown = 1,
    LeftMouseUp = 2,
    RightMouseDown = 3,
    RightMouseUp = 4,
    _LeftMouseDragged = 6,
    _RightMouseDragged = 7,
    _ScrollWheel = 22,
    OtherMouseDown = 25,
    OtherMouseUp = 26,
    _OtherMouseDragged = 27,
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
        wheel_count: c_int,
        wheel1: c_int,
    ) -> CGEventRef;
    fn CGEventPost(tap: CGEventTapLocation, event: CGEventRef);
}
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFRelease(cf: CFTypeRef);
}
