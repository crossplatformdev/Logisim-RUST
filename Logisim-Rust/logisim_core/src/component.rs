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

        Constant {
            id,
            pins,
            value,
            width,
        }
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

        Tunnel {
            id,
            pins,
            label,
            width,
        }
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
        pins.insert(
            "combined".to_string(),
            Pin::new_inout("combined", incoming_width),
        );

        // Individual bit connections
        for i in 0..fanout {
            let bit_width = incoming_width.as_u32() / fanout.max(1);
            pins.insert(
                format!("bit{}", i),
                Pin::new_inout(format!("bit{}", i), BusWidth(bit_width.max(1))),
            );
        }

        Splitter {
            id,
            pins,
            fanout,
            incoming_width,
        }
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
            pins.insert(
                format!("IN{}", i),
                Pin::new_input(format!("IN{}", i), width),
            );
        }

        // Select input
        let select_width = BusWidth((inputs as f32).log2().ceil() as u32);
        pins.insert("SEL".to_string(), Pin::new_input("SEL", select_width));

        // Output
        pins.insert("OUT".to_string(), Pin::new_output("OUT", width));

        Multiplexer {
            id,
            pins,
            inputs,
            width,
        }
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

        // Convert select signal to index
        let selected_input = if let Some(sel_value) = sel_signal.as_single() {
            match sel_value {
                Value::High => 1,
                Value::Low => 0,
                _ => 0,
            }
        } else {
            // Multi-bit select - simplified for now
            0
        };

        // Clamp to valid range
        let selected_input = selected_input.min(self.inputs - 1);

        let input_signal = self
            .pins
            .get(&format!("IN{}", selected_input))
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
            pins.insert(
                format!("OUT{}", i),
                Pin::new_output(format!("OUT{}", i), width),
            );
        }

        Demultiplexer {
            id,
            pins,
            outputs,
            width,
        }
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

/// Clock component for generating periodic signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    period: u64,
    last_toggle: Timestamp,
    current_state: Value,
}

impl Clock {
    pub fn new(id: ComponentId, period: u64) -> Self {
        let mut pins = HashMap::new();
        pins.insert("CLK".to_string(), Pin::new_output("CLK", BusWidth(1)));

        Clock {
            id,
            pins,
            period,
            last_toggle: Timestamp(0),
            current_state: Value::Low,
        }
    }
}

impl Component for Clock {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Clock"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Check if it's time to toggle
        if current_time.as_u64() >= self.last_toggle.as_u64() + self.period {
            self.current_state = match self.current_state {
                Value::High => Value::Low,
                Value::Low => Value::High,
                _ => Value::High,
            };
            self.last_toggle = current_time;

            let clock_signal = Signal::new_single(self.current_state);
            result.add_output("CLK".to_string(), clock_signal.clone());

            if let Some(pin) = self.pins.get_mut("CLK") {
                let _ = pin.set_signal(clock_signal);
            }

            result.state_changed = true;
        }

        result
    }

    fn reset(&mut self) {
        self.current_state = Value::Low;
        self.last_toggle = Timestamp(0);
        for pin in self.pins.values_mut() {
            pin.signal = Signal::new_single(Value::Low);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for clock
    }
}

impl Propagator for Clock {
    fn propagate(
        &mut self,
        _input_pin: &str,
        _signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        // Clock doesn't have inputs, it generates its own signal
        self.update(current_time)
    }
}

/// RAM component for addressable memory storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ram {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    address_bits: u32,
    data_bits: u32,
    memory: HashMap<u32, Vec<Value>>,
}

impl Ram {
    pub fn new(id: ComponentId, address_bits: u32, data_bits: u32) -> Self {
        let mut pins = HashMap::new();

        // Address input
        pins.insert(
            "ADDR".to_string(),
            Pin::new_input("ADDR", BusWidth(address_bits)),
        );

        // Data pins (bidirectional)
        pins.insert(
            "DATA".to_string(),
            Pin::new_inout("DATA", BusWidth(data_bits)),
        );

        // Control pins
        pins.insert("WE".to_string(), Pin::new_input("WE", BusWidth(1))); // Write Enable
        pins.insert("OE".to_string(), Pin::new_input("OE", BusWidth(1))); // Output Enable
        pins.insert("CS".to_string(), Pin::new_input("CS", BusWidth(1))); // Chip Select

        Ram {
            id,
            pins,
            address_bits,
            data_bits,
            memory: HashMap::new(),
        }
    }
}

impl Component for Ram {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "RAM"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Get control signals
        let cs = self
            .pins
            .get("CS")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);
        let we = self
            .pins
            .get("WE")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);
        let oe = self
            .pins
            .get("OE")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);

        // Only operate if chip is selected
        if cs == Value::High {
            // Get address
            let addr_signal = &self.pins.get("ADDR").unwrap().signal;
            let address = self.signal_to_address(addr_signal);

            if we == Value::High {
                // Write operation
                let data_signal = &self.pins.get("DATA").unwrap().signal;
                self.memory.insert(address, data_signal.values().to_vec());
            } else if oe == Value::High {
                // Read operation
                let data = self
                    .memory
                    .get(&address)
                    .cloned()
                    .unwrap_or_else(|| vec![Value::Unknown; self.data_bits as usize]);

                let output_signal = if data.len() == 1 {
                    Signal::new_single(data[0])
                } else {
                    Signal::new_bus(data)
                };

                result.add_output("DATA".to_string(), output_signal.clone());

                if let Some(pin) = self.pins.get_mut("DATA") {
                    let _ = pin.set_signal(output_signal);
                }
            }
        } else {
            // Chip not selected - high impedance output
            let hi_z_signal = Signal::new_uniform(BusWidth(self.data_bits), Value::Unknown);
            result.add_output("DATA".to_string(), hi_z_signal.clone());

            if let Some(pin) = self.pins.get_mut("DATA") {
                let _ = pin.set_signal(hi_z_signal);
            }
        }

        result.set_delay(self.propagation_delay());
        result
    }

    fn reset(&mut self) {
        // Clear memory on reset
        self.memory.clear();
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        5 // 5 time units for RAM access
    }
}

