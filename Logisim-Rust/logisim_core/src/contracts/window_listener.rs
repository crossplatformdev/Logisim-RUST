/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Window event listener contract

pub use super::window_focus_listener::{WindowEvent, WindowEventType};

/// Base contract for window listeners with default no-op implementations
///
/// Dummy implementation of window listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseWindowListenerContract {
    /// Called when the window is opened
    fn window_opened(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is closing
    fn window_closing(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is closed
    fn window_closed(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is iconified (minimized)
    fn window_iconified(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is deiconified (restored)
    fn window_deiconified(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is activated
    fn window_activated(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window is deactivated
    fn window_deactivated(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseWindowListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = WindowEvent {
            window_id: 1,
            event_type: WindowEventType::Opened,
        };

        // Should not panic - all methods have default implementations
        listener.window_opened(&event);
        listener.window_closing(&event);
        listener.window_closed(&event);
        listener.window_iconified(&event);
        listener.window_deiconified(&event);
        listener.window_activated(&event);
        listener.window_deactivated(&event);
    }
}