//! Netlist representation and wire/node management.
//!
//! This module defines the data structures for representing the netlist
//! (the network of connected components), including nodes, nets, and
//! the connections between components.

use crate::component::ComponentId;
use crate::signal::{BusWidth, Signal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// Unique identifier for a node (connection point)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    /// Create a new node ID
    pub fn new(id: u64) -> Self {
        NodeId(id)
    }

    /// Get the ID as u64
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for NodeId {
    fn from(id: u64) -> Self {
        NodeId(id)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "N{}", self.0)
    }
}

/// Unique identifier for a net (wire)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NetId(pub u64);

impl NetId {
    /// Create a new net ID
    pub fn new(id: u64) -> Self {
        NetId(id)
    }

    /// Get the ID as u64
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for NetId {
    fn from(id: u64) -> Self {
        NetId(id)
    }
}

impl fmt::Display for NetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Net{}", self.0)
    }
}

/// A connection point in the netlist
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier for this node
    pub id: NodeId,
    /// Current signal at this node
    pub signal: Signal,
    /// Bus width of this node
    pub width: BusWidth,
    /// Components connected to this node
    pub connected_components: HashSet<ComponentId>,
    /// Name for debugging/display
    pub name: Option<String>,
}

impl Node {
    /// Create a new node
    pub fn new(id: NodeId, width: BusWidth) -> Self {
        Node {
            id,
            signal: Signal::unknown(width),
            width,
            connected_components: HashSet::new(),
            name: None,
        }
    }

    /// Create a new named node
    pub fn new_named(id: NodeId, width: BusWidth, name: String) -> Self {
        Node {
            id,
            signal: Signal::unknown(width),
            width,
            connected_components: HashSet::new(),
            name: Some(name),
        }
    }

    /// Connect a component to this node
    pub fn connect_component(&mut self, component_id: ComponentId) {
        self.connected_components.insert(component_id);
    }

    /// Disconnect a component from this node
    pub fn disconnect_component(&mut self, component_id: ComponentId) -> bool {
        self.connected_components.remove(&component_id)
    }

    /// Update the signal at this node
    pub fn set_signal(&mut self, signal: Signal) -> Result<(), &'static str> {
        if signal.width() != self.width {
            return Err("Signal width mismatch");
        }
        self.signal = signal;
        Ok(())
    }

    /// Get the current signal
    pub fn get_signal(&self) -> &Signal {
        &self.signal
    }

    /// Get all connected components
    pub fn get_connected_components(&self) -> &HashSet<ComponentId> {
        &self.connected_components
    }
}

/// A net (wire) connecting multiple nodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Net {
    /// Unique identifier for this net
    pub id: NetId,
    /// Nodes connected by this net
    pub nodes: HashSet<NodeId>,
    /// Bus width of this net
    pub width: BusWidth,
    /// Name for debugging/display
    pub name: Option<String>,
}

impl Net {
    /// Create a new net
    pub fn new(id: NetId, width: BusWidth) -> Self {
        Net {
            id,
            nodes: HashSet::new(),
            width,
            name: None,
        }
    }

    /// Create a new named net
    pub fn new_named(id: NetId, width: BusWidth, name: String) -> Self {
        Net {
            id,
            nodes: HashSet::new(),
            width,
            name: Some(name),
        }
    }

    /// Add a node to this net
    pub fn add_node(&mut self, node_id: NodeId) {
        self.nodes.insert(node_id);
    }

    /// Remove a node from this net
    pub fn remove_node(&mut self, node_id: NodeId) -> bool {
        self.nodes.remove(&node_id)
    }

    /// Get all nodes in this net
    pub fn get_nodes(&self) -> &HashSet<NodeId> {
        &self.nodes
    }

    /// Check if this net is empty (no nodes)
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

/// Connection between a component pin and a node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Connection {
    /// Component that owns the pin
    pub component_id: ComponentId,
    /// Name of the pin on the component
    pub pin_name: String,
    /// Node that the pin is connected to
    pub node_id: NodeId,
}

impl Connection {
    /// Create a new connection
    pub fn new(component_id: ComponentId, pin_name: String, node_id: NodeId) -> Self {
        Connection {
            component_id,
            pin_name,
            node_id,
        }
    }
}