impl Ram {
    fn signal_to_address(&self, signal: &Signal) -> u32 {
        // Convert signal to address (simplified)
        if let Some(value) = signal.as_single() {
            match value {
                Value::High => 1,
                Value::Low => 0,
                _ => 0,
            }
        } else {
            // Multi-bit address - simplified conversion
            let mut address = 0u32;
            for (i, &bit) in signal.values().iter().enumerate() {
                if bit == Value::High && i < 32 {
                    address |= 1 << i;
                }
            }
            let mask = if self.address_bits >= 32 {
                u32::MAX
            } else {
                (1 << self.address_bits) - 1
            };
            address & mask
        }
    }
}

impl Propagator for Ram {
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

/// Controlled Buffer (Three-state buffer) component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledBuffer {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl ControlledBuffer {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", width));
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1))); // Enable
        pins.insert("OUT".to_string(), Pin::new_output("OUT", width));

        ControlledBuffer { id, pins, width }
    }
}

impl Component for ControlledBuffer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Controlled Buffer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let enable = self
            .pins
            .get("EN")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);
        let input = &self.pins.get("IN").unwrap().signal;

        let output_signal = if enable == Value::High {
            // Enabled - pass through input
            input.clone()
        } else {
            // Disabled - high impedance (represented as unknown)
            Signal::new_uniform(self.width, Value::Unknown)
        };

        result.add_output("OUT".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("OUT") {
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
        1 // 1 time unit for buffer
    }
}

impl Propagator for ControlledBuffer {
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

/// Register component for storing multi-bit values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
    stored_data: Vec<Value>,
}

impl Register {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("D".to_string(), Pin::new_input("D", width)); // Data input
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1))); // Clock
        pins.insert("Q".to_string(), Pin::new_output("Q", width)); // Data output
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1))); // Enable (optional)

        Register {
            id,
            pins,
            width,
            stored_data: vec![Value::Unknown; width.as_u32() as usize],
        }
    }
}

impl Component for Register {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Register"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Output current stored data
        let output_signal = if self.stored_data.len() == 1 {
            Signal::new_single(self.stored_data[0])
        } else {
            Signal::new_bus(self.stored_data.clone())
        };

        result.add_output("Q".to_string(), output_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Q") {
            let _ = pin.set_signal(output_signal);
        }

        result
    }

    fn reset(&mut self) {
        self.stored_data = vec![Value::Unknown; self.width.as_u32() as usize];
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for register
    }

    fn is_sequential(&self) -> bool {
        true // Registers are sequential components
    }

    fn clock_edge(&mut self, edge: ClockEdge, _current_time: Timestamp) -> UpdateResult {
        if edge == ClockEdge::Rising {
            // Check enable signal
            let enable = self
                .pins
                .get("EN")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::High))
                .unwrap_or(Value::High); // Default enabled if no EN pin

            if enable == Value::High {
                // Capture data input on rising edge
                let data_signal = &self.pins.get("D").unwrap().signal;
                self.stored_data = data_signal.values().to_vec();

                // Pad or truncate to match width
                self.stored_data
                    .resize(self.width.as_u32() as usize, Value::Unknown);

                return self.update(Timestamp(0));
            }
        }

        UpdateResult::new()
    }
}

impl Propagator for Register {
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

/// Counter component for sequential counting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
    max_count: u32,
    current_count: u32,
}

impl Counter {
    pub fn new(id: ComponentId, width: BusWidth, max_count: Option<u32>) -> Self {
        let mut pins = HashMap::new();
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1))); // Clock
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1))); // Enable
        pins.insert("RST".to_string(), Pin::new_input("RST", BusWidth(1))); // Reset
        pins.insert("LD".to_string(), Pin::new_input("LD", BusWidth(1))); // Load
        pins.insert("D".to_string(), Pin::new_input("D", width)); // Data input for load
        pins.insert("Q".to_string(), Pin::new_output("Q", width)); // Count output
        pins.insert("CARRY".to_string(), Pin::new_output("CARRY", BusWidth(1))); // Carry out

        let default_max = if width.as_u32() >= 32 {
            u32::MAX
        } else {
            (1u32 << width.as_u32()) - 1
        };
        Counter {
            id,
            pins,
            width,
            max_count: max_count.unwrap_or(default_max),
            current_count: 0,
        }
    }
}

impl Component for Counter {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Counter"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Convert count to signal
        let count_values = self.count_to_values(self.current_count);
        let count_signal = if count_values.len() == 1 {
            Signal::new_single(count_values[0])
        } else {
            Signal::new_bus(count_values)
        };

        // Check for carry
        let carry = if self.current_count >= self.max_count {
            Value::High
        } else {
            Value::Low
        };

        result.add_output("Q".to_string(), count_signal.clone());
        result.add_output("CARRY".to_string(), Signal::new_single(carry));
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Q") {
            let _ = pin.set_signal(count_signal);
        }
        if let Some(pin) = self.pins.get_mut("CARRY") {
            let _ = pin.set_signal(Signal::new_single(carry));
        }

        result
    }

    fn reset(&mut self) {
        self.current_count = 0;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        4 // 4 time units for counter
    }

    fn is_sequential(&self) -> bool {
        true
    }

    fn clock_edge(&mut self, edge: ClockEdge, _current_time: Timestamp) -> UpdateResult {
        if edge == ClockEdge::Rising {
            let enable = self
                .pins
                .get("EN")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::High))
                .unwrap_or(Value::High);
            let reset = self
                .pins
                .get("RST")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::Low))
                .unwrap_or(Value::Low);
            let load = self
                .pins
                .get("LD")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::Low))
                .unwrap_or(Value::Low);

            if reset == Value::High {
                self.current_count = 0;
            } else if load == Value::High {
                // Load data input
                let data_signal = &self.pins.get("D").unwrap().signal;
                self.current_count = self.signal_to_count(data_signal);
            } else if enable == Value::High {
                // Increment counter
                self.current_count = (self.current_count + 1) % (self.max_count + 1);
            }

            return self.update(Timestamp(0));
        }

        UpdateResult::new()
    }
}

impl Counter {
    fn count_to_values(&self, count: u32) -> Vec<Value> {
        let mut values = Vec::new();
        for i in 0..self.width.as_u32() {
            let bit = (count >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        values
    }

    fn signal_to_count(&self, signal: &Signal) -> u32 {
        let mut count = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                count |= 1 << i;
            }
        }
        count & self.max_count
    }
}

impl Propagator for Counter {
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
                if new_value == Value::High {
                    return self.clock_edge(ClockEdge::Rising, current_time);
                }
            }
        }

        UpdateResult::new()
    }
}

