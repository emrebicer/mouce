use crate::error::Error;

pub type CallbackId = u8;

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[cfg(not(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
#[derive(Debug)]
pub enum ScrollDirection {
    Up,
    Down,
}

#[cfg(any(
    target_os = "windows",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
#[derive(Debug)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum MouseEvent {
    RelativeMove(i32, i32),
    AbsoluteMove(i32, i32),
    Press(MouseButton),
    Release(MouseButton),
    Scroll(ScrollDirection),
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub trait MouseActions {
    /// Move the mouse to the given `x`, `y` coordinates
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_to(0, 0), Ok(()));
    /// ```
    fn move_to(&mut self, x: usize, y: usize) -> std::io::Result<()>;
    /// Move the mouse relative to the current position
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_relative(100, 100), Ok(()));
    /// ```
    fn move_relative(&mut self, x_offset: i32, y_offset: i32) -> std::io::Result<()> {
        let (x, y) = self.get_position()?;
        self.move_to((x + x_offset) as usize, (y + y_offset) as usize)
    }
    /// Get the current position of the mouse
    ///
    /// # Examples
    ///
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::error::Error;
    ///
    /// let manager = Mouse::new();
    /// manager.move_to(0, 0);
    /// // This function may not be implemented on some platforms such as Linux Wayland
    /// let valid_outs = vec![Ok((0, 0)), Err(Error::NotImplemented)];
    /// assert!(valid_outs.contains(&manager.get_position()));
    /// ```
    fn get_position(&self) -> std::io::Result<(i32, i32)>;
    /// Press down the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.press_button(&MouseButton::Left), Ok(()));
    /// ```
    fn press_button(&mut self, button: &MouseButton) -> std::io::Result<()>;
    /// Release the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.release_button(&MouseButton::Left), Ok(()));
    /// ```
    fn release_button(&mut self, button: &MouseButton) -> std::io::Result<()>;
    /// Click the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
    /// ```
    fn click_button(&mut self, button: &MouseButton) -> std::io::Result<()> {
        self.press_button(&button)?;
        self.release_button(&button)
    }
    /// Scroll the mouse wheel towards to the given direction
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::ScrollDirection;
    /// use std::{thread, time};
    ///
    /// let manager = Mouse::new();
    /// let sleep_duration = time::Duration::from_millis(250);
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Down), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Up), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    /// ```
    fn scroll_wheel(&mut self, direction: &ScrollDirection) -> std::io::Result<()>;
    /// Attach a callback function to mouse events
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::error::Error;
    ///
    /// let mut manager = Mouse::new();
    /// let hook_result = manager.hook(Box::new(|e| println!("New event: {:?}", e)));
    /// match hook_result {
    ///     Ok(id) => {
    ///         assert_eq!(manager.unhook(id), Ok(()));
    ///     }
    ///     // Hooking may require user privileges on some systems
    ///     // e.g. requires super user for Linux
    ///     Err(err) => assert_eq!(Error::PermissionDenied, err),
    /// }
    /// ```
    fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId, Error>;
    /// Remove the callback function with the given `CallbackId`
    fn unhook(&mut self, callback_id: CallbackId) -> Result<(), Error>;
    /// Remove all callback functions
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let mut manager = Mouse::new();
    /// assert_eq!(manager.unhook_all(), Ok(()));
    /// ```
    fn unhook_all(&mut self) -> Result<(), Error>;
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
)))]
pub trait MouseActions {
    /// Move the mouse to the given `x`, `y` coordinates
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_to(0, 0), Ok(()));
    /// ```
    fn move_to(&self, x: usize, y: usize) -> Result<(), Error>;
    /// Move the mouse relative to the current position
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_relative(100, 100), Ok(()));
    /// ```
    fn move_relative(&self, x_offset: i32, y_offset: i32) -> Result<(), Error> {
        let (x, y) = self.get_position()?;
        self.move_to((x + x_offset) as usize, (y + y_offset) as usize)
    }
    /// Get the current position of the mouse
    ///
    /// # Examples
    ///
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::error::Error;
    ///
    /// let manager = Mouse::new();
    /// manager.move_to(0, 0);
    /// // This function may not be implemented on some platforms such as Linux Wayland
    /// let valid_outs = vec![Ok((0, 0)), Err(Error::NotImplemented)];
    /// assert!(valid_outs.contains(&manager.get_position()));
    /// ```
    fn get_position(&self) -> Result<(i32, i32), Error>;
    /// Press down the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.press_button(&MouseButton::Left), Ok(()));
    /// ```
    fn press_button(&self, button: &MouseButton) -> Result<(), Error>;
    /// Release the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.release_button(&MouseButton::Left), Ok(()));
    /// ```
    fn release_button(&self, button: &MouseButton) -> Result<(), Error>;
    /// Click the given mouse button
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
    /// ```
    fn click_button(&self, button: &MouseButton) -> Result<(), Error> {
        self.press_button(&button)?;
        self.release_button(&button)
    }
    /// Scroll the mouse wheel towards to the given direction
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::common::ScrollDirection;
    /// use std::{thread, time};
    ///
    /// let manager = Mouse::new();
    /// let sleep_duration = time::Duration::from_millis(250);
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Down), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Up), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    /// ```
    fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error>;
    /// Attach a callback function to mouse events
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::error::Error;
    ///
    /// let mut manager = Mouse::new();
    /// let hook_result = manager.hook(Box::new(|e| println!("New event: {:?}", e)));
    /// match hook_result {
    ///     Ok(id) => {
    ///         assert_eq!(manager.unhook(id), Ok(()));
    ///     }
    ///     // Hooking may require user privileges on some systems
    ///     // e.g. requires super user for Linux
    ///     Err(err) => assert_eq!(Error::PermissionDenied, err),
    /// }
    /// ```
    fn hook(&mut self, callback: Box<dyn Fn(&MouseEvent) + Send>) -> Result<CallbackId, Error>;
    /// Remove the callback function with the given `CallbackId`
    fn unhook(&mut self, callback_id: CallbackId) -> Result<(), Error>;
    /// Remove all callback functions
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    ///
    /// let mut manager = Mouse::new();
    /// assert_eq!(manager.unhook_all(), Ok(()));
    /// ```
    fn unhook_all(&mut self) -> Result<(), Error>;
}

