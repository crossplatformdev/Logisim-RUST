//! Component traits and types for the simulation.
//!
//! This module defines the interfaces that digital logic components must implement
//! to participate in the simulation, including I/O pins and signal propagation.

use crate::signal::{BusWidth, Signal, Timestamp, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::ops::Not;

/// Unique identifier for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ComponentId(pub u64);

impl ComponentId {
    /// Create a new component ID
    pub fn new(id: u64) -> Self {
        ComponentId(id)
    }

    /// Get the ID as u64
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl From<u64> for ComponentId {
    fn from(id: u64) -> Self {
        ComponentId(id)
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "C{}", self.0)
    }
}

/// Direction of a pin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PinDirection {
    /// Input pin
    Input,
    /// Output pin
    Output,
    /// Bidirectional pin
    InOut,
}

/// Represents a connection point on a component
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    /// Name of this pin
    pub name: String,
    /// Direction of this pin
    pub direction: PinDirection,
    /// Bus width of this pin
    pub width: BusWidth,
    /// Current signal on this pin
    pub signal: Signal,
}

impl Pin {
    /// Create a new input pin
    pub fn new_input(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::Input,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Create a new output pin
    pub fn new_output(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::Output,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Create a new bidirectional pin
    pub fn new_inout(name: impl Into<String>, width: BusWidth) -> Self {
        Pin {
            name: name.into(),
            direction: PinDirection::InOut,
            width,
            signal: Signal::unknown(width),
        }
    }

    /// Check if this is an input pin
    pub fn is_input(&self) -> bool {
        matches!(self.direction, PinDirection::Input | PinDirection::InOut)
    }

    /// Check if this is an output pin
    pub fn is_output(&self) -> bool {
        matches!(self.direction, PinDirection::Output | PinDirection::InOut)
    }

    /// Update the signal on this pin
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
}

/// Result of a component update
#[derive(Debug, Clone)]
pub struct UpdateResult {
    /// New output signals to propagate
    pub outputs: HashMap<String, Signal>,
    /// Propagation delay for these outputs
    pub delay: u64,
    /// Whether the component state changed
    pub state_changed: bool,
}

impl UpdateResult {
    /// Create a new update result with no outputs
    pub fn new() -> Self {
        UpdateResult {
            outputs: HashMap::new(),
            delay: 0,
            state_changed: false,
        }
    }

    /// Create an update result with outputs
    pub fn with_outputs(outputs: HashMap<String, Signal>, delay: u64) -> Self {
        UpdateResult {
            outputs,
            delay,
            state_changed: true,
        }
    }

    /// Add an output signal
    pub fn add_output(&mut self, pin_name: String, signal: Signal) {
        self.outputs.insert(pin_name, signal);
        self.state_changed = true;
    }

    /// Set the propagation delay
    pub fn set_delay(&mut self, delay: u64) {
        self.delay = delay;
    }
}

impl Default for UpdateResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait that all simulation components must implement
pub trait Component: std::fmt::Debug {
    /// Get the unique identifier for this component
    fn id(&self) -> ComponentId;

    /// Get the name/type of this component
    fn name(&self) -> &str;

    /// Get all pins on this component
    fn pins(&self) -> &HashMap<String, Pin>;

    /// Get all pins on this component (mutable)
    fn pins_mut(&mut self) -> &mut HashMap<String, Pin>;

    /// Get a specific pin by name
    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins().get(name)
    }

    /// Get a specific pin by name (mutable)
    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins_mut().get_mut(name)
    }

    /// Update the component's outputs based on current inputs
    /// This is called when input signals change
    fn update(&mut self, current_time: Timestamp) -> UpdateResult;

    /// Reset the component to its initial state
    fn reset(&mut self);

    /// Get the typical propagation delay for this component
    fn propagation_delay(&self) -> u64 {
        1 // Default 1 time unit
    }

    /// Check if this component has sequential behavior (memory)
    fn is_sequential(&self) -> bool {
        false // Most components are combinational
    }

    /// Handle a clock edge (for sequential components)
    fn clock_edge(&mut self, _edge: ClockEdge, _current_time: Timestamp) -> UpdateResult {
        UpdateResult::new() // Default: no response to clock
    }
}

/// Clock edge types for sequential components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockEdge {
    /// Rising edge (low to high)
    Rising,
    /// Falling edge (high to low)
    Falling,
}

/// Trait for components that can propagate signals
pub trait Propagator {
    /// Propagate a signal change through this component
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult;