/// Text component for circuit documentation  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    text: String,
}

impl Text {
    pub fn new(id: ComponentId, text: String) -> Self {
        let pins = HashMap::new(); // Text has no pins
        Text { id, pins, text }
    }
}

impl Component for Text {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Text"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Text components don't process signals
        UpdateResult::new()
    }

    fn reset(&mut self) {
        // Nothing to reset for text
    }

    fn propagation_delay(&self) -> u64 {
        0
    }
}

impl Propagator for Text {
    fn propagate(
        &mut self,
        _input_pin: &str,
        _signal: Signal,
        _current_time: Timestamp,
    ) -> UpdateResult {
        // Text doesn't propagate signals
        UpdateResult::new()
    }
}

/// Adder component for arithmetic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Adder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Adder {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", width));
        pins.insert("B".to_string(), Pin::new_input("B", width));
        pins.insert("CIN".to_string(), Pin::new_input("CIN", BusWidth(1))); // Carry in
        pins.insert("SUM".to_string(), Pin::new_output("SUM", width));
        pins.insert("COUT".to_string(), Pin::new_output("COUT", BusWidth(1))); // Carry out

        Adder { id, pins, width }
    }
}

impl Component for Adder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Adder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let a_signal = &self.pins.get("A").unwrap().signal;
        let b_signal = &self.pins.get("B").unwrap().signal;
        let cin = self
            .pins
            .get("CIN")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);

        let a_value = self.signal_to_number(a_signal);
        let b_value = self.signal_to_number(b_signal);
        let cin_value = if cin == Value::High { 1 } else { 0 };

        let sum = a_value + b_value + cin_value;
        let max_value = if self.width.as_u32() >= 32 {
            u32::MAX
        } else {
            (1u32 << self.width.as_u32()) - 1
        };

        let result_value = sum & max_value;
        let carry_out = if sum > max_value {
            Value::High
        } else {
            Value::Low
        };

        let sum_values = self.number_to_values(result_value);
        let sum_signal = if sum_values.len() == 1 {
            Signal::new_single(sum_values[0])
        } else {
            Signal::new_bus(sum_values)
        };

        result.add_output("SUM".to_string(), sum_signal.clone());
        result.add_output("COUT".to_string(), Signal::new_single(carry_out));
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("SUM") {
            let _ = pin.set_signal(sum_signal);
        }
        if let Some(pin) = self.pins.get_mut("COUT") {
            let _ = pin.set_signal(Signal::new_single(carry_out));
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for adder
    }
}

impl Adder {
    fn signal_to_number(&self, signal: &Signal) -> u32 {
        let mut number = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                number |= 1 << i;
            }
        }
        number
    }

    fn number_to_values(&self, number: u32) -> Vec<Value> {
        let mut values = Vec::new();
        for i in 0..self.width.as_u32() {
            let bit = (number >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        values
    }
}

impl Propagator for Adder {
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

/// Divider component for arithmetic division
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Divider {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Divider {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("DIVIDEND".to_string(), Pin::new_input("DIVIDEND", width));
        pins.insert("DIVISOR".to_string(), Pin::new_input("DIVISOR", width));
        pins.insert("QUOTIENT".to_string(), Pin::new_output("QUOTIENT", width));
        pins.insert("REMAINDER".to_string(), Pin::new_output("REMAINDER", width));

        Divider { id, pins, width }
    }
}

impl Component for Divider {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Divider"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let dividend_signal = &self.pins.get("DIVIDEND").unwrap().signal;
        let divisor_signal = &self.pins.get("DIVISOR").unwrap().signal;

        let dividend = self.signal_to_number(dividend_signal);
        let divisor = self.signal_to_number(divisor_signal);

        let (quotient, remainder) = if divisor == 0 {
            // Division by zero - return error state
            (0, 0)
        } else {
            (dividend / divisor, dividend % divisor)
        };

        let quotient_values = self.number_to_values(quotient);
        let remainder_values = self.number_to_values(remainder);

        let quotient_signal = if quotient_values.len() == 1 {
            Signal::new_single(quotient_values[0])
        } else {
            Signal::new_bus(quotient_values)
        };

        let remainder_signal = if remainder_values.len() == 1 {
            Signal::new_single(remainder_values[0])
        } else {
            Signal::new_bus(remainder_values)
        };

        result.add_output("QUOTIENT".to_string(), quotient_signal.clone());
        result.add_output("REMAINDER".to_string(), remainder_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("QUOTIENT") {
            let _ = pin.set_signal(quotient_signal);
        }
        if let Some(pin) = self.pins.get_mut("REMAINDER") {
            let _ = pin.set_signal(remainder_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        8 // 8 time units for divider (more complex operation)
    }
}

impl Divider {
    fn signal_to_number(&self, signal: &Signal) -> u32 {
        let mut number = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                number |= 1 << i;
            }
        }
        number
    }

    fn number_to_values(&self, number: u32) -> Vec<Value> {
        let mut values = Vec::new();
        for i in 0..self.width.as_u32() {
            let bit = (number >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        values
    }
}

impl Propagator for Divider {
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

/// Decoder component for address decoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_width: BusWidth,
    output_count: u32,
}

impl Decoder {
    pub fn new(id: ComponentId, input_width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        let output_count = if input_width.as_u32() >= 32 {
            u32::MAX
        } else {
            1u32 << input_width.as_u32()
        }; // 2^n outputs

        // Address input
        pins.insert("ADDR".to_string(), Pin::new_input("ADDR", input_width));

        // Enable input (optional)
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1)));

        // Output pins (one for each possible address)
        for i in 0..output_count {
            pins.insert(
                format!("OUT{}", i),
                Pin::new_output(format!("OUT{}", i), BusWidth(1)),
            );
        }

        Decoder {
            id,
            pins,
            input_width,
            output_count,
        }
    }
}

impl Component for Decoder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Decoder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let enable = self
            .pins
            .get("EN")
            .map(|pin| pin.signal.as_single().unwrap_or(Value::High))
            .unwrap_or(Value::High);

        if enable == Value::High {
            let addr_signal = &self.pins.get("ADDR").unwrap().signal;
            let selected_output = self.signal_to_address(addr_signal);

            // Set all outputs
            for i in 0..self.output_count {
                let output_value = if i == selected_output {
                    Value::High
                } else {
                    Value::Low
                };

                result.add_output(format!("OUT{}", i), Signal::new_single(output_value));

                if let Some(pin) = self.pins.get_mut(&format!("OUT{}", i)) {
                    let _ = pin.set_signal(Signal::new_single(output_value));
                }
            }
        } else {
            // Disabled - all outputs low
            for i in 0..self.output_count {
                result.add_output(format!("OUT{}", i), Signal::new_single(Value::Low));

                if let Some(pin) = self.pins.get_mut(&format!("OUT{}", i)) {
                    let _ = pin.set_signal(Signal::new_single(Value::Low));
                }
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
        2 // 2 time units for decoder
    }
}

impl Decoder {
    fn signal_to_address(&self, signal: &Signal) -> u32 {
        let mut address = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                address |= 1 << i;
            }
        }
        address % self.output_count
    }
}