/// The complete netlist representing the circuit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Netlist {
    /// All nodes in the netlist
    nodes: HashMap<NodeId, Node>,
    /// All nets in the netlist
    nets: HashMap<NetId, Net>,
    /// All connections between components and nodes
    connections: Vec<Connection>,
    /// Next node ID to assign
    next_node_id: u64,
    /// Next net ID to assign
    next_net_id: u64,
}

impl Netlist {
    /// Create a new empty netlist
    pub fn new() -> Self {
        Netlist {
            nodes: HashMap::new(),
            nets: HashMap::new(),
            connections: Vec::new(),
            next_node_id: 1,
            next_net_id: 1,
        }
    }

    /// Create a new node
    pub fn create_node(&mut self, width: BusWidth) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = Node::new(id, width);
        self.nodes.insert(id, node);
        id
    }

    /// Create a new named node
    pub fn create_named_node(&mut self, width: BusWidth, name: String) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;

        let node = Node::new_named(id, width, name);
        self.nodes.insert(id, node);
        id
    }

    /// Create a new net
    pub fn create_net(&mut self, width: BusWidth) -> NetId {
        let id = NetId(self.next_net_id);
        self.next_net_id += 1;

        let net = Net::new(id, width);
        self.nets.insert(id, net);
        id
    }

    /// Create a new named net
    pub fn create_named_net(&mut self, width: BusWidth, name: String) -> NetId {
        let id = NetId(self.next_net_id);
        self.next_net_id += 1;

        let net = Net::new_named(id, width, name);
        self.nets.insert(id, net);
        id
    }

    /// Get a node by ID
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Get a node by ID (mutable)
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Get a net by ID
    pub fn get_net(&self, id: NetId) -> Option<&Net> {
        self.nets.get(&id)
    }

    /// Get a net by ID (mutable)
    pub fn get_net_mut(&mut self, id: NetId) -> Option<&mut Net> {
        self.nets.get_mut(&id)
    }

    /// Connect a component pin to a node
    pub fn connect(
        &mut self,
        component_id: ComponentId,
        pin_name: String,
        node_id: NodeId,
    ) -> Result<(), &'static str> {
        // Check if node exists
        if !self.nodes.contains_key(&node_id) {
            return Err("Node does not exist");
        }

        // Add the connection
        let connection = Connection::new(component_id, pin_name, node_id);
        self.connections.push(connection);

        // Update the node's connected components
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.connect_component(component_id);
        }

        Ok(())
    }

    /// Disconnect a component pin from a node
    pub fn disconnect(
        &mut self,
        component_id: ComponentId,
        pin_name: &str,
    ) -> Result<(), &'static str> {
        // Find and remove the connection
        let mut found_node_id = None;
        self.connections.retain(|conn| {
            if conn.component_id == component_id && conn.pin_name == pin_name {
                found_node_id = Some(conn.node_id);
                false
            } else {
                true
            }
        });

        // Update the node's connected components
        if let Some(node_id) = found_node_id {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.disconnect_component(component_id);
            }
            Ok(())
        } else {
            Err("Connection not found")
        }
    }

    /// Get all connections for a component
    pub fn get_component_connections(&self, component_id: ComponentId) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|conn| conn.component_id == component_id)
            .collect()
    }

    /// Get the node connected to a specific component pin
    pub fn get_pin_node(&self, component_id: ComponentId, pin_name: &str) -> Option<NodeId> {
        self.connections
            .iter()
            .find(|conn| conn.component_id == component_id && conn.pin_name == pin_name)
            .map(|conn| conn.node_id)
    }

    /// Update signal at a node
    pub fn set_node_signal(&mut self, node_id: NodeId, signal: Signal) -> Result<(), &'static str> {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.set_signal(signal)
        } else {
            Err("Node not found")
        }
    }

    /// Get signal at a node
    pub fn get_node_signal(&self, node_id: NodeId) -> Option<&Signal> {
        self.nodes.get(&node_id).map(|node| &node.signal)
    }

    /// Get all nodes
    pub fn get_all_nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Get all nets
    pub fn get_all_nets(&self) -> &HashMap<NetId, Net> {
        &self.nets
    }

    /// Get all connections
    pub fn get_all_connections(&self) -> &[Connection] {
        &self.connections
    }

    /// Get all components that should be notified when a node's signal changes
    pub fn get_affected_components(&self, node_id: NodeId) -> Vec<ComponentId> {
        if let Some(node) = self.nodes.get(&node_id) {
            node.connected_components.iter().copied().collect()
        } else {
            Vec::new()
        }
    }

    /// Clear all nodes, nets, and connections
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.nets.clear();
        self.connections.clear();
        self.next_node_id = 1;
        self.next_net_id = 1;
    }

    /// Get statistics about the netlist
    pub fn stats(&self) -> NetlistStats {
        NetlistStats {
            node_count: self.nodes.len(),
            net_count: self.nets.len(),
            connection_count: self.connections.len(),
        }
    }
}