    /// Get all components that should be notified when this component's outputs change
    fn get_dependent_components(&self) -> Vec<ComponentId> {
        Vec::new() // Default: no dependencies
    }
}

/// A basic AND gate implementation for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl AndGate {
    /// Create a new 2-input AND gate
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        AndGate { id, pins }
    }
}

impl Component for AndGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "AND"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.and(b);
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        // Update internal pin state
        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for AND gate
    }
}

impl Propagator for AndGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// A basic clocked latch implementation for testing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockedLatch {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    stored_value: Value,
}

impl ClockedLatch {
    /// Create a new clocked latch
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("D".to_string(), Pin::new_input("D", BusWidth(1)));
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1)));
        pins.insert("Q".to_string(), Pin::new_output("Q", BusWidth(1)));

        ClockedLatch {
            id,
            pins,
            stored_value: Value::Unknown,
        }
    }
}

impl Component for ClockedLatch {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "LATCH"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // For a latch, output always reflects stored value
        let output_signal = Signal::new_single(self.stored_value);

        let mut result = UpdateResult::new();
        result.add_output("Q".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        // Update internal pin state
        if let Some(pin) = self.pins.get_mut("Q") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        self.stored_value = Value::Unknown;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn is_sequential(&self) -> bool {
        true
    }

    fn clock_edge(&mut self, edge: ClockEdge, current_time: Timestamp) -> UpdateResult {
        if edge == ClockEdge::Rising {
            // On rising edge, capture the D input
            let d_value = self
                .pins
                .get("D")
                .unwrap()
                .signal
                .as_single()
                .unwrap_or(Value::Unknown);

            self.stored_value = d_value;
            self.update(current_time)
        } else {
            UpdateResult::new()
        }
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for latch
    }
}

impl Propagator for ClockedLatch {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal.clone());
        }

        // For clock input, check for edges
        if input_pin == "CLK" {
            if let Some(new_value) = signal.as_single() {
                // Detect clock edge (simplified - in real implementation would track previous value)
                if new_value == Value::High {
                    return self.clock_edge(ClockEdge::Rising, current_time);
                }
            }
        }

        UpdateResult::new()
    }
}

/// A Pin component for input/output connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinComponent {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    direction: PinDirection,
    width: BusWidth,
}

impl PinComponent {
    /// Create a new input pin
    pub fn new_input(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("".to_string(), Pin::new_output("", width));

        PinComponent {
            id,
            pins,
            direction: PinDirection::Input,
            width,
        }
    }

    /// Create a new output pin
    pub fn new_output(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("".to_string(), Pin::new_input("", width));

        PinComponent {
            id,
            pins,
            direction: PinDirection::Output,
            width,
        }
    }
}

impl Component for PinComponent {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Pin"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Pins are simple pass-through - they just connect signals
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for pins
    }
}

impl Propagator for PinComponent {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        _current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal.clone());
        }

        let mut result = UpdateResult::new();
        result.add_output("".to_string(), signal);
        result
    }
}

/// OR Gate implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl OrGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        OrGate { id, pins }
    }
}

impl Component for OrGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "OR"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.or(b);
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for OrGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// NOT Gate implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl NotGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NotGate { id, pins }
    }
}

impl Component for NotGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "NOT"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.not();
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for NotGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// NAND Gate implementation  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NandGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl NandGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NandGate { id, pins }
    }
}

impl Component for NandGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "NAND"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.and(b).not();
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for NandGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// NOR Gate implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl NorGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        NorGate { id, pins }
    }
}

impl Component for NorGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "NOR"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.or(b).not();
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for NorGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// XOR Gate implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XorGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl XorGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        XorGate { id, pins }
    }
}

impl Component for XorGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "XOR"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.xor(b);
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for XorGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// XNOR Gate implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XnorGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl XnorGate {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        XnorGate { id, pins }
    }
}

impl Component for XnorGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "XNOR"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self
            .pins
            .get("A")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);
        let b = self
            .pins
            .get("B")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let output = a.xor(b).not();
        let output_signal = Signal::new_single(output);

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        1
    }
}

impl Propagator for XnorGate {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// Constant value component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constant {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    value: Value,
    width: BusWidth,
}

impl Constant {
    pub fn new(id: ComponentId, value: Value, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Y".to_string(), Pin::new_output("Y", width));

        Constant { id, pins, value, width }
    }
}

impl Component for Constant {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Constant"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let output_signal = if self.width.is_single_bit() {
            Signal::new_single(self.value)
        } else {
            Signal::new_uniform(self.width, self.value)
        };

        let mut result = UpdateResult::new();
        result.add_output("Y".to_string(), output_signal.clone());

        if let Some(pin) = self.pins.get_mut("Y") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        // Constants maintain their value on reset
        self.update(Timestamp(0));
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for constants
    }
}

