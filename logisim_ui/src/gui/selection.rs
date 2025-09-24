//! Selection management - equivalent to the Java Selection class

use logisim_core::{ComponentId, NetId};
use std::collections::HashSet;

/// Manages selection of components and wires in the schematic
#[derive(Debug, Clone, Default)]
pub struct Selection {
    /// Selected components
    selected_components: HashSet<ComponentId>,

    /// Selected nets/wires
    selected_nets: HashSet<NetId>,
}

impl Selection {
    /// Create a new empty selection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a component to the selection
    pub fn add_component(&mut self, component_id: ComponentId) {
        self.selected_components.insert(component_id);
    }

    /// Remove a component from the selection
    pub fn remove_component(&mut self, component_id: &ComponentId) {
        self.selected_components.remove(component_id);
    }

    /// Add a net to the selection
    pub fn add_net(&mut self, net_id: NetId) {
        self.selected_nets.insert(net_id);
    }

    /// Remove a net from the selection
    pub fn remove_net(&mut self, net_id: &NetId) {
        self.selected_nets.remove(net_id);
    }

    /// Clear all selections
    pub fn clear(&mut self) {
        self.selected_components.clear();
        self.selected_nets.clear();
    }

    /// Check if a component is selected
    pub fn is_component_selected(&self, component_id: &ComponentId) -> bool {
        self.selected_components.contains(component_id)
    }

    /// Check if a net is selected
    pub fn is_net_selected(&self, net_id: &NetId) -> bool {
        self.selected_nets.contains(net_id)
    }

    /// Get all selected components
    pub fn selected_components(&self) -> &HashSet<ComponentId> {
        &self.selected_components
    }

    /// Get all selected nets
    pub fn selected_nets(&self) -> &HashSet<NetId> {
        &self.selected_nets
    }

    /// Check if selection is empty
    pub fn is_empty(&self) -> bool {
        self.selected_components.is_empty() && self.selected_nets.is_empty()
    }

    /// Get the total number of selected items
    pub fn count(&self) -> usize {
        self.selected_components.len() + self.selected_nets.len()
    }
}