impl Default for Netlist {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about a netlist
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistStats {
    pub node_count: usize,
    pub net_count: usize,
    pub connection_count: usize,
}

impl fmt::Display for NetlistStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Netlist: {} nodes, {} nets, {} connections",
            self.node_count, self.net_count, self.connection_count
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_node(BusWidth(1));

        assert_eq!(node_id, NodeId(1));
        let node = netlist.get_node(node_id).unwrap();
        assert_eq!(node.id, node_id);
        assert_eq!(node.width, BusWidth(1));
        assert!(node.connected_components.is_empty());
    }

    #[test]
    fn test_named_node_creation() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_named_node(BusWidth(8), "DataBus".to_string());

        let node = netlist.get_node(node_id).unwrap();
        assert_eq!(node.name, Some("DataBus".to_string()));
        assert_eq!(node.width, BusWidth(8));
    }

    #[test]
    fn test_connection() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_node(BusWidth(1));
        let component_id = ComponentId(42);

        // Connect component pin to node
        let result = netlist.connect(component_id, "OUT".to_string(), node_id);
        assert!(result.is_ok());

        // Check connection was created
        let connections = netlist.get_component_connections(component_id);
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].pin_name, "OUT");
        assert_eq!(connections[0].node_id, node_id);

        // Check node was updated
        let node = netlist.get_node(node_id).unwrap();
        assert!(node.connected_components.contains(&component_id));
    }

    #[test]
    fn test_disconnection() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_node(BusWidth(1));
        let component_id = ComponentId(42);

        // Connect then disconnect
        netlist
            .connect(component_id, "OUT".to_string(), node_id)
            .unwrap();
        let result = netlist.disconnect(component_id, "OUT");
        assert!(result.is_ok());

        // Check connection was removed
        let connections = netlist.get_component_connections(component_id);
        assert!(connections.is_empty());

        // Check node was updated
        let node = netlist.get_node(node_id).unwrap();
        assert!(node.connected_components.is_empty());
    }

    #[test]
    fn test_pin_node_lookup() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_node(BusWidth(1));
        let component_id = ComponentId(42);

        netlist
            .connect(component_id, "OUT".to_string(), node_id)
            .unwrap();

        let found_node = netlist.get_pin_node(component_id, "OUT");
        assert_eq!(found_node, Some(node_id));

        let not_found = netlist.get_pin_node(component_id, "IN");
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_signal_propagation_preparation() {
        let mut netlist = Netlist::new();
        let node_id = netlist.create_node(BusWidth(1));
        let comp1 = ComponentId(1);
        let comp2 = ComponentId(2);

        // Connect multiple components to the same node
        netlist.connect(comp1, "OUT".to_string(), node_id).unwrap();
        netlist.connect(comp2, "IN".to_string(), node_id).unwrap();

        // Get affected components
        let affected = netlist.get_affected_components(node_id);
        assert_eq!(affected.len(), 2);
        assert!(affected.contains(&comp1));
        assert!(affected.contains(&comp2));
    }

    #[test]
    fn test_netlist_stats() {
        let mut netlist = Netlist::new();

        let node1 = netlist.create_node(BusWidth(1));
        let node2 = netlist.create_node(BusWidth(8));
        let _net = netlist.create_net(BusWidth(1));

        netlist
            .connect(ComponentId(1), "A".to_string(), node1)
            .unwrap();
        netlist
            .connect(ComponentId(2), "B".to_string(), node2)
            .unwrap();

        let stats = netlist.stats();
        assert_eq!(stats.node_count, 2);
        assert_eq!(stats.net_count, 1);
        assert_eq!(stats.connection_count, 2);
    }
}