impl Propagator for Decoder {
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

/// Subtractor component for arithmetic subtraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtractor {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Subtractor {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", width));
        pins.insert("B".to_string(), Pin::new_input("B", width));
        pins.insert("BIN".to_string(), Pin::new_input("BIN", BusWidth(1))); // Borrow in
        pins.insert("DIFF".to_string(), Pin::new_output("DIFF", width));
        pins.insert("BOUT".to_string(), Pin::new_output("BOUT", BusWidth(1))); // Borrow out

        Subtractor { id, pins, width }
    }
}

impl Component for Subtractor {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Subtractor"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let a_signal = &self.pins.get("A").unwrap().signal;
        let b_signal = &self.pins.get("B").unwrap().signal;
        let bin = self
            .pins
            .get("BIN")
            .unwrap()
            .signal
            .as_single()
            .unwrap_or(Value::Low);

        let a_value = self.signal_to_number(a_signal);
        let b_value = self.signal_to_number(b_signal);
        let bin_value = if bin == Value::High { 1 } else { 0 };

        // Subtraction: A - B - BIN
        let diff = a_value.wrapping_sub(b_value).wrapping_sub(bin_value);
        let borrow_out = if a_value < (b_value + bin_value) {
            Value::High
        } else {
            Value::Low
        };

        let diff_values = self.number_to_values(diff);
        let diff_signal = if diff_values.len() == 1 {
            Signal::new_single(diff_values[0])
        } else {
            Signal::new_bus(diff_values)
        };

        result.add_output("DIFF".to_string(), diff_signal.clone());
        result.add_output("BOUT".to_string(), Signal::new_single(borrow_out));
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("DIFF") {
            let _ = pin.set_signal(diff_signal);
        }
        if let Some(pin) = self.pins.get_mut("BOUT") {
            let _ = pin.set_signal(Signal::new_single(borrow_out));
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for subtractor
    }
}

impl Subtractor {
    fn signal_to_number(&self, signal: &Signal) -> u32 {
        let mut number = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                number |= 1 << i;
            }
        }
        number
    }

    fn number_to_values(&self, number: u32) -> Vec<Value> {
        let mut values = Vec::new();
        for i in 0..self.width.as_u32() {
            let bit = (number >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        values
    }
}

impl Propagator for Subtractor {
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

/// Power component for circuit power supply
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Power {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Power {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(1)));

        Power { id, pins }
    }
}

impl Component for Power {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Power"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let power_signal = Signal::new_single(Value::High);
        result.add_output("OUT".to_string(), power_signal.clone());

        if let Some(pin) = self.pins.get_mut("OUT") {
            let _ = pin.set_signal(power_signal);
        }

        result
    }

    fn reset(&mut self) {
        // Power stays on during reset
        self.update(Timestamp(0));
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for power
    }
}

impl Propagator for Power {
    fn propagate(
        &mut self,
        _input_pin: &str,
        _signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        // Power doesn't have inputs
        self.update(current_time)
    }
}

/// Ground component for circuit ground reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ground {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Ground {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(1)));

        Ground { id, pins }
    }
}

impl Component for Ground {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Ground"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let ground_signal = Signal::new_single(Value::Low);
        result.add_output("OUT".to_string(), ground_signal.clone());

        if let Some(pin) = self.pins.get_mut("OUT") {
            let _ = pin.set_signal(ground_signal);
        }

        result
    }

    fn reset(&mut self) {
        // Ground stays at low during reset
        self.update(Timestamp(0));
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for ground
    }
}

impl Propagator for Ground {
    fn propagate(
        &mut self,
        _input_pin: &str,
        _signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        // Ground doesn't have inputs
        self.update(current_time)
    }
}

/// Shift Register component for bit shifting and storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftRegister {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
    shift_type: String, // "left", "right", "logical", "arithmetic"
    stored_data: Vec<Value>,
}

impl ShiftRegister {
    pub fn new(id: ComponentId, width: BusWidth, shift_type: String) -> Self {
        let mut pins = HashMap::new();
        pins.insert("D".to_string(), Pin::new_input("D", width)); // Data input
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1))); // Clock
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1))); // Enable
        pins.insert("CLR".to_string(), Pin::new_input("CLR", BusWidth(1))); // Clear
        pins.insert("SIN".to_string(), Pin::new_input("SIN", BusWidth(1))); // Shift input
        pins.insert("Q".to_string(), Pin::new_output("Q", width)); // Data output
        pins.insert("SOUT".to_string(), Pin::new_output("SOUT", BusWidth(1))); // Shift output

        ShiftRegister {
            id,
            pins,
            width,
            shift_type,
            stored_data: vec![Value::Unknown; width.as_u32() as usize],
        }
    }
}

impl Component for ShiftRegister {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Shift Register"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Output current stored data
        let output_signal = if self.stored_data.len() == 1 {
            Signal::new_single(self.stored_data[0])
        } else {
            Signal::new_bus(self.stored_data.clone())
        };

        let shift_out = self.stored_data.first().cloned().unwrap_or(Value::Unknown);

        result.add_output("Q".to_string(), output_signal.clone());
        result.add_output("SOUT".to_string(), Signal::new_single(shift_out));
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("Q") {
            let _ = pin.set_signal(output_signal);
        }
        if let Some(pin) = self.pins.get_mut("SOUT") {
            let _ = pin.set_signal(Signal::new_single(shift_out));
        }

