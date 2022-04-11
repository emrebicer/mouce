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
    fn move_to(&self, x: usize, y: usize);
    /// Get the current position of the mouse
    fn get_position(&self) -> (i32, i32);
    /// Press down the given mouse button
    fn press_button(&self, button: &MouseButton);
    /// Release the given mouse button
    fn release_button(&self, button: &MouseButton);
    /// Click the given mouse button
    fn click_button(&self, button: &MouseButton);
    /// Scroll the mouse wheel towards to the given direction
    fn scroll_wheel(&self, direction: &ScrollDirection);
}

#[cfg(test)]
mod tests {
    use crate::{common::MouseButton, common::ScrollDirection, Mouse};
    use std::{thread, time};

    #[test]
    #[ignore]
    fn move_to_right_bottom() {
        let manager = Mouse::new();
        manager.move_to(1920, 1080);
    }

    #[test]
    #[ignore]
    fn move_to_top_left() {
        let manager = Mouse::new();
        manager.move_to(0, 0);
    }

    #[test]
    #[ignore]
    fn move_to_left_to_right() {
        let manager = Mouse::new();
        let sleep_duration = time::Duration::from_millis(1);
        let mut x = 0;
        while x < 1920 {
            manager.move_to(x, 540);
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
            manager.move_to(960, y);
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
            manager.move_to(position.0, position.1);
            (x, y) = manager.get_position();
            assert_eq!(x, position.0 as i32);
            assert_eq!(y, position.1 as i32);
        }
    }

    #[test]
    #[ignore]
    fn left_click() {
        let manager = Mouse::new();
        manager.click_button(&MouseButton::Left);
    }

    #[test]
    #[ignore]
    fn scroll_down() {
        let manager = Mouse::new();
        for _ in 0..10 {
            manager.scroll_wheel(&ScrollDirection::Down);
            let sleep_duration = time::Duration::from_millis(250);
            thread::sleep(sleep_duration);
        }
    }

    #[test]
    #[ignore]
    fn scroll_up() {
        let manager = Mouse::new();
        for _ in 0..10 {
            manager.scroll_wheel(&ScrollDirection::Up);
            let sleep_duration = time::Duration::from_millis(250);
            thread::sleep(sleep_duration);
        }
    }
}