impl Propagator for Constant {
    fn propagate(
        &mut self,
        _input_pin: &str,
        _signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        // Constants don't have inputs, they always output their fixed value
        self.update(current_time)
    }
}

/// Probe component for monitoring signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Probe {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Probe {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", width));

        Probe { id, pins, width }
    }
}

impl Component for Probe {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Probe"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Probes just monitor the input signal
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for probes
    }
}

impl Propagator for Probe {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        _current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        UpdateResult::new()
    }
}

/// Tunnel component for connecting signals by name
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    label: String,
    width: BusWidth,
}

impl Tunnel {
    pub fn new(id: ComponentId, label: String, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("".to_string(), Pin::new_inout("", width));

        Tunnel { id, pins, label, width }
    }
}

impl Component for Tunnel {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Tunnel"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Tunnels are pass-through components for named connections
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for tunnels
    }
}

impl Propagator for Tunnel {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        _current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal.clone());
        }

        let mut result = UpdateResult::new();
        result.add_output("".to_string(), signal);
        result
    }
}

/// Splitter component for combining/splitting buses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Splitter {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    fanout: u32,
    incoming_width: BusWidth,
}

impl Splitter {
    pub fn new(id: ComponentId, fanout: u32, incoming_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Main bus connection
        pins.insert("combined".to_string(), Pin::new_inout("combined", incoming_width));
        
        // Individual bit connections
        for i in 0..fanout {
            let bit_width = incoming_width.as_u32() / fanout.max(1);
            pins.insert(format!("bit{}", i), Pin::new_inout(&format!("bit{}", i), BusWidth(bit_width.max(1))));
        }

        Splitter { id, pins, fanout, incoming_width }
    }
}

impl Component for Splitter {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Splitter"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();
        
        // Check if we're combining (bits -> combined) or splitting (combined -> bits)
        if let Some(combined_pin) = self.pins.get("combined") {
            let combined_signal = &combined_pin.signal;
            
            // Split combined signal to individual bits
            if combined_signal.width().as_u32() > 1 {
                let values = combined_signal.values();
                let bits_per_output = (values.len() as u32) / self.fanout.max(1);
                
                for i in 0..self.fanout {
                    let start_idx = (i * bits_per_output) as usize;
                    let end_idx = ((i + 1) * bits_per_output).min(values.len() as u32) as usize;
                    
                    if start_idx < values.len() {
                        let bit_values = values[start_idx..end_idx].to_vec();
                        let bit_signal = if bit_values.len() == 1 {
                            Signal::new_single(bit_values[0])
                        } else {
                            Signal::new_bus(bit_values)
                        };
                        
                        result.add_output(format!("bit{}", i), bit_signal);
                    }
                }
            }
        }
        
        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for splitters
    }
}

impl Propagator for Splitter {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// LED component for visual output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Led {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Led {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", BusWidth(1)));

        Led { id, pins }
    }
}

impl Component for Led {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "LED"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // LEDs are output-only components that display the input signal
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for LEDs
    }
}

impl Propagator for Led {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        _current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        UpdateResult::new()
    }
}

/// Multiplexer component for data selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplexer {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    inputs: u32,
    width: BusWidth,
}

impl Multiplexer {
    pub fn new(id: ComponentId, inputs: u32, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Data inputs
        for i in 0..inputs {
            pins.insert(format!("IN{}", i), Pin::new_input(&format!("IN{}", i), width));
        }
        
        // Select input
        let select_width = BusWidth((inputs as f32).log2().ceil() as u32);
        pins.insert("SEL".to_string(), Pin::new_input("SEL", select_width));
        
        // Output
        pins.insert("OUT".to_string(), Pin::new_output("OUT", width));

        Multiplexer { id, pins, inputs, width }
    }
}