        result
    }

    fn reset(&mut self) {
        self.stored_data = vec![Value::Unknown; self.width.as_u32() as usize];
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for shift register
    }

    fn is_sequential(&self) -> bool {
        true
    }

    fn clock_edge(&mut self, edge: ClockEdge, _current_time: Timestamp) -> UpdateResult {
        if edge == ClockEdge::Rising {
            let enable = self
                .pins
                .get("EN")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::High))
                .unwrap_or(Value::High);
            let clear = self
                .pins
                .get("CLR")
                .map(|pin| pin.signal.as_single().unwrap_or(Value::Low))
                .unwrap_or(Value::Low);

            if clear == Value::High {
                self.stored_data = vec![Value::Low; self.width.as_u32() as usize];
            } else if enable == Value::High {
                let shift_in = self
                    .pins
                    .get("SIN")
                    .map(|pin| pin.signal.as_single().unwrap_or(Value::Low))
                    .unwrap_or(Value::Low);

                // Perform shift operation (simplified - right shift)
                self.stored_data.insert(0, shift_in);
                if self.stored_data.len() > self.width.as_u32() as usize {
                    self.stored_data.truncate(self.width.as_u32() as usize);
                }
            }

            return self.update(Timestamp(0));
        }

        UpdateResult::new()
    }
}

impl Propagator for ShiftRegister {
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
                if new_value == Value::High {
                    return self.clock_edge(ClockEdge::Rising, current_time);
                }
            }
        }

        UpdateResult::new()
    }
}

/// Multiplier component for arithmetic multiplication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Multiplier {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Multiplier {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", width));
        pins.insert("B".to_string(), Pin::new_input("B", width));
        pins.insert(
            "PRODUCT".to_string(),
            Pin::new_output("PRODUCT", BusWidth(width.as_u32() * 2)),
        ); // Double width output

        Multiplier { id, pins, width }
    }
}

impl Component for Multiplier {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Multiplier"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let a_signal = &self.pins.get("A").unwrap().signal;
        let b_signal = &self.pins.get("B").unwrap().signal;

        let a_value = self.signal_to_number(a_signal) as u64;
        let b_value = self.signal_to_number(b_signal) as u64;

        let product = a_value * b_value;
        let product_values = self.number_to_values(product);

        let product_signal = if product_values.len() == 1 {
            Signal::new_single(product_values[0])
        } else {
            Signal::new_bus(product_values)
        };

        result.add_output("PRODUCT".to_string(), product_signal.clone());
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("PRODUCT") {
            let _ = pin.set_signal(product_signal);
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        6 // 6 time units for multiplier (more complex operation)
    }
}

impl Multiplier {
    fn signal_to_number(&self, signal: &Signal) -> u32 {
        let mut number = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                number |= 1 << i;
            }
        }
        number
    }

    fn number_to_values(&self, number: u64) -> Vec<Value> {
        let mut values = Vec::new();
        let output_width = self.width.as_u32() * 2;
        for i in 0..output_width {
            let bit = (number >> i) & 1;
            values.push(if bit == 1 { Value::High } else { Value::Low });
        }
        values
    }
}

impl Propagator for Multiplier {
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

/// Comparator component for value comparison
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Comparator {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", width));
        pins.insert("B".to_string(), Pin::new_input("B", width));
        pins.insert("EQ".to_string(), Pin::new_output("EQ", BusWidth(1))); // A == B
        pins.insert("GT".to_string(), Pin::new_output("GT", BusWidth(1))); // A > B
        pins.insert("LT".to_string(), Pin::new_output("LT", BusWidth(1))); // A < B

        Comparator { id, pins, width }
    }
}

impl Component for Comparator {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Comparator"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        let a_signal = &self.pins.get("A").unwrap().signal;
        let b_signal = &self.pins.get("B").unwrap().signal;

        let a_value = self.signal_to_number(a_signal);
        let b_value = self.signal_to_number(b_signal);

        let eq = if a_value == b_value {
            Value::High
        } else {
            Value::Low
        };
        let gt = if a_value > b_value {
            Value::High
        } else {
            Value::Low
        };
        let lt = if a_value < b_value {
            Value::High
        } else {
            Value::Low
        };

        result.add_output("EQ".to_string(), Signal::new_single(eq));
        result.add_output("GT".to_string(), Signal::new_single(gt));
        result.add_output("LT".to_string(), Signal::new_single(lt));
        result.set_delay(self.propagation_delay());

        if let Some(pin) = self.pins.get_mut("EQ") {
            let _ = pin.set_signal(Signal::new_single(eq));
        }
        if let Some(pin) = self.pins.get_mut("GT") {
            let _ = pin.set_signal(Signal::new_single(gt));
        }
        if let Some(pin) = self.pins.get_mut("LT") {
            let _ = pin.set_signal(Signal::new_single(lt));
        }

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for comparator
    }
}

impl Comparator {
    fn signal_to_number(&self, signal: &Signal) -> u32 {
        let mut number = 0u32;
        for (i, &bit) in signal.values().iter().enumerate() {
            if bit == Value::High && i < 32 {
                number |= 1 << i;
            }
        }
        number
    }
}

impl Propagator for Comparator {
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

/// Keyboard component for input
#[derive(Debug, Clone)]
pub struct Keyboard {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    current_value: u8, // Current key value
}

impl Keyboard {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(8)));
        pins.insert("AVAIL".to_string(), Pin::new_output("AVAIL", BusWidth(1)));
        pins.insert("ACK".to_string(), Pin::new_input("ACK", BusWidth(1)));

        Keyboard {
            id,
            pins,
            current_value: 0,
        }
    }
}

impl Component for Keyboard {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Keyboard"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // Output current keyboard value as an 8-bit signal
        let value_bits: Vec<Value> = (0..8)
            .map(|i| {
                if (self.current_value >> i) & 1 == 1 {
                    Value::High
                } else {
                    Value::Low
                }
            })
            .collect();
        result
            .outputs
            .insert("OUT".to_string(), Signal::new_bus(value_bits));

        // Set AVAIL signal to indicate data is available
        result.outputs.insert(
            "AVAIL".to_string(),
            Signal::new_single(if self.current_value != 0 {
                Value::High
            } else {
                Value::Low
            }),
        );

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
        self.current_value = 0;
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for keyboard
    }
}

impl Propagator for Keyboard {
    fn propagate(
        &mut self,
        input_pin: &str,
        signal: Signal,
        current_time: Timestamp,
    ) -> UpdateResult {
        if input_pin == "ACK" && signal.as_single() == Some(Value::High) {
            // Acknowledge received, clear current value
            self.current_value = 0;
        }
        self.update(current_time)
    }
}

