/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Contract interfaces for event handling
//!
//! This module provides default trait implementations for various UI event handlers,
//! allowing implementors to only override the methods they need rather than
//! implementing all methods of the parent trait.

pub mod component_listener;
pub mod document_listener;
pub mod key_listener;
pub mod layout_manager;
pub mod list_data_listener;
pub mod mouse_input_listener;
pub mod mouse_listener;
pub mod mouse_motion_listener;
pub mod window_focus_listener;
pub mod window_listener;

// Re-export all contract traits
pub use component_listener::*;
pub use document_listener::*;
pub use key_listener::*;
pub use layout_manager::*;
pub use list_data_listener::*;
pub use mouse_input_listener::*;
pub use mouse_listener::*;
pub use mouse_motion_listener::*;
pub use window_focus_listener::*;
pub use window_listener::*;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Comprehensive test that demonstrates all contract traits work together
    /// and provide default implementations that don't panic
    #[test]
    fn test_all_contracts_integration() {
        // Create a comprehensive test component that implements all contracts
        struct TestComponent;

        impl BaseComponentListenerContract for TestComponent {}
        impl BaseDocumentListenerContract for TestComponent {}
        impl BaseKeyListenerContract for TestComponent {}
        impl BaseListDataListenerContract for TestComponent {}
        impl BaseMouseInputListenerContract for TestComponent {}
        impl BaseMouseMotionListenerContract for TestComponent {}
        impl BaseWindowFocusListenerContract for TestComponent {}
        impl BaseWindowListenerContract for TestComponent {}

        // Test component that implements the layout manager contract
        struct TestLayoutManager;
        impl BaseLayoutManagerContract for TestLayoutManager {
            fn preferred_layout_size(&self, _container: &Container) -> Dimension {
                Dimension::new(100, 100)
            }

            fn minimum_layout_size(&self, _container: &Container) -> Dimension {
                Dimension::new(50, 50)
            }
        }

        // Test mouse listener that requires implementation
        struct TestMouseListener;
        impl BaseMouseListenerContract for TestMouseListener {
            fn mouse_clicked(&mut self, _mouse_event: &MouseEvent) {
                // Required implementation
            }
        }

        let mut component = TestComponent;
        let mut layout_manager = TestLayoutManager;
        let mut mouse_listener = TestMouseListener;

        // Test all component events
        let comp_event = ComponentEvent {
            component_id: 1,
            event_type: ComponentEventType::Resized { width: 100, height: 200 },
        };
        component.component_resized(&comp_event);
        component.component_moved(&comp_event);
        component.component_shown(&comp_event);
        component.component_hidden(&comp_event);

        // Test document events
        let doc_event = DocumentEvent {
            document_id: 1,
            event_type: DocumentEventType::Insert,
            offset: 0,
            length: 5,
        };
        component.insert_update(&doc_event);
        component.remove_update(&doc_event);
        component.changed_update(&doc_event);

        // Test key events
        let key_event = KeyEvent {
            key_code: 65,
            key_char: Some('A'),
            modifiers: KeyModifiers {
                shift: false,
                ctrl: false,
                alt: false,
                meta: false,
            },
            event_type: KeyEventType::Typed,
        };
        component.key_typed(&key_event);
        component.key_pressed(&key_event);
        component.key_released(&key_event);

        // Test list data events
        let list_event = ListDataEvent {
            source_id: 1,
            event_type: ListDataEventType::IntervalAdded,
            index0: 0,
            index1: 2,
        };
        component.interval_added(&list_event);
        component.interval_removed(&list_event);
        component.contents_changed(&list_event);

        // Test mouse events
        let mouse_event = MouseEvent {
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

        // Test mouse input listener (explicit calls to avoid ambiguity)
        BaseMouseInputListenerContract::mouse_clicked(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_pressed(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_released(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_entered(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_exited(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_dragged(&mut component, &mouse_event);
        BaseMouseInputListenerContract::mouse_moved(&mut component, &mouse_event);

        // Test mouse motion listener (explicit calls to avoid ambiguity)
        BaseMouseMotionListenerContract::mouse_dragged(&mut component, &mouse_event);
        BaseMouseMotionListenerContract::mouse_moved(&mut component, &mouse_event);

        // Test mouse listener
        mouse_listener.mouse_clicked(&mouse_event);
        mouse_listener.mouse_pressed(&mouse_event);
        mouse_listener.mouse_released(&mouse_event);
        mouse_listener.mouse_entered(&mouse_event);
        mouse_listener.mouse_exited(&mouse_event);

        // Test window events
        let window_event = WindowEvent {
            window_id: 1,
            event_type: WindowEventType::FocusGained,
        };

        // Test window focus listener
        component.window_gained_focus(&window_event);
        component.window_lost_focus(&window_event);

        // Test window listener
        component.window_opened(&window_event);
        component.window_closing(&window_event);
        component.window_closed(&window_event);
        component.window_iconified(&window_event);
        component.window_deiconified(&window_event);
        component.window_activated(&window_event);
        component.window_deactivated(&window_event);

        // Test layout manager
        let test_component = Component {
            id: 1,
            name: "test".to_string(),
            bounds: (0, 0, 10, 10),
        };
        let container = Container {
            id: 1,
            components: vec![test_component.clone()],
            bounds: (0, 0, 100, 100),
        };

        layout_manager.add_layout_component("center", &test_component);
        layout_manager.remove_layout_component(&test_component);
        let preferred = layout_manager.preferred_layout_size(&container);
        let minimum = layout_manager.minimum_layout_size(&container);
        assert_eq!(preferred, Dimension::new(100, 100));
        assert_eq!(minimum, Dimension::new(50, 50));

        // If we reach here, all contracts work correctly with default implementations
        assert!(true, "All contract traits work correctly");
    }
}