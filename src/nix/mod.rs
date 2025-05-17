///
/// This module contains the mouse action functions
/// for the unix-like systems
///
use crate::common::{CallbackId, MouseButton, MouseEvent, ScrollDirection};
use crate::error::Error;
use crate::nix::uinput::{
    InputEvent, TimeVal, BTN_LEFT, BTN_MIDDLE, BTN_RIGHT, EV_KEY, EV_REL, REL_HWHEEL, REL_WHEEL,
    REL_X, REL_Y,
};
use glob::glob;
use std::collections::HashMap;
use std::fs::File;
use std::mem::size_of;
use std::os::unix::io::AsRawFd;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(feature = "x11")]
use std::{process::Command, str::from_utf8};
#[cfg(feature = "x11")]
pub mod x11;

pub mod uinput;

type Callbacks = Arc<Mutex<HashMap<CallbackId, Box<dyn Fn(&MouseEvent) + Send>>>>;

/// Start the event listener for nix systems
fn start_nix_listener(callbacks: &Callbacks) -> Result<(), Error> {
    let (tx, rx) = mpsc::channel();

    let mut previous_paths = vec![];
    // Read all the mouse events listed under /dev/input/by-id and
    // /dev/input/by-path. These directories are collections of symlinks
    // to /dev/input/event*
    //
    // I am only interested in the ones that end with `-event-mouse`
    for file in glob("/dev/input/by-id/*-event-mouse")
        .expect("Failed to read by-id glob pattern")
        .chain(
            glob("/dev/input/by-path/*-event-mouse").expect("Failed to read by-path glob pattern"),
        )
    {
        let mut file = file.expect("Failed because of an IO error");

        // Get the link if it exists
        if let Ok(rel_path) = file.read_link() {
            if rel_path.is_absolute() {
                file = rel_path;
            } else {
                // Remove the file name from the path buffer, leaving us with path to directory
                file.pop();
                // Push the relative path of the link (e.g. `../event8`)
                file.push(rel_path);
                // Get the absolute path to final path
                file = std::fs::canonicalize(file)
                    .expect("Can't get absolute path to linked device file");
            }
        }

        let path = file.display().to_string();

        if previous_paths.contains(&path) {
            continue;
        }

        previous_paths.push(path.clone());

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
            let val = received.value;

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
                    MouseEvent::Scroll(
                        if received.value > 0 {
                            ScrollDirection::Up
                        } else {
                            ScrollDirection::Down
                        },
                        received.value.unsigned_abs(),
                    )
                } else if code == REL_HWHEEL {
                    MouseEvent::Scroll(
                        if received.value > 0 {
                            ScrollDirection::Right
                        } else {
                            ScrollDirection::Left
                        },
                        received.value.unsigned_abs(),
                    )
                } else if code == REL_X {
                    MouseEvent::RelativeMove(val, 0)
                } else if code == REL_Y {
                    MouseEvent::RelativeMove(0, val)
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

// Legacy function to check if x11 is available, it was used to fallback to uinput if
// X11 was not available, this feature is not included anymore but perhaps can be reimplemented
// in the build.rs to determine if x11 is enabled but not available in compile time
#[cfg(feature = "x11")]
fn is_x11() -> bool {
    // Try to verify x11 using loginctl
    let loginctl_output = Command::new("sh")
        .arg("-c")
        .arg("loginctl show-session $(loginctl | awk '/tty/ {print $1}') -p Type --value")
        .output();

    if let Ok(out) = loginctl_output {
        if from_utf8(&out.stdout).unwrap().trim().to_lowercase() == "x11" {
            return true;
        }
    }

    // If loginctl fails try to read the environment variable $XDG_SESSION_TYPE
    if let Ok(session_type) = std::env::var("XDG_SESSION_TYPE") {
        if session_type.trim().to_lowercase() == "x11" {
            return true;
        }
    }

    false
}

extern "C" {
    fn read(fd: i32, buf: *mut InputEvent, count: usize) -> i32;
}
