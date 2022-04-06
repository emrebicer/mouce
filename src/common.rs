pub enum MouseButton {
    LeftClick,
    MiddleClick,
    RightClick,
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