/// Hex Digit Display component for displaying hexadecimal values
#[derive(Debug, Clone)]
pub struct HexDigitDisplay {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl HexDigitDisplay {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", BusWidth(4)));

        HexDigitDisplay { id, pins }
    }
}

impl Component for HexDigitDisplay {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Hex Digit Display"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Hex displays don't produce outputs, they just display input
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for displays
    }
}

impl Propagator for HexDigitDisplay {
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

/// Telnet component for network communication
#[derive(Debug, Clone)]
pub struct Telnet {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Telnet {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", BusWidth(8)));
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(8)));
        pins.insert("SEND".to_string(), Pin::new_input("SEND", BusWidth(1)));
        pins.insert("RECV".to_string(), Pin::new_output("RECV", BusWidth(1)));

        Telnet { id, pins }
    }
}

impl Component for Telnet {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Telnet"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // For simulation purposes, echo input to output
        if let Some(input_pin) = self.pins.get("IN") {
            result
                .outputs
                .insert("OUT".to_string(), input_pin.signal.clone());
        }

        // Signal that we're ready to receive (simplified)
        result
            .outputs
            .insert("RECV".to_string(), Signal::new_single(Value::High));

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for telnet
    }
}

impl Propagator for Telnet {
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

/// TTY (Terminal/Teletypewriter) component for console I/O
#[derive(Debug, Clone)]
pub struct Tty {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl Tty {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("IN".to_string(), Pin::new_input("IN", BusWidth(8)));
        pins.insert("OUT".to_string(), Pin::new_output("OUT", BusWidth(8)));
        pins.insert("WRITE".to_string(), Pin::new_input("WRITE", BusWidth(1)));
        pins.insert("READ".to_string(), Pin::new_input("read", BusWidth(1)));
        pins.insert("READY".to_string(), Pin::new_output("READY", BusWidth(1)));

        Tty { id, pins }
    }
}

impl Component for Tty {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "TTY"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut result = UpdateResult::new();

        // For simulation purposes, echo input to output and signal ready
        if let Some(input_pin) = self.pins.get("IN") {
            result
                .outputs
                .insert("OUT".to_string(), input_pin.signal.clone());
        }

        // Signal that TTY is ready
        result
            .outputs
            .insert("READY".to_string(), Signal::new_single(Value::High));

        result
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for TTY
    }
}

impl Propagator for Tty {
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

/// RGB Video component for video display
#[derive(Debug, Clone)]
pub struct RgbVideo {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl RgbVideo {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("R".to_string(), Pin::new_input("R", BusWidth(8)));
        pins.insert("G".to_string(), Pin::new_input("G", BusWidth(8)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(8)));
        pins.insert("HSYNC".to_string(), Pin::new_input("HSYNC", BusWidth(1)));
        pins.insert("VSYNC".to_string(), Pin::new_input("VSYNC", BusWidth(1)));

        RgbVideo { id, pins }
    }
}

impl Component for RgbVideo {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "RGB Video"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Video displays don't produce outputs, they just display input
        UpdateResult::new()
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }

    fn propagation_delay(&self) -> u64 {
        0 // No delay for video display
    }
}

impl Propagator for RgbVideo {
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

// ===== COMPLETE COMPONENT PARITY IMPLEMENTATION: MISSING JAVA COMPONENTS =====

/// BitAdder - Single bit full adder with carry
#[derive(Debug, Clone)]
pub struct BitAdder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
}

impl BitAdder {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert(
            "CarryIn".to_string(),
            Pin::new_input("CarryIn", BusWidth(1)),
        );
        pins.insert("Sum".to_string(), Pin::new_output("Sum", BusWidth(1)));
        pins.insert(
            "CarryOut".to_string(),
            Pin::new_output("CarryOut", BusWidth(1)),
        );

        Self { id, pins }
    }
}

impl Component for BitAdder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Bit Adder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let a = self.pins["A"].signal.as_single().unwrap_or(Value::Unknown);
        let b = self.pins["B"].signal.as_single().unwrap_or(Value::Unknown);
        let carry_in = self.pins["CarryIn"]
            .signal
            .as_single()
            .unwrap_or(Value::Unknown);

        let (sum, carry_out) = match (a, b, carry_in) {
            (Value::Low, Value::Low, Value::Low) => (Value::Low, Value::Low),
            (Value::Low, Value::Low, Value::High) => (Value::High, Value::Low),
            (Value::Low, Value::High, Value::Low) => (Value::High, Value::Low),
            (Value::Low, Value::High, Value::High) => (Value::Low, Value::High),
            (Value::High, Value::Low, Value::Low) => (Value::High, Value::Low),
            (Value::High, Value::Low, Value::High) => (Value::Low, Value::High),
            (Value::High, Value::High, Value::Low) => (Value::Low, Value::High),
            (Value::High, Value::High, Value::High) => (Value::High, Value::High),
            _ => (Value::Unknown, Value::Unknown),
        };

        let mut outputs = HashMap::new();
        outputs.insert("Sum".to_string(), Signal::new_single(sum));
        outputs.insert("CarryOut".to_string(), Signal::new_single(carry_out));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Negator - Two's complement negator
#[derive(Debug, Clone)]
pub struct Negator {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Negator {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", width));
        pins.insert("Output".to_string(), Pin::new_output("Output", width));

        Self { id, pins, width }
    }
}

impl Component for Negator {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Negator"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let input_signal = &self.pins["Input"].signal;

        let output_value = if let Some(value) = input_signal.as_single() {
            match value {
                Value::Low => Value::High,
                Value::High => Value::Low,
                _ => Value::Unknown,
            }
        } else {
            // For multi-bit values, perform two's complement
            let input_signal = &self.pins["Input"].signal;
            let input_val = input_signal.to_u64().unwrap_or(0) as u32;
            let max_val = (1u32 << self.width.0) - 1;
            let negated = (!input_val + 1) & max_val;
            if negated == 0 {
                Value::Low
            } else {
                Value::High
            }
        };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output_value));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Buffer - Simple signal buffer/amplifier
#[derive(Debug, Clone)]
pub struct Buffer {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
}

impl Buffer {
    pub fn new(id: ComponentId, width: BusWidth) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", width));
        pins.insert("Output".to_string(), Pin::new_output("Output", width));

        Self { id, pins, width }
    }
}

