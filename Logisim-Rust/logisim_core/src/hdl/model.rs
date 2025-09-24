//! HDL Model
//!
//! Core HDL model interfaces and data structures.
//! This module ports functionality from Java com.cburch.hdl.HdlModel and HdlModelListener.

use crate::data::BitWidth;
use std::collections::HashMap;

/// Port description for HDL entities
/// 
/// Equivalent to Java HdlModel.PortDescription record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortDescription {
    /// Port name
    name: String,
    /// Port type (e.g., "std_logic", "std_logic_vector")
    port_type: String,
    /// Port width in bits
    width_int: i32,
    /// BitWidth representation
    width: BitWidth,
}

impl PortDescription {
    /// Create a new port description
    pub fn new(name: String, port_type: String, width: i32) -> Self {
        Self {
            name,
            port_type,
            width_int: width,
            width: BitWidth::create(if width > 0 { width as u32 } else { 1 }),
        }
    }

    /// Get port name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get port type
    pub fn get_type(&self) -> &str {
        &self.port_type
    }

    /// Get port width as integer
    pub fn get_width_int(&self) -> i32 {
        self.width_int
    }

    /// Get port width as BitWidth
    pub fn get_width(&self) -> BitWidth {
        self.width
    }
}

/// HDL model change event
#[derive(Debug, Clone)]
pub enum HdlModelEvent {
    /// Content has been set/changed
    ContentSet,
    /// Model was cloned
    ModelCloned,
}

/// Listener for HDL model changes
/// 
/// Equivalent to Java HdlModelListener interface
pub trait HdlModelListener: Send + Sync {
    /// Called when the content of the given model has been set
    fn content_set(&mut self, source: &dyn HdlModel);
}

/// Base trait for HDL models
/// 
/// Equivalent to Java HdlModel interface.
/// Represents objects that contain mutable text representing code
/// written in an HDL. Listeners may be attached to handle updates.
pub trait HdlModel: Send + Sync {
    /// Add a listener for changes to the values
    fn add_hdl_model_listener(&mut self, listener: Box<dyn HdlModelListener>);

    /// Compare the model's content with another model
    fn compare_model(&self, other: &dyn HdlModel) -> bool;

    /// Compare the model's content with a string
    fn compare_content(&self, value: &str) -> bool;

    /// Get the content of the HDL-IP component
    fn get_content(&self) -> &str;

    /// Get the component's name
    fn get_name(&self) -> &str;

    /// Get the component's input ports
    fn get_inputs(&self) -> &[PortDescription];

    /// Get the component's output ports
    fn get_outputs(&self) -> &[PortDescription];

    /// Remove a listener for changes to the values
    fn remove_hdl_model_listener(&mut self, listener_id: usize);

    /// Set the content of the component
    /// Returns true if content was successfully set
    fn set_content(&mut self, content: String) -> bool;
}

/// Basic HDL model implementation
/// 
/// Provides a simple implementation of the HdlModel trait
#[derive(Debug, Clone)]
pub struct BasicHdlModel {
    name: String,
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    listeners: Vec<Box<dyn HdlModelListener>>,
    next_listener_id: usize,
    listener_ids: HashMap<usize, usize>, // maps listener_id to index in listeners vec
}

impl BasicHdlModel {
    /// Create a new basic HDL model
    pub fn new(name: String) -> Self {
        Self {
            name,
            content: String::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            listeners: Vec::new(),
            next_listener_id: 0,
            listener_ids: HashMap::new(),
        }
    }

    /// Set input ports
    pub fn set_inputs(&mut self, inputs: Vec<PortDescription>) {
        self.inputs = inputs;
    }

    /// Set output ports
    pub fn set_outputs(&mut self, outputs: Vec<PortDescription>) {
        self.outputs = outputs;
    }

    /// Fire content set event to all listeners
    fn fire_content_set(&mut self) {
        // We need to work around the borrow checker here since we need mutable
        // references to listeners while having an immutable reference to self
        let mut temp_listeners = std::mem::take(&mut self.listeners);
        
        for listener in &mut temp_listeners {
            listener.content_set(self);
        }

        // Remove any listeners that are no longer valid (this would be done
        // with weak references in Java, but we'll handle it differently)
        self.listeners = temp_listeners;
    }
}

impl HdlModel for BasicHdlModel {
    fn add_hdl_model_listener(&mut self, listener: Box<dyn HdlModelListener>) {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        let index = self.listeners.len();
        self.listener_ids.insert(id, index);
        self.listeners.push(listener);
    }

    fn compare_model(&self, other: &dyn HdlModel) -> bool {
        self.content == other.get_content() &&
        self.name == other.get_name() &&
        self.inputs == other.get_inputs() &&
        self.outputs == other.get_outputs()
    }

    fn compare_content(&self, value: &str) -> bool {
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

    fn remove_hdl_model_listener(&mut self, listener_id: usize) {
        if let Some(&index) = self.listener_ids.get(&listener_id) {
            if index < self.listeners.len() {
                self.listeners.remove(index);
                self.listener_ids.remove(&listener_id);
                
                // Update indices for remaining listeners
                for (_, stored_index) in self.listener_ids.iter_mut() {
                    if *stored_index > index {
                        *stored_index -= 1;
                    }
                }
            }
        }
    }

    fn set_content(&mut self, content: String) -> bool {
        if self.content != content {
            self.content = content;
            self.fire_content_set();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestListener {
        content_set_called: bool,
    }

    impl HdlModelListener for TestListener {
        fn content_set(&mut self, _source: &dyn HdlModel) {
            self.content_set_called = true;
        }
    }

    #[test]
    fn test_port_description_creation() {
        let port = PortDescription::new("clk".to_string(), "std_logic".to_string(), 1);
        assert_eq!(port.get_name(), "clk");
        assert_eq!(port.get_type(), "std_logic");
        assert_eq!(port.get_width_int(), 1);
        assert_eq!(port.get_width(), BitWidth::create(1));
    }

    #[test]
    fn test_basic_hdl_model_creation() {
        let model = BasicHdlModel::new("test_entity".to_string());
        assert_eq!(model.get_name(), "test_entity");
        assert_eq!(model.get_content(), "");
        assert!(model.get_inputs().is_empty());
        assert!(model.get_outputs().is_empty());
    }

    #[test]
    fn test_content_setting() {
        let mut model = BasicHdlModel::new("test".to_string());
        assert!(model.set_content("entity test is end;".to_string()));
        assert_eq!(model.get_content(), "entity test is end;");
        
        // Setting same content should return false
        assert!(!model.set_content("entity test is end;".to_string()));
    }

    #[test]
    fn test_port_management() {
        let mut model = BasicHdlModel::new("test".to_string());
        
        let inputs = vec![
            PortDescription::new("clk".to_string(), "std_logic".to_string(), 1),
            PortDescription::new("data".to_string(), "std_logic_vector".to_string(), 8),
        ];
        
        let outputs = vec![
            PortDescription::new("q".to_string(), "std_logic_vector".to_string(), 8),
        ];
        
        model.set_inputs(inputs.clone());
        model.set_outputs(outputs.clone());
        
        assert_eq!(model.get_inputs(), &inputs);
        assert_eq!(model.get_outputs(), &outputs);
    }

    #[test]
    fn test_content_comparison() {
        let model = BasicHdlModel::new("test".to_string());
        assert!(model.compare_content(""));
        
        let mut model2 = model.clone();
        model2.set_content("different content".to_string());
        assert!(!model.compare_model(&model2));
    }
}