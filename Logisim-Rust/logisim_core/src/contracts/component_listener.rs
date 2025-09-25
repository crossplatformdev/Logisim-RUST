/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Component event listener contract

/// Component event data
#[derive(Debug, Clone)]
pub struct ComponentEvent {
    pub component_id: u32,
    pub event_type: ComponentEventType,
}

#[derive(Debug, Clone)]
pub enum ComponentEventType {
    Resized { width: u32, height: u32 },
    Moved { x: i32, y: i32 },
    Shown,
    Hidden,
}

/// Base contract for component listeners with default no-op implementations
pub trait BaseComponentListenerContract {
    /// Called when the component is resized
    fn component_resized(&mut self, _event: &ComponentEvent) {
        // no-op implementation
    }

    /// Called when the component is moved
    fn component_moved(&mut self, _event: &ComponentEvent) {
        // no-op implementation
    }

    /// Called when the component is shown
    fn component_shown(&mut self, _event: &ComponentEvent) {
        // no-op implementation
    }

    /// Called when the component is hidden
    fn component_hidden(&mut self, _event: &ComponentEvent) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener;

    impl BaseComponentListenerContract for TestListener {}

    #[test]
    fn test_default_implementations() {
        let mut listener = TestListener;
        let event = ComponentEvent {
            component_id: 1,
            event_type: ComponentEventType::Resized {
                width: 100,
                height: 200,
            },
        };

        // Should not panic - all methods have default implementations
        listener.component_resized(&event);
        listener.component_moved(&event);
        listener.component_shown(&event);
        listener.component_hidden(&event);
    }
}