impl Component for Multiplexer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Multiplexer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let sel_signal = self.pins.get("SEL").unwrap().signal.clone();
        let mut selected_input = 0u32;
        
        // Convert select signal to index
        if let Some(sel_value) = sel_signal.as_single() {
            match sel_value {
                Value::High => selected_input = 1,
                Value::Low => selected_input = 0,
                _ => selected_input = 0,
            }
        } else {
            // Multi-bit select - simplified for now
            selected_input = 0;
        }
        
        // Clamp to valid range
        selected_input = selected_input.min(self.inputs - 1);
        
        let input_signal = self.pins.get(&format!("IN{}", selected_input))
            .map(|pin| pin.signal.clone())
            .unwrap_or_else(|| Signal::unknown(self.width));

        let mut result = UpdateResult::new();
        result.add_output("OUT".to_string(), input_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("OUT") {
            let _ = pin.set_signal(input_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for multiplexer
    }
}

impl Propagator for Multiplexer {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

/// Demultiplexer component for data routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demultiplexer {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    outputs: u32,
    width: BusWidth,
}

impl Demultiplexer {
    pub fn new(id: ComponentId, outputs: u32, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        
        // Data input
        pins.insert("IN".to_string(), Pin::new_input("IN", width));
        
        // Select input
        let select_width = BusWidth((outputs as f32).log2().ceil() as u32);
        pins.insert("SEL".to_string(), Pin::new_input("SEL", select_width));
        
        // Data outputs
        for i in 0..outputs {
            pins.insert(format!("OUT{}", i), Pin::new_output(&format!("OUT{}", i), width));
        }

        Demultiplexer { id, pins, outputs, width }
    }
}

impl Component for Demultiplexer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Demultiplexer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let sel_signal = self.pins.get("SEL").unwrap().signal.clone();
        let input_signal = self.pins.get("IN").unwrap().signal.clone();
        let mut selected_output = 0u32;
        
        // Convert select signal to index
        if let Some(sel_value) = sel_signal.as_single() {
            match sel_value {
                Value::High => selected_output = 1,
                Value::Low => selected_output = 0,
                _ => selected_output = 0,
            }
        }
        
        // Clamp to valid range
        selected_output = selected_output.min(self.outputs - 1);
        
        let mut result = UpdateResult::new();
        
        // Route input to selected output, others get unknown
        for i in 0..self.outputs {
            let output_signal = if i == selected_output {
                input_signal.clone()
            } else {
                Signal::unknown(self.width)
            };
            
            result.add_output(format!("OUT{}", i), output_signal.clone());
            
            if let Some(pin) = self.pins.get_mut(&format!("OUT{}", i)) {
                let _ = pin.set_signal(output_signal);
            }
        }
        
        result.set_delay(self.propagation_delay());
        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for demultiplexer
    }
}

impl Propagator for Demultiplexer {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if let Some(pin) = self.pins.get_mut(input_pin) {
            let _ = pin.set_signal(signal);
        }
        self.update(current_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_creation() {
        let pin = Pin::new_input("A", BusWidth(1));
        assert_eq!(pin.name, "A");
        assert!(pin.is_input());
        assert!(!pin.is_output());
        assert_eq!(pin.width, BusWidth(1));
    }

    #[test]
    fn test_and_gate() {
        let mut gate = AndGate::new(ComponentId(1));

        // Set inputs
        gate.get_pin_mut("A")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();
        gate.get_pin_mut("B")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();

        // Update
        let result = gate.update(Timestamp(0));
        assert!(result.state_changed);
        assert_eq!(result.outputs.len(), 1);

        let output = result.outputs.get("Y").unwrap();
        assert_eq!(output.as_single(), Some(Value::High));
    }

    #[test]
    fn test_and_gate_logic() {
        let mut gate = AndGate::new(ComponentId(1));

        // Test all combinations
        let test_cases = [
            (Value::Low, Value::Low, Value::Low),
            (Value::Low, Value::High, Value::Low),
            (Value::High, Value::Low, Value::Low),
            (Value::High, Value::High, Value::High),
        ];

        for (a, b, expected) in test_cases {
            gate.get_pin_mut("A")
                .unwrap()
                .set_signal(Signal::new_single(a))
                .unwrap();
            gate.get_pin_mut("B")
                .unwrap()
                .set_signal(Signal::new_single(b))
                .unwrap();

            let result = gate.update(Timestamp(0));
            let output = result.outputs.get("Y").unwrap();
            assert_eq!(
                output.as_single(),
                Some(expected),
                "AND({}, {}) should be {}",
                a,
                b,
                expected
            );
        }
    }

    #[test]
    fn test_clocked_latch() {
        let mut latch = ClockedLatch::new(ComponentId(2));

        // Set D input
        latch
            .get_pin_mut("D")
            .unwrap()
            .set_signal(Signal::new_single(Value::High))
            .unwrap();

        // Clock edge should capture the input
        let result = latch.clock_edge(ClockEdge::Rising, Timestamp(0));
        assert!(result.state_changed);

        let output = result.outputs.get("Q").unwrap();
        assert_eq!(output.as_single(), Some(Value::High));
        assert_eq!(latch.stored_value, Value::High);
    }
}
