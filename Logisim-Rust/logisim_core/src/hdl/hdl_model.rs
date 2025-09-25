/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL model representation and management
//!
//! Provides the core HDL model interface and data structures for representing
//! HDL components with mutable text content, port descriptions, and event handling.

use super::hdl_model_listener::HdlModelListener;
use crate::data::BitWidth;
use std::collections::HashSet;
use std::sync::{Arc, Weak};

/// Port description for HDL components
///
/// Contains information about input/output ports including name, type, and bit width
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDescription {
    /// Name of the port
    name: String,
    /// Type description of the port (e.g., "std_logic", "std_logic_vector")
    port_type: String,
    /// Bit width as integer
    width_int: i32,
    /// Bit width as BitWidth type
    width: BitWidth,
}

impl PortDescription {
    /// Create a new port description
    ///
    /// # Arguments
    /// * `name` - Name of the port
    /// * `port_type` - Type description of the port
    /// * `width` - Bit width as integer
    pub fn new(name: String, port_type: String, width: i32) -> Self {
        Self {
            name,
            port_type,
            width_int: width,
            width: BitWidth::new(width as u32),
        }
    }

    /// Get the port name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the port type
    pub fn get_type(&self) -> &str {
        &self.port_type
    }

    /// Get the bit width as integer
    pub fn get_width_int(&self) -> i32 {
        self.width_int
    }

    /// Get the bit width as BitWidth
    pub fn get_width(&self) -> BitWidth {
        self.width
    }
}

/// Base interface for objects that contain mutable text representing HDL code
///
/// HDL models support listeners for change notifications and provide access to
/// component ports and content. This should not be confused with the HdlModel
/// in the logisim.vhdl.base package.
pub trait HdlModel {
    /// Register a listener for changes to the model values
    fn add_hdl_model_listener(&mut self, listener: Weak<dyn HdlModelListener>);

    /// Compare the model's content with another model
    fn compare_model(&self, other: &dyn HdlModel) -> bool;

    /// Compare the model's content with a string
    fn compare_string(&self, value: &str) -> bool;

    /// Get the content of the HDL component
    fn get_content(&self) -> &str;

    /// Get the component's name
    fn get_name(&self) -> &str;

    /// Get the component's input ports
    fn get_inputs(&self) -> &[PortDescription];

    /// Get the component's output ports
    fn get_outputs(&self) -> &[PortDescription];

    /// Unregister a listener for changes to the model values
    fn remove_hdl_model_listener(&mut self, listener: &Weak<dyn HdlModelListener>);

    /// Set the content of the component
    ///
    /// # Returns
    /// * `true` if the content was successfully set
    /// * `false` if the content could not be set
    fn set_content(&mut self, content: String) -> bool;
}

/// Basic implementation of an HDL model
///
/// Provides a concrete implementation of the HdlModel trait with listener management
/// and content storage.
#[derive(Debug)]
pub struct BasicHdlModel {
    /// Component name
    name: String,
    /// HDL content
    content: String,
    /// Input port descriptions
    inputs: Vec<PortDescription>,
    /// Output port descriptions
    outputs: Vec<PortDescription>,
    /// Registered listeners
    listeners: HashSet<*const dyn HdlModelListener>,
}

impl BasicHdlModel {
    /// Create a new basic HDL model
    ///
    /// # Arguments
    /// * `name` - Name of the HDL component
    /// * `content` - Initial HDL content
    /// * `inputs` - Input port descriptions
    /// * `outputs` - Output port descriptions
    pub fn new(
        name: String,
        content: String,
        inputs: Vec<PortDescription>,
        outputs: Vec<PortDescription>,
    ) -> Self {
        Self {
            name,
            content,
            inputs,
            outputs,
            listeners: HashSet::new(),
        }
    }

    /// Notify all listeners that content has been set
    fn notify_content_set(&self) {
        // In a full implementation, we would iterate through valid listeners
        // For now, this is a placeholder for the notification mechanism
    }
}

