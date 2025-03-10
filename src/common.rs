use crate::error::Error;

pub type CallbackId = u8;

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum ScrollDirection {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
    RelativeMove(i32, i32),
    AbsoluteMove(i32, i32),
    Press(MouseButton),
    Release(MouseButton),
    Scroll(ScrollDirection, u32),
}

pub trait MouseActions {
    /// Move the mouse to the given `x`, `y` coordinates in logical pixel space
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::MouseActions;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_to(0, 0), Ok(()));
    /// ```
    fn move_to(&self, x: i32, y: i32) -> Result<(), Error>;
    /// Move the mouse relative to the current position in logical pixel space
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::MouseActions;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.move_relative(100, 100), Ok(()));
    /// ```
    fn move_relative(&self, x_offset: i32, y_offset: i32) -> Result<(), Error>;
    /// Get the current position of the mouse in logical pixel space
    ///
    /// # Examples
    ///
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::MouseActions;
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
    /// use mouce::MouseActions;
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
    /// use mouce::MouseActions;
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
    /// use mouce::MouseActions;
    /// use mouce::common::MouseButton;
    ///
    /// let manager = Mouse::new();
    /// assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
    /// ```
    fn click_button(&self, button: &MouseButton) -> Result<(), Error>;
    /// Scroll the mouse wheel towards to the given direction with the given distance
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::MouseActions;
    /// use mouce::common::ScrollDirection;
    /// use std::{thread, time};
    ///
    /// let manager = Mouse::new();
    /// let sleep_duration = time::Duration::from_millis(250);
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Down, 5), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    ///
    /// for _ in 0..5 {
    ///     assert_eq!(manager.scroll_wheel(&ScrollDirection::Up, 5), Ok(()));
    ///     thread::sleep(sleep_duration);
    /// }
    /// ```
    fn scroll_wheel(&self, direction: &ScrollDirection, distance: u32) -> Result<(), Error>;
    /// Attach a callback function to mouse events
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mouce::Mouse;
    /// use mouce::MouseActions;
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
    /// use mouce::MouseActions;
    ///
    /// let mut manager = Mouse::new();
    /// assert_eq!(manager.unhook_all(), Ok(()));
    /// ```
    fn unhook_all(&mut self) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use crate::error::Error;
    use crate::MouseActions;
    use crate::{common::MouseButton, common::ScrollDirection, Mouse};
    use std::sync::Mutex;
    use std::{thread, time};

    // Prevent test running in parallel,
    static TEST_EXECUTER: Mutex<Executor> = Mutex::new(Executor);
    #[derive(Clone, Copy)]
    struct Executor;

    impl Executor {
        fn run_test(self, f: impl FnOnce()) {
            f();
        }
    }

    #[test]
    #[ignore]
    fn move_to_right_bottom() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            assert_eq!(manager.move_to(1920, 1080), Ok(()));
        });
    }

    #[test]
    #[ignore]
    fn move_to_top_left() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            assert_eq!(manager.move_to(0, 0), Ok(()));
        });
    }

    #[test]
    #[ignore]
    fn move_to_left_to_right() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            let sleep_duration = time::Duration::from_millis(1);
            let mut x = 0;
            while x < 1920 {
                assert_eq!(manager.move_to(x, 540), Ok(()));
                x += 1;
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn move_relative_left_to_right() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            let sleep_duration = time::Duration::from_millis(1);
            let mut x = 0;
            assert_eq!(manager.move_to(0, 540), Ok(()));
            while x < 1920 {
                assert_eq!(manager.move_relative(1, 0), Ok(()));
                x += 1;
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn move_to_top_to_bottom() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            let sleep_duration = time::Duration::from_millis(1);
            let mut y = 0;
            while y < 1080 {
                assert_eq!(manager.move_to(960, y), Ok(()));
                y += 1;
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn move_relative_top_to_bottom() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            let sleep_duration = time::Duration::from_millis(1);
            let mut y = 0;
            assert_eq!(manager.move_to(960, 0), Ok(()));
            while y < 1080 {
                assert_eq!(manager.move_relative(0, 1), Ok(()));
                y += 1;
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn get_position() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            match manager.get_position() {
                Ok(_) => {
                    let positions = vec![
                        (0, 0),
                        (100, 100),
                        (250, 250),
                        (325, 325),
                        (400, 100),
                        (100, 400),
                    ];

                    let mut x;
                    let mut y;
                    for position in positions.iter() {
                        assert_eq!(manager.move_to(position.0, position.1), Ok(()));
                        (x, y) = manager.get_position().unwrap();
                        assert_eq!(x, position.0 as i32);
                        assert_eq!(y, position.1 as i32);
                    }
                }
                Err(error) => assert_eq!(error, Error::NotImplemented),
            }
        });
    }

    #[test]
    #[ignore]
    fn left_click() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
        });
    }

    #[test]
    #[ignore]
    fn scroll_down() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            for _ in 0..10 {
                assert_eq!(manager.scroll_wheel(&ScrollDirection::Down, 5), Ok(()));
                let sleep_duration = time::Duration::from_millis(250);
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn scroll_up() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            for _ in 0..10 {
                assert_eq!(manager.scroll_wheel(&ScrollDirection::Up, 5), Ok(()));
                let sleep_duration = time::Duration::from_millis(250);
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn scroll_right() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            for _ in 0..10 {
                assert_eq!(manager.scroll_wheel(&ScrollDirection::Right, 5), Ok(()));
                let sleep_duration = time::Duration::from_millis(250);
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn scroll_left() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let manager = Mouse::new();
            for _ in 0..10 {
                assert_eq!(manager.scroll_wheel(&ScrollDirection::Left, 5), Ok(()));
                let sleep_duration = time::Duration::from_millis(250);
                thread::sleep(sleep_duration);
            }
        });
    }

    #[test]
    #[ignore]
    fn hook_and_unhook() {
        TEST_EXECUTER.lock().unwrap().run_test(|| {
            let mut manager = Mouse::new();
            assert_eq!(manager.unhook(5), Err(Error::UnhookFailed));
            let hook_result = manager.hook(Box::new(|e| println!("{:?}", e)));
            match hook_result {
                Ok(id) => {
                    assert_eq!(manager.unhook(id), Ok(()));

                    manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
                    manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
                    manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();
                    let id = manager.hook(Box::new(|e| println!("{:?}", e))).unwrap();

                    manager.unhook_all().unwrap();
                    assert_eq!(manager.unhook(id), Err(Error::UnhookFailed));
                }
                Err(err) => assert_eq!(Error::PermissionDenied, err),
            }
        });
    }
}