// #[cfg(test)]
// mod tests {
//     use crate::error::Error;
//     use crate::MouseActions;
//     use crate::{common::MouseButton, common::ScrollDirection, Mouse};
//     use std::{thread, time};

//     #[ignore]
//     #[cfg(any(
//         target_os = "linux",
//         target_os = "dragonfly",
//         target_os = "freebsd",
//         target_os = "netbsd",
//         target_os = "openbsd"
//     ))]
//     fn get_mouse_manager() -> Box<dyn MouseActions> {
//         Mouse::new((0, 1920), (0, 1080))
//     }

//     #[ignore]
//     #[cfg(not(any(
//         target_os = "linux",
//         target_os = "dragonfly",
//         target_os = "freebsd",
//         target_os = "netbsd",
//         target_os = "openbsd"
//     )))]
//     fn get_mouse_manager() -> Box<dyn MouseActions> {
//         Mouse::new()
//     }

//     #[test]
//     #[ignore]
//     fn move_to_right_bottom() {
//         let mut manager = get_mouse_manager();
//         assert_eq!(manager.move_to(1920, 1080), Ok(()));
//     }

//     #[test]
//     #[ignore]
//     fn move_to_top_left() {
//         let mut manager = get_mouse_manager();
//         assert_eq!(manager.move_to(0, 0), Ok(()));
//     }

