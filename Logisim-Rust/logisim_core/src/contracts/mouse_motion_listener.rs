/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Mouse motion event listener contract

pub use super::mouse_input_listener::{MouseButton, MouseEvent, MouseModifiers};

/// Base contract for mouse motion listeners with default no-op implementations
///
/// Dummy implementation of mouse motion listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseMouseMotionListenerContract {
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

    impl BaseMouseMotionListenerContract for TestListener {}

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
        listener.mouse_dragged(&event);
        listener.mouse_moved(&event);
    }
}
