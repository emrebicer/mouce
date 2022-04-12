///
/// This module contains the mouse action functions
/// for the unix-like systems
///
use crate::common::MouseActions;

use std::process::Command;
use std::str::from_utf8;

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