//     #[test]
//     #[ignore]
//     fn move_to_left_to_right() {
//         let mut manager = get_mouse_manager();
//         let sleep_duration = time::Duration::from_millis(1);
//         let mut x = 0;
//         while x < 1920 {
//             assert_eq!(manager.move_to(x, 540), Ok(()));
//             x += 1;
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn move_relative_left_to_right() {
//         let mut manager = get_mouse_manager();
//         let sleep_duration = time::Duration::from_millis(1);
//         let mut x = 0;
//         assert_eq!(manager.move_to(0, 540), Ok(()));
//         while x < 1920 {
//             assert_eq!(manager.move_relative(1, 0), Ok(()));
//             x += 1;
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn move_to_top_to_bottom() {
//         let mut manager = get_mouse_manager();
//         let sleep_duration = time::Duration::from_millis(1);
//         let mut y = 0;
//         while y < 1080 {
//             assert_eq!(manager.move_to(960, y), Ok(()));
//             y += 1;
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn move_relative_top_to_bottom() {
//         let mut manager = get_mouse_manager();
//         let sleep_duration = time::Duration::from_millis(1);
//         let mut y = 0;
//         assert_eq!(manager.move_to(960, 0), Ok(()));
//         while y < 1080 {
//             assert_eq!(manager.move_relative(0, 1), Ok(()));
//             y += 1;
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn get_position() {
//         let mut manager = get_mouse_manager();
//         match manager.get_position() {
//             Ok(_) => {
//                 let positions = vec![
//                     (0, 0),
//                     (100, 100),
//                     (250, 250),
//                     (325, 325),
//                     (400, 100),
//                     (100, 400),
//                 ];

//                 let mut x;
//                 let mut y;
//                 for position in positions.iter() {
//                     assert_eq!(manager.move_to(position.0, position.1), Ok(()));
//                     (x, y) = manager.get_position().unwrap();
//                     assert_eq!(x, position.0 as i32);
//                     assert_eq!(y, position.1 as i32);
//                 }
//             }
//             Err(error) => assert_eq!(error, Error::NotImplemented),
//         }
//     }

//     #[test]
//     #[ignore]
//     fn left_click() {
//         let mut manager = get_mouse_manager();
//         assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
//     }

//     #[test]
//     #[ignore]
//     fn scroll_down() {
//         let mut manager = get_mouse_manager();
//         for _ in 0..10 {
//             assert_eq!(manager.scroll_wheel(&ScrollDirection::Down), Ok(()));
//             let sleep_duration = time::Duration::from_millis(250);
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn scroll_up() {
//         let mut manager = get_mouse_manager();
//         for _ in 0..10 {
//             assert_eq!(manager.scroll_wheel(&ScrollDirection::Up), Ok(()));
//             let sleep_duration = time::Duration::from_millis(250);
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[cfg(any(
//         target_os = "windows",
//         target_os = "linux",
//         target_os = "dragonfly",
//         target_os = "freebsd",
//         target_os = "netbsd",
//         target_os = "openbsd"
//     ))]
//     #[test]
//     #[ignore]
//     fn scroll_left() {
//         let mut manager = get_mouse_manager();
//         for _ in 0..10 {
//             assert_eq!(manager.scroll_wheel(&ScrollDirection::Left), Ok(()));
//             let sleep_duration = time::Duration::from_millis(250);
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[cfg(any(
//         target_os = "windows",
//         target_os = "linux",
//         target_os = "dragonfly",
//         target_os = "freebsd",
//         target_os = "netbsd",
//         target_os = "openbsd"
//     ))]
//     #[test]
//     #[ignore]
//     fn scroll_right() {
//         let mut manager = get_mouse_manager();
//         for _ in 0..10 {
//             assert_eq!(manager.scroll_wheel(&ScrollDirection::Right), Ok(()));
//             let sleep_duration = time::Duration::from_millis(250);
//             thread::sleep(sleep_duration);
//         }
//     }

//     #[test]
//     #[ignore]
//     fn hook_and_unhook() {
//         let mut manager = get_mouse_manager();
//         assert_eq!(manager.unhook(5), Err(Error::UnhookFailed));
//         let hook_result = manager.hook(Box::new(|e| println!("{:?}", e)));
//         match hook_result {
//             Ok(id) => {
//                 assert_eq!(manager.unhook(id), Ok(()));

//                 manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
//                 manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
//                 manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
//                 let id = manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();

//                 manager.unhook_all().unwrap();
//                 assert_eq!(manager.unhook(id), Err(Error::UnhookFailed));
//             }
//             Err(err) => assert_eq!(Error::PermissionDenied, err),
//         }
//     }
// }
