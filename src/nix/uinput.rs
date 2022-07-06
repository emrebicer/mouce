///
/// This module contains the mouse action functions
/// for the li&nux systems that uses uinput
///
/// - Unsupported mouse actions
///     - get_position is not available on uinput
///
use crate::common::{CallbackId, MouseActions, MouseButton, MouseEvent, ScrollDirection};
use std::{
    collections::HashMap,
    ffi::CString,
    fs::File,
    io::{Error, ErrorKind, Result, Write},
    os::{
        raw::{c_int, c_uint, c_ulong, c_ushort},
        unix::prelude::AsRawFd,
    },
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct UInputMouseManager {
    uinput_file: File,
    callbacks: Arc<Mutex<HashMap<CallbackId, Box<dyn Fn(&MouseEvent) + Send>>>>,
    callback_counter: CallbackId,
    is_listening: bool,
}

impl UInputMouseManager {
    pub fn new(rng_x: (i32, i32), rng_y: (i32, i32)) -> Self {
        let manager = UInputMouseManager {
            uinput_file: File::options()
                .write(true)
                .open("/dev/uinput")
                .expect("uinput file can not be opened"),
            callbacks: Arc::new(Mutex::new(HashMap::new())),
            callback_counter: 0,
            is_listening: false,
        };
        let fd = manager.uinput_file.as_raw_fd();
        unsafe {
            // For press events (also needed for mouse movement)
            ioctl(fd, UI_SET_EVBIT, EV_KEY);
            ioctl(fd, UI_SET_KEYBIT, BTN_LEFT);
            ioctl(fd, UI_SET_KEYBIT, BTN_RIGHT);
            ioctl(fd, UI_SET_KEYBIT, BTN_MIDDLE);

            // For mouse movement
            ioctl(fd, UI_SET_EVBIT, EV_ABS);
            ioctl(fd, UI_SET_ABSBIT, ABS_X);
            ioctl(
                fd,
                UI_ABS_SETUP,
                &UinputAbsSetup {
                    code: ABS_X as _,
                    absinfo: InputAbsinfo {
                        value: 0,
                        minimum: rng_x.0,
                        maximum: rng_x.1,
                        fuzz: 0,
                        flat: 0,
                        resolution: 0,
                    },
                },
            );
            ioctl(fd, UI_SET_ABSBIT, ABS_Y);
            ioctl(
                fd,
                UI_ABS_SETUP,
                &UinputAbsSetup {
                    code: ABS_Y as _,
                    absinfo: InputAbsinfo {
                        value: 0,
                        minimum: rng_y.0,
                        maximum: rng_y.1,
                        fuzz: 0,
                        flat: 0,
                        resolution: 0,
                    },
                },
            );

            ioctl(fd, UI_SET_EVBIT, EV_REL);
            ioctl(fd, UI_SET_RELBIT, REL_X);
            ioctl(fd, UI_SET_RELBIT, REL_Y);
            ioctl(fd, UI_SET_RELBIT, REL_WHEEL);
        }

        let usetup = UInputSetup {
            id: InputId {
                bustype: BUS_USB,
                // Random vendor and product
                vendor: 0x2222,
                product: 0x3333,
                version: 0,
            },
            name: CString::new("mouce-library-fake-mouse").unwrap(),
            ff_effects_max: 0,
        };

        unsafe {
            ioctl(fd, UI_DEV_SETUP, &usetup);
            ioctl(fd, UI_DEV_CREATE);
        }

        // On UI_DEV_CREATE the kernel will create the device node for this
        // device. We are inserting a pause here so that userspace has time
        // to detect, initialize the new device, and can start listening to
        // the event, otherwise it will not notice the event we are about to send.
        thread::sleep(Duration::from_millis(300));

        manager
    }

    #[inline]
    fn write_raw(&mut self, messages: &[InputEvent]) -> Result<()> {
        let bytes = unsafe { crate::cast_to_bytes(messages) };
        self.uinput_file.write_all(bytes)
    }

    /// Write the given event to the uinput file
    fn emit(&mut self, r#type: c_int, code: c_int, value: c_int) -> Result<()> {
        let event = InputEvent {
            time: TimeVal {
                tv_sec: 0,
                tv_usec: 0,
            },
            r#type: r#type as u16,
            code: code as u16,
            value,
        };
        self.write_raw(&[event])
    }

    /// Syncronize the device
    fn syncronize(&mut self) -> Result<()> {
        self.emit(EV_SYN, SYN_REPORT, 0)?;
        // Give uinput some time to update the mouse location,
        // otherwise it fails to move the mouse on release mode
        // A delay of 1 milliseconds seems to be enough for it
        thread::sleep(Duration::from_millis(1));
        Ok(())
    }

    /// Move the mouse relative to the current position
    fn move_relative(&mut self, x: i32, y: i32) -> Result<()> {
        // uinput does not move the mouse in pixels but uses `units`. I couldn't
        // find information regarding to this uinput `unit`, but according to
        // my findings 1 unit corresponds to exactly 2 pixels.
        //
        // To achieve the expected behavior; divide the parameters by 2
        //
        // This seems like there is a bug in this crate, but the
        // behavior is the same on other projects that make use of
        // uinput. e.g. `ydotool`. When you try to move your mouse,
        // it will move 2x further pixels
        self.emit(EV_REL, REL_X as i32, (x as f32 / 2.).ceil() as i32)?;
        self.emit(EV_REL, REL_Y as i32, (y as f32 / 2.).ceil() as i32)?;
        self.syncronize()
    }
}

impl Drop for UInputMouseManager {
    fn drop(&mut self) {
        let fd = self.uinput_file.as_raw_fd();
        unsafe {
            // Destroy the device, the file is closed automatically by the File module
            ioctl(fd, UI_DEV_DESTROY as u64);
        }
    }
}

impl MouseActions for UInputMouseManager {
    fn move_to(&mut self, x: usize, y: usize) -> Result<()> {
        // // For some reason, absolute mouse move events are not working on uinput
        // // (as I understand those events are intended for touch events)
        // //
        // // As a work around solution; first set the mouse to top left, then
        // // call relative move function to simulate an absolute move event
        //self.move_relative(i32::MIN, i32::MIN)?;
        //self.move_relative(x as i32, y as i32)

        self.emit(EV_ABS, ABS_X as i32, x as i32)?;
        self.emit(EV_ABS, ABS_Y as i32, y as i32)?;
        self.syncronize()
    }

    fn move_relative(&mut self, x_offset: i32, y_offset: i32) -> Result<()> {
        self.move_relative(x_offset, y_offset)
    }

    fn get_position(&self) -> Result<(i32, i32)> {
        // uinput does not let us get the current position of the mouse
        // Err(Error::NotImplemented)
        unimplemented!()
    }

    fn press_button(&mut self, button: &MouseButton) -> Result<()> {
        let btn = match button {
            MouseButton::Left => BTN_LEFT,
            MouseButton::Right => BTN_RIGHT,
            MouseButton::Middle => BTN_MIDDLE,
        };
        self.emit(EV_KEY, btn, 1)?;
        self.syncronize()
    }

    fn release_button(&mut self, button: &MouseButton) -> Result<()> {
        let btn = match button {
            MouseButton::Left => BTN_LEFT,
            MouseButton::Right => BTN_RIGHT,
            MouseButton::Middle => BTN_MIDDLE,
        };
        self.emit(EV_KEY, btn, 0)?;
        self.syncronize()
    }

    fn click_button(&mut self, button: &MouseButton) -> Result<()> {
        self.press_button(&button)?;
        self.release_button(&button)
    }

    fn scroll_wheel(&mut self, direction: &ScrollDirection) -> Result<()> {
        let (code, scroll_value) = match direction {
            ScrollDirection::Up => (REL_WHEEL, 1),
            ScrollDirection::Down => (REL_WHEEL, -1),
            ScrollDirection::Left => (REL_HWHEEL, -1),
            ScrollDirection::Right => (REL_HWHEEL, 1),
        };
        self.emit(EV_REL, code as i32, scroll_value)?;
        self.syncronize()
    }

    fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId> {
        if !self.is_listening {
            super::start_nix_listener(&self.callbacks)?;
            self.is_listening = true;
        }

        let id = self.callback_counter;
        self.callbacks.lock().unwrap().insert(id, callback);
        self.callback_counter += 1;
        Ok(id)
    }

    fn unhook(&mut self, callback_id: CallbackId) -> Result<()> {
        match self.callbacks.lock().unwrap().remove(&callback_id) {
            Some(_) => Ok(()),
            None => Err(Error::new(
                ErrorKind::NotFound,
                format!("callback id {} not found", callback_id),
            )),
        }
    }

    fn unhook_all(&mut self) -> Result<()> {
        self.callbacks.lock().unwrap().clear();
        Ok(())
    }
}

/// ioctl and uinput definitions
const UI_ABS_SETUP: c_ulong = 1075598596;
const UI_SET_EVBIT: c_ulong = 1074025828;
const UI_SET_KEYBIT: c_ulong = 1074025829;
const UI_SET_RELBIT: c_ulong = 1074025830;
const UI_SET_ABSBIT: c_ulong = 1074025831;
const UI_DEV_SETUP: c_ulong = 1079792899;
const UI_DEV_CREATE: c_ulong = 21761;
const UI_DEV_DESTROY: c_uint = 21762;

pub const EV_KEY: c_int = 0x01;
pub const EV_REL: c_int = 0x02;
pub const EV_ABS: c_int = 0x03;
pub const REL_X: c_uint = 0x00;
pub const REL_Y: c_uint = 0x01;
pub const ABS_X: c_uint = 0x00;
pub const ABS_Y: c_uint = 0x01;
pub const REL_HWHEEL: c_uint = 0x06;
pub const REL_WHEEL: c_uint = 0x08;
pub const BTN_LEFT: c_int = 0x110;
pub const BTN_RIGHT: c_int = 0x111;
pub const BTN_MIDDLE: c_int = 0x112;
const SYN_REPORT: c_int = 0x00;
const EV_SYN: c_int = 0x00;
const BUS_USB: c_ushort = 0x03;

/// uinput types
#[repr(C)]
struct UInputSetup {
    id: InputId,
    name: CString,
    ff_effects_max: c_ulong,
}

#[repr(C)]
struct InputId {
    bustype: c_ushort,
    vendor: c_ushort,
    product: c_ushort,
    version: c_ushort,
}

#[repr(C)]
pub struct InputEvent {
    pub time: TimeVal,
    pub r#type: u16,
    pub code: u16,
    pub value: c_int,
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: c_ulong,
    pub tv_usec: c_ulong,
}

#[repr(C)]
pub struct UinputAbsSetup {
    pub code: u16,
    pub absinfo: InputAbsinfo,
}

#[repr(C)]
pub struct InputAbsinfo {
    pub value: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub fuzz: i32,
    pub flat: i32,
    pub resolution: i32,
}

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}