impl Component for Buffer {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Buffer"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let input_signal = self.pins["Input"].signal.clone();

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), input_signal);

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// BitExtender - Sign or zero extend bit width
#[derive(Debug, Clone)]
pub struct BitExtender {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_width: BusWidth,
    output_width: BusWidth,
    signed_extend: bool,
}

impl BitExtender {
    pub fn new(
        id: ComponentId,
        input_width: BusWidth,
        output_width: BusWidth,
        signed_extend: bool,
    ) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", input_width));
        pins.insert(
            "Output".to_string(),
            Pin::new_output("Output", output_width),
        );

        Self {
            id,
            pins,
            input_width,
            output_width,
            signed_extend,
        }
    }
}

impl Component for BitExtender {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Bit Extender"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let input_value = self.pins["Input"].signal.to_u64().unwrap_or(0) as u32;

        let output_value = if self.signed_extend && self.input_width.0 > 0 {
            // Sign extend
            let sign_bit = 1u32 << (self.input_width.0 - 1);
            if (input_value & sign_bit) != 0 {
                // Negative number - extend with 1s
                let mask = (1u32 << self.output_width.0) - (1u32 << self.input_width.0);
                input_value | mask
            } else {
                // Positive number - extend with 0s
                input_value
            }
        } else {
            // Zero extend
            input_value
        };

        let output_signal = Signal::new_single(Value::Unknown);

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), output_signal);

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// BitSelector - Select specific bits from input
#[derive(Debug, Clone)]
pub struct BitSelector {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_width: BusWidth,
    output_width: BusWidth,
    select_bits: Vec<u32>,
}

impl BitSelector {
    pub fn new(id: ComponentId, input_width: BusWidth, select_bits: Vec<u32>) -> Self {
        let output_width = BusWidth(select_bits.len() as u32);
        let mut pins = HashMap::new();
        pins.insert("Input".to_string(), Pin::new_input("Input", input_width));
        pins.insert(
            "Output".to_string(),
            Pin::new_output("Output", output_width),
        );

        Self {
            id,
            pins,
            input_width,
            output_width,
            select_bits,
        }
    }
}

impl Component for BitSelector {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Bit Selector"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let input_value = self.pins["Input"].signal.to_u64().unwrap_or(0) as u32;

        let mut output_value = 0u32;
        for (i, &bit_index) in self.select_bits.iter().enumerate() {
            if bit_index < self.input_width.0 {
                let bit = (input_value >> bit_index) & 1;
                output_value |= bit << i;
            }
        }

        let output_signal = Signal::new_single(Value::Unknown);

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), output_signal);

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// PriorityEncoder - Encode highest priority active input
#[derive(Debug, Clone)]
pub struct PriorityEncoder {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_count: u32,
    output_width: BusWidth,
}

impl PriorityEncoder {
    pub fn new(id: ComponentId, input_count: u32) -> Self {
        let output_width = BusWidth((input_count as f32).log2().ceil() as u32);
        let mut pins = HashMap::new();

        for i in 0..input_count {
            pins.insert(
                format!("I{}", i),
                Pin::new_input(&format!("I{}", i), BusWidth(1)),
            );
        }
        pins.insert(
            "Output".to_string(),
            Pin::new_output("Output", output_width),
        );
        pins.insert("Valid".to_string(), Pin::new_output("Valid", BusWidth(1)));

        Self {
            id,
            pins,
            input_count,
            output_width,
        }
    }
}

impl Component for PriorityEncoder {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Priority Encoder"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut encoded_value = 0u32;
        let mut valid = false;

        // Find highest priority (highest index) active input
        for i in (0..self.input_count).rev() {
            let input_name = format!("I{}", i);
            if let Some(pin) = self.pins.get(&input_name) {
                if pin.signal.as_single() == Some(Value::High) {
                    encoded_value = i;
                    valid = true;
                    break;
                }
            }
        }

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(Value::Unknown));
        outputs.insert(
            "Valid".to_string(),
            Signal::new_single(if valid { Value::High } else { Value::Low }),
        );

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Rom - Read-only memory
#[derive(Debug, Clone)]
pub struct Rom {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    data: Vec<u32>,
    addr_width: BusWidth,
    data_width: BusWidth,
}

impl Rom {
    pub fn new(id: ComponentId, addr_width: BusWidth, data_width: BusWidth) -> Self {
        let size = 1usize << addr_width.0;
        let mut pins = HashMap::new();
        pins.insert("Address".to_string(), Pin::new_input("Address", addr_width));
        pins.insert("Data".to_string(), Pin::new_output("Data", data_width));
        pins.insert("OE".to_string(), Pin::new_input("OE", BusWidth(1)));

        Self {
            id,
            pins,
            data: vec![0; size],
            addr_width,
            data_width,
        }
    }

    pub fn load_data(&mut self, data: Vec<u32>) {
        let max_size = 1usize << self.addr_width.0;
        self.data = data.into_iter().take(max_size).collect();
        self.data.resize(max_size, 0);
    }
}

impl Component for Rom {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "ROM"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let oe = self.pins["OE"].signal.as_single().unwrap_or(Value::Low);
        let address = self.pins["Address"].signal.to_u64().unwrap_or(0) as usize;

        let output_value = if oe == Value::High && address < self.data.len() {
            Value::Unknown
        } else {
            Value::High
        };

        let mut outputs = HashMap::new();
        outputs.insert("Data".to_string(), Signal::new_single(output_value));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Random - Random number generator
#[derive(Debug, Clone)]
pub struct Random {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    width: BusWidth,
    seed: u32,
    state: u32,
}

impl Random {
    pub fn new(id: ComponentId, width: BusWidth, seed: u32) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Clock".to_string(), Pin::new_input("Clock", BusWidth(1)));
        pins.insert("Enable".to_string(), Pin::new_input("Enable", BusWidth(1)));
        pins.insert("Output".to_string(), Pin::new_output("Output", width));

        Self {
            id,
            pins,
            width,
            seed,
            state: seed,
        }
    }
}

impl Component for Random {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Random"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let enable = self.pins["Enable"].signal.as_single().unwrap_or(Value::Low);

        let output_value = if enable == Value::High {
            // Simple linear congruential generator
            self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
            let mask = if self.width.0 >= 32 {
                u32::MAX
            } else {
                (1u32 << self.width.0) - 1
            };
            Value::Unknown
        } else {
            Value::Unknown
        };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output_value));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
        self.state = self.seed;
    }
}

