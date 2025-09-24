/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Mouse input event listener contract

/// Mouse event data
#[derive(Debug, Clone)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub modifiers: MouseModifiers,
    pub click_count: u32,
}

#[derive(Debug, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Debug, Clone)]
pub struct MouseModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

/// Base contract for mouse input listeners with default no-op implementations
///
/// Dummy implementation of mouse input listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseMouseInputListenerContract {
    /// Called when the mouse is clicked
    fn mouse_clicked(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when a mouse button is pressed
    fn mouse_pressed(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when a mouse button is released
    fn mouse_released(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when the mouse enters a component
    fn mouse_entered(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when the mouse exits a component
    fn mouse_exited(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when the mouse is dragged
    fn mouse_dragged(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }

    /// Called when the mouse is moved
    fn mouse_moved(&mut self, _mouse_event: &MouseEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseMouseInputListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = MouseEvent {
            x: 10,
            y: 20,
            button: MouseButton::Left,
            modifiers: MouseModifiers {
                shift: false,
                ctrl: false,
                alt: false,
                meta: false,
            },
            click_count: 1,
        };

        // Should not panic - all methods have default implementations
        listener.mouse_clicked(&event);
        listener.mouse_pressed(&event);
        listener.mouse_released(&event);
        listener.mouse_entered(&event);
        listener.mouse_exited(&event);
        listener.mouse_dragged(&event);
        listener.mouse_moved(&event);
    }
}