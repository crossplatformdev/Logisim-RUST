/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Key event listener contract

/// Key event data
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key_code: u32,
    pub key_char: Option<char>,
    pub modifiers: KeyModifiers,
    pub event_type: KeyEventType,
}

#[derive(Debug, Clone)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone)]
pub enum KeyEventType {
    Typed,
    Pressed,
    Released,
}

/// Base contract for key listeners with default no-op implementations
///
/// Dummy implementation of key listener interface. The main purpose of this trait
/// is to provide default (empty) implementation of interface methods as, unfortunately
/// many UI frameworks' interfaces do not come with default implementation even they easily could.
/// Implementing this trait instead of the parent one allows skipping the need of implementing
/// all, even unneeded, methods. That saves some efforts and reduces overall LOC.
pub trait BaseKeyListenerContract {
    /// Called when a key is typed (pressed and released)
    fn key_typed(&mut self, _key_event: &KeyEvent) {
        // no-op implementation
    }

    /// Called when a key is pressed
    fn key_pressed(&mut self, _key_event: &KeyEvent) {
        // no-op implementation
    }

    /// Called when a key is released
    fn key_released(&mut self, _key_event: &KeyEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseKeyListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = KeyEvent {
            key_code: 65, // 'A'
            key_char: Some('A'),
            modifiers: KeyModifiers {
                shift: false,
                ctrl: false,
                alt: false,
                meta: false,
            },
            event_type: KeyEventType::Typed,
        };

        // Should not panic - all methods have default implementations
        listener.key_typed(&event);
        listener.key_pressed(&event);
        listener.key_released(&event);
    }
}