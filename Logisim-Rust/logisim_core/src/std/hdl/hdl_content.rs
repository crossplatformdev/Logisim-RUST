//! HDL Content
//!
//! Base trait for HDL content management.
//! This module provides trait definitions for HDL content components.

use crate::hdl::{HdlModel, HdlModelListener, PortDescription};
use std::collections::HashMap;

/// Trait for HDL content components
pub trait HdlContent {
    /// Get the HDL content as string
    fn get_content(&self) -> &str;

    /// Set the HDL content
    fn set_content(&mut self, content: String);

    /// Get the name of the component
    fn get_name(&self) -> &str;

    /// Check if the content is valid
    fn is_valid(&self) -> bool;
}

/// HDL content structure for managing HDL components
#[derive(Clone)]
pub struct BasicHdlContent {
    name: String,
    content: String,
    inputs: Vec<PortDescription>,
    outputs: Vec<PortDescription>,
    listeners: Vec<Box<dyn HdlModelListener + Send + Sync>>,
    next_listener_id: u32,
    listener_ids: HashMap<u32, usize>,
}

impl std::fmt::Debug for BasicHdlContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicHdlContent")
            .field("name", &self.name)
            .field("content", &self.content)
            .field("inputs", &self.inputs)
            .field("outputs", &self.outputs)
            .field("listener_count", &self.listeners.len())
            .finish()
    }
}

impl BasicHdlContent {
    /// Create new HDL content
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

    /// Get the HDL content as string
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Set the HDL content
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    /// Get the name of the component
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Check if the content is valid
    pub fn is_valid(&self) -> bool {
        !self.content.is_empty()
    }

    /// Concatenate two arrays (equivalent to Java HdlContent.concat)
    pub fn concat<T: Clone>(first: &[T], second: &[T]) -> Vec<T> {
        let mut result = Vec::with_capacity(first.len() + second.len());
        result.extend_from_slice(first);
        result.extend_from_slice(second);
        result
    }
}

impl HdlContent for BasicHdlContent {
    fn get_content(&self) -> &str {
        &self.content
    }

    fn set_content(&mut self, content: String) {
        self.content = content;
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_valid(&self) -> bool {
        !self.content.is_empty()
    }
}

impl BasicHdlContent {
    /// Set inputs
    pub fn set_inputs(&mut self, inputs: Vec<PortDescription>) {
        self.inputs = inputs;
    }

    /// Set outputs  
    pub fn set_outputs(&mut self, outputs: Vec<PortDescription>) {
        self.outputs = outputs;
    }

    /// Fire content set event to all listeners
    pub fn fire_content_set(&mut self) {
        // Similar to the BasicHdlModel implementation
        let mut temp_listeners = std::mem::take(&mut self.listeners);
        
        for listener in &mut temp_listeners {
            listener.content_set(self);
        }

        self.listeners = temp_listeners;
    }

    /// Get all ports (inputs and outputs combined)
    pub fn get_all_ports(&self) -> Vec<PortDescription> {
        Self::concat(&self.inputs, &self.outputs)
    }
}

impl HdlModel for HdlContent {
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

/// HDL content attribute wrapper
/// 
/// Provides attribute-specific functionality for HDL content.
/// This would be used by component attribute systems.
#[derive(Debug)]
pub struct HdlContentAttribute {
    content: HdlContent,
    attribute_name: String,
}

impl HdlContentAttribute {
    /// Create new HDL content attribute
    pub fn new(attribute_name: String, entity_name: String) -> Self {
        Self {
            content: HdlContent::new(entity_name),
            attribute_name,
        }
    }

    /// Get the attribute name
    pub fn get_attribute_name(&self) -> &str {
        &self.attribute_name
    }

    /// Get reference to the content
    pub fn get_content(&self) -> &HdlContent {
        &self.content
    }

    /// Get mutable reference to the content
    pub fn get_content_mut(&mut self) -> &mut HdlContent {
        &mut self.content
    }
}

/// HDL content editor interface
/// 
/// Represents an editor for HDL content, equivalent to Java HdlContentEditor.
/// This is a trait that HDL editors should implement.
pub trait HdlContentEditor {
    /// Get the current text content
    fn get_text(&self) -> &str;

    /// Set the text content
    fn set_text(&mut self, text: String);

    /// Check if the content has been modified
    fn is_modified(&self) -> bool;

    /// Mark content as saved (clear modified flag)
    fn mark_saved(&mut self);

    /// Get the associated HDL model
    fn get_model(&self) -> Option<&dyn HdlModel>;

    /// Set the associated HDL model
    fn set_model(&mut self, model: Box<dyn HdlModel>);
}

/// Basic HDL content editor implementation
pub struct BasicHdlContentEditor {
    text: String,
    original_text: String,
    model: Option<Box<dyn HdlModel>>,
}

impl BasicHdlContentEditor {
    /// Create a new basic HDL content editor
    pub fn new() -> Self {
        Self {
            text: String::new(),
            original_text: String::new(),
            model: None,
        }
    }

    /// Create editor with initial text
    pub fn with_text(text: String) -> Self {
        Self {
            original_text: text.clone(),
            text,
            model: None,
        }
    }
}

impl Default for BasicHdlContentEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl HdlContentEditor for BasicHdlContentEditor {
    fn get_text(&self) -> &str {
        &self.text
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
    }

    fn is_modified(&self) -> bool {
        self.text != self.original_text
    }

    fn mark_saved(&mut self) {
        self.original_text = self.text.clone();
    }

    fn get_model(&self) -> Option<&dyn HdlModel> {
        self.model.as_ref().map(|m| m.as_ref())
    }

    fn set_model(&mut self, model: Box<dyn HdlModel>) {
        self.model = Some(model);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdl_content_creation() {
        let content = HdlContent::new("test_entity".to_string());
        assert_eq!(content.get_name(), "test_entity");
        assert_eq!(content.get_content(), "");
        assert!(content.get_inputs().is_empty());
        assert!(content.get_outputs().is_empty());
    }

    #[test]
    fn test_concat_function() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let result = HdlContent::concat(&first, &second);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_hdl_content_attribute() {
        let attr = HdlContentAttribute::new("content".to_string(), "entity".to_string());
        assert_eq!(attr.get_attribute_name(), "content");
        assert_eq!(attr.get_content().get_name(), "entity");
    }

    #[test]
    fn test_basic_hdl_content_editor() {
        let mut editor = BasicHdlContentEditor::new();
        assert_eq!(editor.get_text(), "");
        assert!(!editor.is_modified());

        editor.set_text("entity test is end;".to_string());
        assert_eq!(editor.get_text(), "entity test is end;");
        assert!(editor.is_modified());

        editor.mark_saved();
        assert!(!editor.is_modified());
    }

    #[test]
    fn test_editor_with_initial_text() {
        let editor = BasicHdlContentEditor::with_text("initial content".to_string());
        assert_eq!(editor.get_text(), "initial content");
        assert!(!editor.is_modified());
    }

    #[test]
    fn test_get_all_ports() {
        let mut content = HdlContent::new("test".to_string());
        
        let inputs = vec![
            PortDescription::new("in1".to_string(), "std_logic".to_string(), 1),
        ];
        let outputs = vec![
            PortDescription::new("out1".to_string(), "std_logic".to_string(), 1),
        ];
        
        content.set_inputs(inputs.clone());
        content.set_outputs(outputs.clone());
        
        let all_ports = content.get_all_ports();
        assert_eq!(all_ports.len(), 2);
        assert_eq!(all_ports[0].get_name(), "in1");
        assert_eq!(all_ports[1].get_name(), "out1");
    }
}