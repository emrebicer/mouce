///
/// This module contains the mouse action functions
/// for the unix-like systems
///
use crate::common::MouseActions;

use std::env;

mod linux_uinput;
mod x11;

pub struct NixMouseManager {}

impl NixMouseManager {
    pub fn new() -> Box<dyn MouseActions> {
        let display_manager = env::var("XDG_SESSION_TYPE").unwrap_or("x11".to_string());
        match display_manager.as_str() {
            "x11" => Box::new(x11::X11MouseManager::new()),
            _ => Box::new(linux_uinput::LinuxUInputMouseManager::new()),
        }
    }
}
