use crate::error::Error;

pub enum MouseButton {
    Left,
    Middle,
    Right,
}

pub enum ScrollDirection {
    Up,
    Down,
}

pub trait MouseActions {
    /// Move the mouse to the given `x`, `y` coordinates
    fn move_to(&self, x: usize, y: usize) -> Result<(), Error>;
    /// Move the mouse relative to the current position
    fn move_relative(&self, x_offset: i32, y_offset: i32) -> Result<(), Error> {
        let (x, y) = self.get_position()?;
        self.move_to((x + x_offset) as usize, (y + y_offset) as usize)
    }
    /// Get the current position of the mouse
    fn get_position(&self) -> Result<(i32, i32), Error>;
    /// Press down the given mouse button
    fn press_button(&self, button: &MouseButton) -> Result<(), Error>;
    /// Release the given mouse button
    fn release_button(&self, button: &MouseButton) -> Result<(), Error>;
    /// Click the given mouse button
    fn click_button(&self, button: &MouseButton) -> Result<(), Error>{
        self.press_button(&button)?;
        self.release_button(&button)
    }
    /// Scroll the mouse wheel towards to the given direction
    fn scroll_wheel(&self, direction: &ScrollDirection) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use crate::{common::MouseButton, common::ScrollDirection, Mouse};
    use std::{thread, time};

    #[test]
    #[ignore]
    fn move_to_right_bottom() {
        let manager = Mouse::new();
        assert_eq!(manager.move_to(1920, 1080), Ok(()));
    }

    #[test]
    #[ignore]
    fn move_to_top_left() {
        let manager = Mouse::new();
        assert_eq!(manager.move_to(0, 0), Ok(()));
    }

    #[test]
    #[ignore]
    fn move_to_left_to_right() {
        let manager = Mouse::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut x = 0;
        while x < 1920 {
            assert_eq!(manager.move_to(x, 540), Ok(()));
            x += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn move_relative_left_to_right() {
        let manager = Mouse::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut x = 0;
        assert_eq!(manager.move_to(0, 540), Ok(()));
        while x < 1920 {
            assert_eq!(manager.move_relative(1, 0), Ok(()));
            x += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn move_to_top_to_bottom() {
        let manager = Mouse::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut y = 0;
        while y < 1080 {
            assert_eq!(manager.move_to(960, y), Ok(()));
            y += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn move_relative_top_to_bottom() {
        let manager = Mouse::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut y = 0;
        assert_eq!(manager.move_to(960, 0), Ok(()));
        while y < 1080 {
            assert_eq!(manager.move_relative(0, 1), Ok(()));
            y += 1;
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn get_position() {
        let manager = Mouse::new();
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

    #[test]
    #[ignore]
    fn left_click() {
        let manager = Mouse::new();
        assert_eq!(manager.click_button(&MouseButton::Left), Ok(()));
    }

    #[test]
    #[ignore]
    fn scroll_down() {
        let manager = Mouse::new();
        for _ in 0..10 {
            assert_eq!(manager.scroll_wheel(&ScrollDirection::Down), Ok(()));
            let sleep_duration = time::Duration::from_millis(250);
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn scroll_up() {
        let manager = Mouse::new();
        for _ in 0..10 {
            assert_eq!(manager.scroll_wheel(&ScrollDirection::Up), Ok(()));
            let sleep_duration = time::Duration::from_millis(250);
            thread::sleep(sleep_duration);
        }
    }
}