impl HdlModel for BasicHdlModel {
    fn add_hdl_model_listener(&mut self, listener: Weak<dyn HdlModelListener>) {
        if let Some(listener_ref) = listener.upgrade() {
            let listener_ptr = Arc::as_ptr(&listener_ref) as *const dyn HdlModelListener;
            self.listeners.insert(listener_ptr);
        }
    }

    fn compare_model(&self, other: &dyn HdlModel) -> bool {
        self.content == other.get_content()
    }

    fn compare_string(&self, value: &str) -> bool {
        self.content == value
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_inputs(&self) -> &[PortDescription] {
        &self.inputs
    }

    fn get_outputs(&self) -> &[PortDescription] {
        &self.outputs
    }

    fn remove_hdl_model_listener(&mut self, listener: &Weak<dyn HdlModelListener>) {
        if let Some(listener_ref) = listener.upgrade() {
            let listener_ptr = Arc::as_ptr(&listener_ref) as *const dyn HdlModelListener;
            self.listeners.remove(&listener_ptr);
        }
    }

    fn set_content(&mut self, content: String) -> bool {
        if self.content != content {
            self.content = content;
            self.notify_content_set();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    struct MockListener {
        notifications: std::sync::Mutex<usize>,
    }

    impl HdlModelListener for MockListener {
        fn content_set(&self, _source: &dyn HdlModel) {
            let mut count = self.notifications.lock().unwrap();
            *count += 1;
        }
    }

    #[test]
    fn test_port_description() {
        let port = PortDescription::new("clock".to_string(), "std_logic".to_string(), 1);

        assert_eq!(port.get_name(), "clock");
        assert_eq!(port.get_type(), "std_logic");
        assert_eq!(port.get_width_int(), 1);
        assert_eq!(port.get_width(), BitWidth::new(1));
    }

    #[test]
    fn test_basic_hdl_model_creation() {
        let inputs = vec![
            PortDescription::new("clk".to_string(), "std_logic".to_string(), 1),
            PortDescription::new("data_in".to_string(), "std_logic_vector".to_string(), 8),
        ];
        let outputs = vec![PortDescription::new(
            "data_out".to_string(),
            "std_logic_vector".to_string(),
            8,
        )];

        let model = BasicHdlModel::new(
            "test_component".to_string(),
            "-- VHDL code here".to_string(),
            inputs,
            outputs,
        );

        assert_eq!(model.get_name(), "test_component");
        assert_eq!(model.get_content(), "-- VHDL code here");
        assert_eq!(model.get_inputs().len(), 2);
        assert_eq!(model.get_outputs().len(), 1);
    }

    #[test]
    fn test_hdl_model_content_comparison() {
        let model = BasicHdlModel::new("test".to_string(), "content".to_string(), vec![], vec![]);

        assert!(model.compare_string("content"));
        assert!(!model.compare_string("different"));
    }

    #[test]
    fn test_hdl_model_set_content() {
        let mut model =
            BasicHdlModel::new("test".to_string(), "original".to_string(), vec![], vec![]);

        assert!(model.set_content("new content".to_string()));
        assert_eq!(model.get_content(), "new content");

        // Setting same content should return false
        assert!(!model.set_content("new content".to_string()));
    }

    #[test]
    fn test_hdl_model_listener_management() {
        let mut model = BasicHdlModel::new(
            "test".to_string(),
            "content".to_string(),
            vec![],
            vec![],
        );

        let listener = Arc::new(MockListener {
            notifications: std::sync::Mutex::new(0),
        });
        
        // Create trait object Arc first, then downgrade
        let trait_obj: Arc<dyn HdlModelListener> = listener;
        let weak_listener = Arc::downgrade(&trait_obj);

        model.add_hdl_model_listener(weak_listener.clone());
        model.remove_hdl_model_listener(&weak_listener);
        // Test passes if no panics occur
    }

    #[test]
    fn test_port_description_equality() {
        let port1 = PortDescription::new("test".to_string(), "std_logic".to_string(), 1);
        let port2 = PortDescription::new("test".to_string(), "std_logic".to_string(), 1);
        let port3 = PortDescription::new("different".to_string(), "std_logic".to_string(), 1);

        assert_eq!(port1, port2);
        assert_ne!(port1, port3);
    }
}
