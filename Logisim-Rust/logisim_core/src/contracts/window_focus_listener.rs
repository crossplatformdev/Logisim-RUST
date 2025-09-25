/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Window focus event listener contract

/// Window event data
#[derive(Debug, Clone)]
pub struct WindowEvent {
    pub window_id: u32,
    pub event_type: WindowEventType,
}

#[derive(Debug, Clone)]
pub enum WindowEventType {
    FocusGained,
    FocusLost,
    Opened,
    Closing,
    Closed,
    Iconified,
    Deiconified,
    Activated,
    Deactivated,
}

/// Base contract for window focus listeners with default no-op implementations
///
/// Dummy implementation of window focus listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseWindowFocusListenerContract {
    /// Called when the window gains focus
    fn window_gained_focus(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }

    /// Called when the window loses focus
    fn window_lost_focus(&mut self, _event: &WindowEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseWindowFocusListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = WindowEvent {
            window_id: 1,
            event_type: WindowEventType::FocusGained,
        };

        // Should not panic - all methods have default implementations
        listener.window_gained_focus(&event);
        listener.window_lost_focus(&event);
    }
}
