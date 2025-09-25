/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Mouse event listener contract

pub use super::mouse_input_listener::{MouseButton, MouseEvent, MouseModifiers};

/// Base contract for mouse listeners with default no-op implementations
///
/// Dummy implementation of mouse listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseMouseListenerContract {
    /// Called when the mouse is clicked - no default implementation provided intentionally
    fn mouse_clicked(&mut self, mouse_event: &MouseEvent);

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
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseMouseListenerContract for TestListener {
        fn mouse_clicked(&mut self, _mouse_event: &MouseEvent) {
            // Required implementation
        }
    }

    #[test]
    fn test_mouse_listener() {
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

        // Test required method
        listener.mouse_clicked(&event);

        // Test default implementations
        listener.mouse_pressed(&event);
        listener.mouse_released(&event);
        listener.mouse_entered(&event);
        listener.mouse_exited(&event);
    }
}