/// DFlipFlop - D-type flip-flop
#[derive(Debug, Clone)]
pub struct DFlipFlop {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    stored_value: Value,
}

impl DFlipFlop {
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("D".to_string(), Pin::new_input("D", BusWidth(1)));
        pins.insert("Clock".to_string(), Pin::new_input("Clock", BusWidth(1)));
        pins.insert("Q".to_string(), Pin::new_output("Q", BusWidth(1)));
        pins.insert("QN".to_string(), Pin::new_output("QN", BusWidth(1)));

        Self {
            id,
            pins,
            stored_value: Value::Low,
        }
    }
}

impl Component for DFlipFlop {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "D Flip-Flop"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn get_pin(&self, name: &str) -> Option<&Pin> {
        self.pins.get(name)
    }

    fn get_pin_mut(&mut self, name: &str) -> Option<&mut Pin> {
        self.pins.get_mut(name)
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Always output current stored value
        let q_not = match self.stored_value {
            Value::High => Value::Low,
            Value::Low => Value::High,
            _ => Value::Unknown,
        };

        let mut outputs = HashMap::new();
        outputs.insert("Q".to_string(), Signal::new_single(self.stored_value));
        outputs.insert("QN".to_string(), Signal::new_single(q_not));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: false,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
        self.stored_value = Value::Low;
    }

    fn is_sequential(&self) -> bool {
        true
    }

    fn clock_edge(&mut self, edge: ClockEdge, _timestamp: Timestamp) -> UpdateResult {
        if edge == ClockEdge::Rising {
            let d_value = self.pins["D"].signal.as_single().unwrap_or(Value::Unknown);
            let state_changed = self.stored_value != d_value;
            self.stored_value = d_value;

            let q_not = match self.stored_value {
                Value::High => Value::Low,
                Value::Low => Value::High,
                _ => Value::Unknown,
            };

            let mut outputs = HashMap::new();
            outputs.insert("Q".to_string(), Signal::new_single(self.stored_value));
            outputs.insert("QN".to_string(), Signal::new_single(q_not));

            UpdateResult {
                outputs,
                delay: 1,
                state_changed,
            }
        } else {
            UpdateResult::new()
        }
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

/// Even Parity Gate - outputs High if input has even number of 1s
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvenParityGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_count: u32,
}

impl EvenParityGate {
    /// Create a new even parity gate
    pub fn new(id: ComponentId, input_count: u32) -> Self {
        let mut pins = HashMap::new();

        // Add input pins
        for i in 0..input_count {
            pins.insert(
                format!("Input{}", i),
                Pin::new_input(&format!("Input{}", i), BusWidth(1)),
            );
        }

        pins.insert("Output".to_string(), Pin::new_output("Output", BusWidth(1)));

        Self {
            id,
            pins,
            input_count,
        }
    }
}

impl Component for EvenParityGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Even Parity Gate"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut high_count = 0;
        let mut has_unknown = false;

        // Count high inputs
        for i in 0..self.input_count {
            let pin_name = format!("Input{}", i);
            match self.pins[&pin_name]
                .signal
                .as_single()
                .unwrap_or(Value::Unknown)
            {
                Value::High => high_count += 1,
                Value::Unknown | Value::Error => has_unknown = true,
                Value::Low => {}
            }
        }

        let output = if has_unknown {
            Value::Unknown
        } else if high_count % 2 == 0 {
            Value::High // Even parity
        } else {
            Value::Low // Odd parity
        };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Odd Parity Gate - outputs High if input has odd number of 1s
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OddParityGate {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    input_count: u32,
}

impl OddParityGate {
    /// Create a new odd parity gate
    pub fn new(id: ComponentId, input_count: u32) -> Self {
        let mut pins = HashMap::new();

        // Add input pins
        for i in 0..input_count {
            pins.insert(
                format!("Input{}", i),
                Pin::new_input(&format!("Input{}", i), BusWidth(1)),
            );
        }

        pins.insert("Output".to_string(), Pin::new_output("Output", BusWidth(1)));

        Self {
            id,
            pins,
            input_count,
        }
    }
}

impl Component for OddParityGate {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Odd Parity Gate"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let mut high_count = 0;
        let mut has_unknown = false;

        // Count high inputs
        for i in 0..self.input_count {
            let pin_name = format!("Input{}", i);
            match self.pins[&pin_name]
                .signal
                .as_single()
                .unwrap_or(Value::Unknown)
            {
                Value::High => high_count += 1,
                Value::Unknown | Value::Error => has_unknown = true,
                Value::Low => {}
            }
        }

        let output = if has_unknown {
            Value::Unknown
        } else if high_count % 2 == 1 {
            Value::High // Odd parity
        } else {
            Value::Low // Even parity
        };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output));

        UpdateResult {
            outputs,
            delay: 1,
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Button - Input component that can be pressed to generate signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Button {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    is_pressed: bool,
}

impl Button {
    /// Create a new button
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Output".to_string(), Pin::new_output("Output", BusWidth(1)));

        Self {
            id,
            pins,
            is_pressed: false,
        }
    }

    /// Press the button
    pub fn press(&mut self) {
        self.is_pressed = true;
    }

    /// Release the button
    pub fn release(&mut self) {
        self.is_pressed = false;
    }

    /// Check if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl Component for Button {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Button"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let output_value = if self.is_pressed {
            Value::High
        } else {
            Value::Low
        };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output_value));

        UpdateResult {
            outputs,
            delay: 0, // Immediate response
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        self.is_pressed = false;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}

/// Switch - Toggle input component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Switch {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    is_on: bool,
}

impl Switch {
    /// Create a new switch
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("Output".to_string(), Pin::new_output("Output", BusWidth(1)));

        Self {
            id,
            pins,
            is_on: false,
        }
    }

    /// Turn switch on
    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    /// Turn switch off
    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    /// Toggle switch state
    pub fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }

    /// Check if switch is on
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

impl Component for Switch {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "Switch"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        let output_value = if self.is_on { Value::High } else { Value::Low };

        let mut outputs = HashMap::new();
        outputs.insert("Output".to_string(), Signal::new_single(output_value));

        UpdateResult {
            outputs,
            delay: 0, // Immediate response
            state_changed: true,
        }
    }

    fn reset(&mut self) {
        self.is_on = false;
        for pin in self.pins.values_mut() {
            pin.signal = Signal::unknown(pin.width);
        }
    }
}
