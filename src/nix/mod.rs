///
/// This module contains the mouse action functions
/// for the unix-like systems
///
use crate::common::{CallbackId, MouseActions, MouseButton, MouseEvent, ScrollDirection};
use crate::error::Error;
use crate::nix::uinput::{
    InputEvent, TimeVal, BTN_LEFT, BTN_MIDDLE, BTN_RIGHT, EV_KEY, EV_REL, REL_WHEEL, REL_X, REL_Y,
};
use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::process::Command;
use std::str::from_utf8;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

mod uinput;
mod x11;

pub struct NixMouseManager {}

impl NixMouseManager {
    pub fn new() -> Box<dyn MouseActions> {
        // Try to identify the display manager using loginctl, if it fails
        // read the environment variable $XDG_SESSION_TYPE
        let output = Command::new("sh")
            .arg("-c")
            .arg("loginctl show-session $(awk '/tty/ {print $1}' <(loginctl)) -p Type | awk -F= '{print $2}'")
            .output()
            .unwrap_or(
                Command::new("sh")
                    .arg("-c")
                    .arg("echo $XDG_SESSION_TYPE")
                    .output().unwrap()
                );

        let display_manager = from_utf8(&output.stdout).unwrap().trim();

        match display_manager {
            "x11" => Box::new(x11::X11MouseManager::new()),
            // If the display manager is unknown default to uinput
            _ => Box::new(uinput::UInputMouseManager::new()),
        }
    }
}

/// Start the event listener for nix systems
fn start_nix_listener(
    callbacks: &Arc<Mutex<HashMap<CallbackId, Box<dyn Fn(&MouseEvent) + Send>>>>,
) -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();

    // Read all the mouse events listed under /dev/input/by-id
    // by-id directory is a collection of symlinks to /dev/input/event*
    // I am only interested in the ones that end with `-event-mouse`
    for file in glob("/dev/input/by-id/*-event-mouse").expect("Failed to read glob pattern") {
        let path = file
            .expect("Failed because of an IO error")
            .display()
            .to_string();

        let event = match File::options().read(true).open(path) {
            Ok(file) => file,
            Err(_) => return Err(Error::PermissionDenied),
        };

        // Create a thread for this mouse-event file
        let tx = tx.clone();
        thread::spawn(move || loop {
            let mut buffer = InputEvent {
                time: TimeVal {
                    tv_sec: 0,
                    tv_usec: 0,
                },
                r#type: 0,
                code: 0,
                value: 0,
            };
            unsafe {
                read(event.as_raw_fd(), &mut buffer, size_of::<InputEvent>());
            }
            tx.send(buffer).unwrap();
        });
    }

    let callbacks = callbacks.clone();
    // Create a thread for handling the callbacks
    thread::spawn(move || {
        for received in rx {
            // Construct the library's MouseEvent
            let r#type = received.r#type as i32;
            let code = received.code as i32;
            let val = received.value as i32;

            let mouse_event = if r#type == EV_KEY {
                let button = if code == BTN_LEFT {
                    MouseButton::Left
                } else if code == BTN_RIGHT {
                    MouseButton::Right
                } else if code == BTN_MIDDLE {
                    MouseButton::Middle
                } else {
                    // Ignore the unknown mouse buttons
                    continue;
                };

                if received.value == 1 {
                    MouseEvent::Press(button)
                } else {
                    MouseEvent::Release(button)
                }
            } else if r#type == EV_REL {
                let code = received.code as u32;
                if code == REL_WHEEL {
                    MouseEvent::Scroll(if received.value > 0 {
                        ScrollDirection::Up
                    } else {
                        ScrollDirection::Down
                    })
                } else if code == REL_X {
                    MouseEvent::Move(val, 0)
                } else if code == REL_Y {
                    MouseEvent::Move(0, val)
                } else {
                    continue;
                }
            } else {
                // Ignore other unknown events
                continue;
            };

            // Invoke all given callbacks with the constructed mouse event
            for callback in callbacks.lock().unwrap().values() {
                callback(&mouse_event);
            }
        }
    });

    Ok(())
}

extern "C" {
    fn read(fd: i32, buf: *mut InputEvent, count: usize) -> i32;
}
