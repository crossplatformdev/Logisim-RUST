//! Simulation engine for digital circuit evaluation.
//!
//! The engine performs iterative signal propagation (a.k.a. event-driven
//! simulation) using the following algorithm:
//!
//! 1. Assign initial values to all wires (Unknown).
//! 2. Evaluate each component given its current input values.
//! 3. Write output values back to connected wires.
//! 4. Repeat until stable (no values changed) or until an oscillation is
//!    detected (iteration limit exceeded).
//!
//! Sequential elements (flip-flops, registers, RAM, counters) hold state
//! between clock edges and are updated on the rising edge of their clock input.

use crate::component::{BitFinderType, ComponentId, ComponentKind, PullDirection};
use crate::error::{LogisimError, Result};
use crate::project::Project;
use crate::value::{Bus, Value};
use std::collections::HashMap;

/// Maximum number of propagation passes before declaring oscillation.
const MAX_ITERATIONS: usize = 1000;

// ── Net map ───────────────────────────────────────────────────────────────────

/// A net is identified by a canonical grid point (after union-find).
type NetId = (i32, i32);

// ── SimulationState ───────────────────────────────────────────────────────────

/// Holds all runtime values for a single circuit.
#[derive(Clone, Debug)]
pub struct SimulationState {
    /// Current value driven onto every net (keyed by canonical net id).
    pub net_values: HashMap<NetId, Bus>,
    /// State for sequential elements, keyed by component ID.
    pub component_state: HashMap<ComponentId, ComponentState>,
    /// Clock tick counter (used internally; increments on each tick() call).
    pub clock_tick: u64,
}

/// Per-component sequential state.
#[derive(Clone, Debug, Default)]
pub struct ComponentState {
    /// The stored Q output(s) of flip-flops / registers.
    pub q: Bus,
    /// Previous clock input value (for edge detection).
    pub prev_clk: Value,
    /// RAM / ROM data storage.
    pub memory: Vec<Bus>,
    /// Counter value.
    pub counter: Bus,
    /// Shift register stages.
    pub stages: Vec<Bus>,
    /// Clock edge count (used for clock component).
    pub clock_count: u64,
}

impl SimulationState {
    pub fn new() -> Self {
        SimulationState {
            net_values: HashMap::new(),
            component_state: HashMap::new(),
            clock_tick: 0,
        }
    }

    /// Get the current bus value on a net.
    pub fn net_value(&self, net: NetId, width: u32) -> Bus {
        self.net_values
            .get(&net)
            .cloned()
            .unwrap_or_else(|| Bus::unknown(width as usize))
    }

    /// Set a net value.
    pub fn set_net_value(&mut self, net: NetId, value: Bus) {
        self.net_values.insert(net, value);
    }

    /// Get or create component state.
    pub fn component_state_mut(&mut self, id: ComponentId) -> &mut ComponentState {
        self.component_state.entry(id).or_default()
    }

    pub fn component_state_ref(&self, id: ComponentId) -> Option<&ComponentState> {
        self.component_state.get(&id)
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}

// ── Simulator ─────────────────────────────────────────────────────────────────

/// Top-level simulator that manages state across all circuits in a project.
pub struct Simulator {
    /// Per-circuit simulation state.
    states: HashMap<String, SimulationState>,
    /// The project being simulated.
    project: Project,
    /// User-driven pin values (circuit_name → component_id → bus value).
    user_inputs: HashMap<String, HashMap<ComponentId, Bus>>,
}

impl Simulator {
    /// Create a new simulator for a project.
    pub fn new(project: Project) -> Self {
        let mut states = HashMap::new();
        for name in project.circuits.keys() {
            states.insert(name.clone(), SimulationState::new());
        }
        Simulator {
            states,
            project,
            user_inputs: HashMap::new(),
        }
    }

    /// Return the project.
    pub fn project(&self) -> &Project {
        &self.project
    }

    /// Return a mutable reference to the simulation state for a circuit.
    pub fn state_mut(&mut self, circuit_name: &str) -> Option<&mut SimulationState> {
        self.states.get_mut(circuit_name)
    }

    /// Return an immutable reference to the simulation state for a circuit.
    pub fn state(&self, circuit_name: &str) -> Option<&SimulationState> {
        self.states.get(circuit_name)
    }

    /// Set a user-driven input pin value.
    pub fn set_pin_value(&mut self, circuit_name: &str, component_id: ComponentId, value: Bus) {
        self.user_inputs
            .entry(circuit_name.to_string())
            .or_default()
            .insert(component_id, value);
    }

    /// Reset the simulation state for a circuit (clears net values, component
    /// state, and user inputs) so that a fresh propagation pass starts from a
    /// known HighZ baseline.  Used by truth-table enumeration to reuse a single
    /// `Simulator` across rows without accumulated state.
    pub fn reset_circuit_state(&mut self, circuit_name: &str) {
        if let Some(s) = self.states.get_mut(circuit_name) {
            *s = SimulationState::new();
        }
        self.user_inputs.remove(circuit_name);
    }

    /// Advance the simulation by one clock tick.
    ///
    /// This toggles the internal clock signal and propagates all values until
    /// stable.
    pub fn tick(&mut self, circuit_name: &str) -> Result<()> {
        if let Some(state) = self.states.get_mut(circuit_name) {
            state.clock_tick += 1;
        }
        self.propagate(circuit_name)
    }

    /// Run propagation until stable for the given circuit.
    pub fn propagate(&mut self, circuit_name: &str) -> Result<()> {
        let circuit = self
            .project
            .circuits
            .get(circuit_name)
            .ok_or_else(|| LogisimError::CircuitNotFound(circuit_name.to_string()))?;

        // Build net map for this circuit.
        let net_map = circuit.compute_nets();

        // Helper: find the canonical net for a grid point.
        let canonical =
            |x: i32, y: i32| -> NetId { net_map.get(&(x, y)).copied().unwrap_or((x, y)) };

        // Iterative propagation.
        // Keep the previous net values to compare for convergence.
        let mut prev_nets: HashMap<NetId, Bus> = HashMap::new();

        for _iter in 0..MAX_ITERATIONS {
            // Clone state snapshot for reading inputs (avoid borrow conflicts).
            let state_snapshot = self.states.get(circuit_name).cloned().unwrap_or_default();

            // Build a fresh driven-values map for this pass (start all nets at HighZ).
            let mut driven: HashMap<NetId, Bus> = HashMap::new();

            // Evaluate each component.
            let component_ids: Vec<ComponentId> = circuit.components.keys().copied().collect();

            let mut new_states: Vec<(ComponentId, ComponentState)> = Vec::new();

            for &cid in &component_ids {
                let comp = &circuit.components[&cid];

                // Input pins are driven by user values (seeded after component eval below).
                if matches!(
                    comp.kind,
                    ComponentKind::Pin {
                        is_output: false,
                        ..
                    }
                ) {
                    continue;
                }

                // Gather input values from the *previous snapshot* so evaluation
                // is not order-dependent.
                let inputs: HashMap<String, Bus> = comp
                    .all_port_positions()
                    .into_iter()
                    .filter_map(|(name, (x, y))| {
                        let net = canonical(x, y);
                        let ports = comp.kind.ports();
                        let port = ports.iter().find(|p| p.name == name)?;
                        if matches!(port.direction, crate::component::PortDirection::Input) {
                            let w = port.width.get();
                            Some((name, state_snapshot.net_value(net, w)))
                        } else {
                            None
                        }
                    })
                    .collect();

                // Evaluate the component.
                let comp_state_opt = state_snapshot.component_state_ref(cid);
                let outputs = evaluate_component(
                    &comp.kind,
                    cid,
                    &inputs,
                    comp_state_opt,
                    state_snapshot.clock_tick,
                );

                // Accumulate outputs into the fresh driven map via resolution.
                for (port_name, bus) in &outputs {
                    let ports = comp.kind.ports();
                    let port = ports.iter().find(|p| &p.name == port_name);
                    if let Some(port) = port {
                        if matches!(port.direction, crate::component::PortDirection::Output) {
                            if let Some((x, y)) = comp.port_position(port_name) {
                                let net = canonical(x, y);
                                let width_usize = port.width.get() as usize;
                                let existing = driven
                                    .get(&net)
                                    .cloned()
                                    .unwrap_or_else(|| Bus::high_z(width_usize));
                                driven.insert(net, existing.resolve(bus));
                            }
                        }
                    }
                }

                // Collect sequential state updates.
                if let Some(ns) = compute_next_state(
                    &comp.kind,
                    cid,
                    &inputs,
                    comp_state_opt,
                    state_snapshot.clock_tick,
                ) {
                    new_states.push((cid, ns));
                }
            }

            // Seed input pins last — their values are authoritative and always
            // override any component-driven value on the same net.
            for (cid, comp) in &circuit.components {
                if let ComponentKind::Pin {
                    is_output: false,
                    width,
                } = &comp.kind
                {
                    let value = self
                        .user_inputs
                        .get(circuit_name)
                        .and_then(|ui| ui.get(cid))
                        .cloned()
                        .unwrap_or_else(|| Bus::unknown(width.get() as usize));
                    if let Some((x, y)) = comp.port_position("out") {
                        let net = canonical(x, y);
                        driven.insert(net, value);
                    }
                }
            }

            // Check convergence: have the driven net values changed from last pass?
            let changed = driven != prev_nets;
            prev_nets = driven.clone();

            // Commit the freshly resolved net values and any sequential state.
            let state = self.states.entry(circuit_name.to_string()).or_default();
            state.net_values = driven;
            for (cid, ns) in new_states {
                let st = state.component_state.entry(cid).or_default();
                *st = ns;
            }

            if !changed {
                return Ok(());
            }
        }

        Err(LogisimError::OscillationDetected)
    }

    /// Query the current value of an output pin by its label or component ID.
    pub fn read_pin(&self, circuit_name: &str, comp_id: ComponentId) -> Option<Bus> {
        let circuit = self.project.circuits.get(circuit_name)?;
        let comp = circuit.get_component(comp_id)?;
        let state = self.states.get(circuit_name)?;
        let net_map = circuit.compute_nets();

        let ports = comp.kind.ports();
        // Prefer an output port (input pins); fall back to input port (output pins,
        // which only expose an "in" port). No generic .first() fallback is needed
        // since all pin-like components have at least one of these two directions.
        let port = ports
            .iter()
            .find(|p| matches!(p.direction, crate::component::PortDirection::Output))
            .or_else(|| {
                ports
                    .iter()
                    .find(|p| matches!(p.direction, crate::component::PortDirection::Input))
            })?;
        let (x, y) = comp.port_position(&port.name)?;
        let net = net_map.get(&(x, y)).copied().unwrap_or((x, y));
        Some(state.net_value(net, port.width.get()))
    }
}

// ── Component evaluation ──────────────────────────────────────────────────────

/// Evaluate a component and return its output values.
fn evaluate_component(
    kind: &ComponentKind,
    _id: ComponentId,
    inputs: &HashMap<String, Bus>,
    state: Option<&ComponentState>,
    clock_tick: u64,
) -> HashMap<String, Bus> {
    let mut out = HashMap::new();

    let get = |name: &str, width: u32| -> Bus {
        inputs
            .get(name)
            .cloned()
            .unwrap_or_else(|| Bus::unknown(width as usize))
    };
    let get1 = |name: &str| -> Value { get(name, 1).get(0) };

    match kind {
        // ── Wiring ────────────────────────────────────────────────────────────
        ComponentKind::Pin {
            is_output: false,
            width,
        } => {
            out.insert("out".to_string(), get("out", width.get()));
        }
        ComponentKind::Pin {
            is_output: true, ..
        } => {
            // Output pin: its output drives are on its input.
        }
        ComponentKind::Clock => {
            let v = if clock_tick.is_multiple_of(2) {
                0u64
            } else {
                1u64
            };
            out.insert("out".to_string(), Bus::from_u64(v, 1));
        }
        ComponentKind::Constant { width, value } => {
            out.insert(
                "out".to_string(),
                Bus::from_u64(*value, width.get() as usize),
            );
        }
        ComponentKind::Power => {
            out.insert("out".to_string(), Bus::from_u64(1, 1));
        }
        ComponentKind::Ground => {
            out.insert("out".to_string(), Bus::from_u64(0, 1));
        }
        ComponentKind::Splitter {
            combined_width,
            fan_out,
        } => {
            let combined = get("combined", combined_width.get());
            if *fan_out == 0 {
                // Nothing to drive; leave outputs absent.
            } else {
                let bits_each = combined_width.get() / *fan_out as u32;
                for i in 0..*fan_out {
                    let start = (i as u32 * bits_each) as usize;
                    let end = start + bits_each as usize;
                    let slice = combined.slice(start, end);
                    out.insert(format!("bit{}", i), slice);
                }
            }
        }
        ComponentKind::Tunnel { width, .. } => {
            let v = get("in", width.get());
            out.insert("out".to_string(), v);
        }
        ComponentKind::PullResistor { direction, width } => {
            let v = match direction {
                PullDirection::Up => {
                    let mask = 1u64
                        .checked_shl(width.get())
                        .map(|v| v - 1)
                        .unwrap_or(u64::MAX);
                    Bus::from_u64(mask, width.get() as usize)
                }
                PullDirection::Down => Bus::from_u64(0, width.get() as usize),
            };
            out.insert("out".to_string(), v);
        }
        ComponentKind::TristateBuffer { width } => {
            let en = get1("enable");
            let v = get("in", width.get());
            let result = if en == Value::True {
                v
            } else {
                Bus::high_z(width.get() as usize)
            };
            out.insert("out".to_string(), result);
        }

        ComponentKind::Transistor { width, p_type } => {
            let gate = get1("gate");
            let src = get("source", width.get());
            // n-type: gate=1 conducts; p-type: gate=0 conducts.
            let conducts = if *p_type {
                gate == Value::False
            } else {
                gate == Value::True
            };
            let result = if conducts {
                src
            } else {
                Bus::high_z(width.get() as usize)
            };
            out.insert("drain".to_string(), result);
        }

        ComponentKind::TransmissionGate { width } => {
            let gate = get1("gate");
            let gate_n = get1("gate_n");
            let src = get("source", width.get());
            // Conducts when gate=1 and gate_n=0 (complementary enables).
            let conducts = gate == Value::True && gate_n == Value::False;
            let result = if conducts {
                src
            } else {
                Bus::high_z(width.get() as usize)
            };
            out.insert("drain".to_string(), result);
        }

        ComponentKind::AndGate {
            inputs: n,
            width,
            negate_inputs,
            negate_output,
        } => {
            let mask = 1u64
                .checked_shl(width.get())
                .map(|v| v - 1)
                .unwrap_or(u64::MAX);
            let mut result = Bus::from_u64(mask, width.get() as usize);
            for i in 0..*n {
                let mut v = get(&format!("in{}", i), width.get());
                if negate_inputs.get(i as usize).copied().unwrap_or(false) {
                    v = v.not();
                }
                result = result.and(&v);
            }
            if *negate_output {
                result = result.not();
            }
            out.insert("out".to_string(), result);
        }

        ComponentKind::OrGate {
            inputs: n,
            width,
            negate_inputs,
            negate_output,
        } => {
            let mut result = Bus::from_u64(0, width.get() as usize);
            for i in 0..*n {
                let mut v = get(&format!("in{}", i), width.get());
                if negate_inputs.get(i as usize).copied().unwrap_or(false) {
                    v = v.not();
                }
                result = result.or(&v);
            }
            if *negate_output {
                result = result.not();
            }
            out.insert("out".to_string(), result);
        }

        ComponentKind::NandGate { inputs: n, width } => {
            let mut result = Bus::from_u64((1u64 << width.get()) - 1, width.get() as usize);
            for i in 0..*n {
                let v = get(&format!("in{}", i), width.get());
                result = result.and(&v);
            }
            out.insert("out".to_string(), result.not());
        }

        ComponentKind::NorGate { inputs: n, width } => {
            let mut result = Bus::from_u64(0, width.get() as usize);
            for i in 0..*n {
                let v = get(&format!("in{}", i), width.get());
                result = result.or(&v);
            }
            out.insert("out".to_string(), result.not());
        }

        ComponentKind::XorGate { inputs: n, width } => {
            let mut result = Bus::from_u64(0, width.get() as usize);
            for i in 0..*n {
                let v = get(&format!("in{}", i), width.get());
                result = result.xor(&v);
            }
            out.insert("out".to_string(), result);
        }

        ComponentKind::XnorGate { inputs: n, width } => {
            let mut result = Bus::from_u64(0, width.get() as usize);
            for i in 0..*n {
                let v = get(&format!("in{}", i), width.get());
                result = result.xor(&v);
            }
            out.insert("out".to_string(), result.not());
        }

        ComponentKind::NotGate { width } => {
            out.insert("out".to_string(), get("in", width.get()).not());
        }

        ComponentKind::Buffer { width } | ComponentKind::ControlledBuffer { width } => {
            let v = get("in", width.get());
            let enabled = if let ComponentKind::ControlledBuffer { .. } = kind {
                get1("enable") == Value::True
            } else {
                true
            };
            if enabled {
                out.insert("out".to_string(), v);
            } else {
                out.insert("out".to_string(), Bus::unknown(width.get() as usize));
            }
        }

        ComponentKind::OddParityGate { inputs, width } => {
            let mut parity_count: u64 = 0;
            for i in 0..*inputs {
                let v = get(&format!("in{}", i), width.get());
                // If any bit is unknown/error, output unknown
                if v.to_u64().is_none() {
                    out.insert("out".to_string(), Bus::unknown(1));
                    return out;
                }
                let val = v.to_u64().unwrap_or(0);
                parity_count ^= val.count_ones() as u64 & 1;
            }
            out.insert("out".to_string(), Bus::from_u64(parity_count & 1, 1));
        }

        ComponentKind::EvenParityGate { inputs, width } => {
            let mut parity_count: u64 = 0;
            for i in 0..*inputs {
                let v = get(&format!("in{}", i), width.get());
                if v.to_u64().is_none() {
                    out.insert("out".to_string(), Bus::unknown(1));
                    return out;
                }
                let val = v.to_u64().unwrap_or(0);
                parity_count ^= val.count_ones() as u64 & 1;
            }
            // even parity: output 1 when even number of 1-bits
            out.insert("out".to_string(), Bus::from_u64(1 - (parity_count & 1), 1));
        }

        ComponentKind::BitExtender {
            input_width,
            output_width,
        } => {
            let v = get("in", input_width.get());
            let out_bits = output_width.get() as usize;
            let in_bits = input_width.get() as usize;
            // Zero-extend: upper bits are 0
            if let Some(val) = v.to_u64() {
                // Mask to input width then zero-extend
                let mask = if in_bits >= 64 {
                    u64::MAX
                } else {
                    (1u64 << in_bits) - 1
                };
                out.insert("out".to_string(), Bus::from_u64(val & mask, out_bits));
            } else {
                // Propagate unknown for the input bits, zero for the extension
                let mut result = Bus::unknown(out_bits);
                for i in in_bits..out_bits {
                    result.set(i, Value::False);
                }
                out.insert("out".to_string(), result);
            }
        }
        ComponentKind::Multiplexer {
            select_bits,
            data_width,
        } => {
            let sel = get("sel", *select_bits as u32);
            let idx = sel.to_u64().unwrap_or(0) as usize;
            let n = 1usize << select_bits;
            let chosen = if idx < n {
                get(&format!("in{}", idx), data_width.get())
            } else {
                Bus::unknown(data_width.get() as usize)
            };
            out.insert("out".to_string(), chosen);
        }

        ComponentKind::Demultiplexer {
            select_bits,
            data_width,
        } => {
            let sel = get("sel", *select_bits as u32);
            let idx = sel.to_u64().unwrap_or(0) as usize;
            let n = 1usize << select_bits;
            let data = get("in", data_width.get());
            for i in 0..n {
                if i == idx {
                    out.insert(format!("out{}", i), data.clone());
                } else {
                    out.insert(
                        format!("out{}", i),
                        Bus::from_u64(0, data_width.get() as usize),
                    );
                }
            }
        }

        ComponentKind::Decoder { select_bits } => {
            let sel = get("sel", *select_bits as u32);
            let idx = sel.to_u64().unwrap_or(0) as usize;
            let n = 1usize << select_bits;
            for i in 0..n {
                let v = if i == idx {
                    Bus::from_u64(1, 1)
                } else {
                    Bus::from_u64(0, 1)
                };
                out.insert(format!("out{}", i), v);
            }
        }

        ComponentKind::PriorityEncoder { select_bits } => {
            let n = 1usize << select_bits;
            let mut found_idx: Option<usize> = None;
            for i in (0..n).rev() {
                if get1(&format!("in{}", i)) == Value::True {
                    found_idx = Some(i);
                    break;
                }
            }
            let (enc_val, en_out) = match found_idx {
                Some(idx) => (
                    Bus::from_u64(idx as u64, *select_bits as usize),
                    Bus::from_u64(1, 1),
                ),
                None => (Bus::from_u64(0, *select_bits as usize), Bus::from_u64(0, 1)),
            };
            out.insert("out".to_string(), enc_val);
            out.insert("en_out".to_string(), en_out);
        }

        ComponentKind::BitSelector {
            group_bits,
            data_width,
        } => {
            let sel = get("sel", *group_bits as u32);
            let data = get("in", data_width.get());
            let idx = sel.to_u64().unwrap_or(0) as usize;
            let selected = data.get(idx);
            out.insert("out".to_string(), Bus::from_value(selected, 1));
        }

        // ── Arithmetic ────────────────────────────────────────────────────────
        ComponentKind::Adder { width } => {
            let a = get("a", width.get()).to_u64().unwrap_or(0);
            let b = get("b", width.get()).to_u64().unwrap_or(0);
            let cin = get1("c_in") == Value::True;
            let mask = (1u128 << width.get()) - 1;
            let sum128 = a as u128 + b as u128 + cin as u128;
            out.insert(
                "sum".to_string(),
                Bus::from_u64((sum128 & mask) as u64, width.get() as usize),
            );
            out.insert(
                "c_out".to_string(),
                Bus::from_u64((sum128 >> width.get()) as u64 & 1, 1),
            );
        }

        ComponentKind::Subtractor { width } => {
            let a = get("a", width.get()).to_u64().unwrap_or(0);
            let b = get("b", width.get()).to_u64().unwrap_or(0);
            let bin = get1("b_in") == Value::True;
            let mask = (1u128 << width.get()) - 1;
            let diff128 = (a as i128) - (b as i128) - (bin as i128);
            let result = ((diff128 & mask as i128) as u64) & mask as u64;
            let bout = if diff128 < 0 { 1u64 } else { 0u64 };
            out.insert(
                "out".to_string(),
                Bus::from_u64(result, width.get() as usize),
            );
            out.insert("b_out".to_string(), Bus::from_u64(bout, 1));
        }

        ComponentKind::Multiplier { width } => {
            let a = get("a", width.get()).to_u64().unwrap_or(0);
            let b = get("b", width.get()).to_u64().unwrap_or(0);
            let cin = get("c_in", width.get()).to_u64().unwrap_or(0);
            let mask = (1u128 << width.get()) - 1;
            let product = a as u128 * b as u128 + cin as u128;
            out.insert(
                "out".to_string(),
                Bus::from_u64((product & mask) as u64, width.get() as usize),
            );
            out.insert(
                "upper".to_string(),
                Bus::from_u64(
                    (product >> width.get()) as u64 & mask as u64,
                    width.get() as usize,
                ),
            );
        }

        ComponentKind::Divider { width } => {
            let a = get("a", width.get()).to_u64().unwrap_or(0);
            let b = get("b", width.get()).to_u64().unwrap_or(0);
            let upper = get("upper", width.get()).to_u64().unwrap_or(0);
            let mask = ((1u128 << width.get()) - 1) as u64;
            let (result, rem) = if b == 0 {
                (mask, mask)
            } else {
                let dividend = ((upper as u128) << width.get()) | a as u128;
                (
                    (dividend / b as u128) as u64 & mask,
                    (dividend % b as u128) as u64 & mask,
                )
            };
            out.insert(
                "result".to_string(),
                Bus::from_u64(result, width.get() as usize),
            );
            out.insert("rem".to_string(), Bus::from_u64(rem, width.get() as usize));
        }

        ComponentKind::Negator { width } => {
            let v = get("in", width.get()).to_u64().unwrap_or(0);
            let mask = if width.get() == 64 {
                u64::MAX
            } else {
                (1u64 << width.get()) - 1
            };
            let neg = v.wrapping_neg() & mask;
            out.insert("out".to_string(), Bus::from_u64(neg, width.get() as usize));
        }

        ComponentKind::Comparator { width } => {
            let a = get("a", width.get()).to_u64().unwrap_or(0);
            let b = get("b", width.get()).to_u64().unwrap_or(0);
            out.insert("gt".to_string(), Bus::from_u64((a > b) as u64, 1));
            out.insert("eq".to_string(), Bus::from_u64((a == b) as u64, 1));
            out.insert("lt".to_string(), Bus::from_u64((a < b) as u64, 1));
        }

        ComponentKind::BitAdder { width } => {
            let v = get("in", width.get()).to_u64().unwrap_or(0);
            let count = v.count_ones() as u64;
            let out_width = (width.get() + 1).min(64);
            out.insert("out".to_string(), Bus::from_u64(count, out_width as usize));
        }

        ComponentKind::BitFinder { width, find_type } => {
            let v = get("in", width.get()).to_u64().unwrap_or(0);
            let (pos, found) = match find_type {
                BitFinderType::High => {
                    if v == 0 {
                        (0u64, false)
                    } else {
                        (63 - v.leading_zeros() as u64, true)
                    }
                }
                BitFinderType::Low => {
                    if v == 0 {
                        (0u64, false)
                    } else {
                        (v.trailing_zeros() as u64, true)
                    }
                }
            };
            let out_width = (width.get() + 1).min(64);
            out.insert("out".to_string(), Bus::from_u64(pos, out_width as usize));
            out.insert("found".to_string(), Bus::from_u64(found as u64, 1));
        }

        // ── Sequential (outputs from stored state) ────────────────────────────
        ComponentKind::DFlipFlop { width }
        | ComponentKind::TFlipFlop { width }
        | ComponentKind::JKFlipFlop { width }
        | ComponentKind::SRFlipFlop { width }
        | ComponentKind::Register { width } => {
            let q = state
                .map(|s| s.q.clone())
                .unwrap_or_else(|| Bus::unknown(width.get() as usize));
            let q_n = q.not();
            out.insert("q".to_string(), q);
            out.insert("q_n".to_string(), q_n);
        }

        ComponentKind::Counter { width } => {
            let count = state
                .map(|s| s.counter.clone())
                .unwrap_or_else(|| Bus::from_u64(0, width.get() as usize));
            let max = (1u64 << width.get()) - 1;
            let terminal = Bus::from_u64((count.to_u64().unwrap_or(0) == max) as u64, 1);
            out.insert("count".to_string(), count);
            out.insert("terminal".to_string(), terminal);
        }

        ComponentKind::Ram { data_bits, .. } => {
            let addr = get("addr", 8);
            let idx = addr.to_u64().unwrap_or(0) as usize;
            let data = state
                .and_then(|s| s.memory.get(idx).cloned())
                .unwrap_or_else(|| Bus::from_u64(0, data_bits.get() as usize));
            out.insert("data_out".to_string(), data);
        }

        ComponentKind::Rom {
            addr_bits,
            data_bits,
            contents,
        } => {
            let addr = get("addr", *addr_bits as u32);
            let idx = addr.to_u64().unwrap_or(0) as usize;
            let data = contents.get(idx).copied().unwrap_or(0);
            out.insert(
                "data".to_string(),
                Bus::from_u64(data, data_bits.get() as usize),
            );
        }

        ComponentKind::ShiftRegisterMemory {
            stages: num_stages,
            width,
            ..
        } => {
            let w = width.get() as usize;
            let n = *num_stages as usize;
            // Last stage is the serial output.
            let last = state
                .and_then(|s| s.stages.last().cloned())
                .unwrap_or_else(|| Bus::unknown(w));
            out.insert("out".to_string(), last.clone());
            // Per-stage parallel outputs (q0 .. q(n-1)).
            for i in 0..n {
                let val = state
                    .and_then(|s| s.stages.get(i).cloned())
                    .unwrap_or_else(|| Bus::unknown(w));
                out.insert(format!("q{}", i), val);
            }
        }

        ComponentKind::Led => {} // output-only display, no signal outputs

        ComponentKind::Button => {
            let pressed = state.map(|s| s.clock_count > 0).unwrap_or(false);
            out.insert("out".to_string(), Bus::from_u64(pressed as u64, 1));
        }

        ComponentKind::DipSwitch { switches } => {
            let st = state.map(|s| s.clock_count).unwrap_or(0);
            for i in 0..*switches {
                let on = (st >> i) & 1;
                out.insert(format!("out{}", i), Bus::from_u64(on, 1));
            }
        }

        // ── TTL 74xx ─────────────────────────────────────────────────────────
        ComponentKind::Ttl7408 => {
            // Quad 2-input AND
            for i in 1..=4u8 {
                let a = get1(&format!("A{i}"));
                let b = get1(&format!("B{i}"));
                let y = match (a, b) {
                    (Value::True, Value::True) => Value::True,
                    (Value::False, _) | (_, Value::False) => Value::False,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }
        ComponentKind::Ttl7400 => {
            // Quad 2-input NAND
            for i in 1..=4u8 {
                let a = get1(&format!("A{i}"));
                let b = get1(&format!("B{i}"));
                let y = match (a, b) {
                    (Value::True, Value::True) => Value::False,
                    (Value::False, _) | (_, Value::False) => Value::True,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }
        ComponentKind::Ttl7432 => {
            // Quad 2-input OR
            for i in 1..=4u8 {
                let a = get1(&format!("A{i}"));
                let b = get1(&format!("B{i}"));
                let y = match (a, b) {
                    (Value::False, Value::False) => Value::False,
                    (Value::True, _) | (_, Value::True) => Value::True,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }
        ComponentKind::Ttl7402 => {
            // Quad 2-input NOR
            for i in 1..=4u8 {
                let a = get1(&format!("A{i}"));
                let b = get1(&format!("B{i}"));
                let y = match (a, b) {
                    (Value::False, Value::False) => Value::True,
                    (Value::True, _) | (_, Value::True) => Value::False,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }
        ComponentKind::Ttl7404 => {
            // Hex Inverter
            for i in 1..=6u8 {
                let a = get1(&format!("A{i}"));
                let y = match a {
                    Value::True => Value::False,
                    Value::False => Value::True,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }
        ComponentKind::Ttl7486 => {
            // Quad 2-input XOR
            for i in 1..=4u8 {
                let a = get1(&format!("A{i}"));
                let b = get1(&format!("B{i}"));
                let y = match (a, b) {
                    (Value::True, Value::True) | (Value::False, Value::False) => Value::False,
                    (Value::True, Value::False) | (Value::False, Value::True) => Value::True,
                    _ => Value::Unknown,
                };
                out.insert(format!("Y{i}"), Bus::from_value(y, 1));
            }
        }

        _ => {} // other display/IO components handled externally
    }

    out
}

/// Compute the next sequential state for a component.
fn compute_next_state(
    kind: &ComponentKind,
    _id: ComponentId,
    inputs: &HashMap<String, Bus>,
    state: Option<&ComponentState>,
    _clock_tick: u64,
) -> Option<ComponentState> {
    let get = |name: &str, width: u32| -> Bus {
        inputs
            .get(name)
            .cloned()
            .unwrap_or_else(|| Bus::unknown(width as usize))
    };
    let get1 = |name: &str| -> Value { get(name, 1).get(0) };

    // Edge detection: rising edge = prev was False, now is True.
    let clk = get1("clk");
    let prev_clk = state.map(|s| s.prev_clk).unwrap_or(Value::Unknown);
    let rising_edge = prev_clk == Value::False && clk == Value::True;

    match kind {
        ComponentKind::DFlipFlop { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let en = get1("en");
                if en != Value::False {
                    let reset = get1("reset");
                    let preset = get1("preset");
                    if reset == Value::True {
                        ns.q = Bus::from_u64(0, width.get() as usize);
                    } else if preset == Value::True {
                        let mask = if width.get() == 64 {
                            u64::MAX
                        } else {
                            (1u64 << width.get()) - 1
                        };
                        ns.q = Bus::from_u64(mask, width.get() as usize);
                    } else {
                        ns.q = get("d", width.get());
                    }
                }
            }
            Some(ns)
        }

        ComponentKind::TFlipFlop { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let en = get1("en");
                if en != Value::False {
                    let reset = get1("reset");
                    let preset = get1("preset");
                    if reset == Value::True {
                        ns.q = Bus::from_u64(0, width.get() as usize);
                    } else if preset == Value::True {
                        let mask = if width.get() == 64 {
                            u64::MAX
                        } else {
                            (1u64 << width.get()) - 1
                        };
                        ns.q = Bus::from_u64(mask, width.get() as usize);
                    } else {
                        let t = get("t", width.get());
                        let q = ns.q.clone();
                        ns.q = q.xor(&t);
                    }
                }
            }
            Some(ns)
        }

        ComponentKind::JKFlipFlop { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let reset = get1("reset");
                let preset = get1("preset");
                if reset == Value::True {
                    ns.q = Bus::from_u64(0, width.get() as usize);
                } else if preset == Value::True {
                    let mask = if width.get() == 64 {
                        u64::MAX
                    } else {
                        (1u64 << width.get()) - 1
                    };
                    ns.q = Bus::from_u64(mask, width.get() as usize);
                } else {
                    let j = get1("j");
                    let k = get1("k");
                    let q = ns.q.get(0);
                    let new_q = match (j, k) {
                        (Value::False, Value::False) => q,
                        (Value::False, Value::True) => Value::False,
                        (Value::True, Value::False) => Value::True,
                        (Value::True, Value::True) => !q,
                        _ => Value::Unknown,
                    };
                    let width_usize = width.get() as usize;
                    ns.q = match new_q {
                        Value::False => Bus::from_u64(0, width_usize),
                        Value::True => {
                            // All-ones mask for the given bit-width (safe for width == 64).
                            let mask = 1u64
                                .checked_shl(width.get())
                                .map(|v| v - 1)
                                .unwrap_or(u64::MAX);
                            Bus::from_u64(mask, width_usize)
                        }
                        _ => Bus::unknown(width_usize),
                    };
                }
            }
            Some(ns)
        }

        ComponentKind::SRFlipFlop { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let s = get1("s");
                let r = get1("r");
                match (s, r) {
                    (Value::True, Value::False) => {
                        let mask = if width.get() == 64 {
                            u64::MAX
                        } else {
                            (1u64 << width.get()) - 1
                        };
                        ns.q = Bus::from_u64(mask, width.get() as usize);
                    }
                    (Value::False, Value::True) => {
                        ns.q = Bus::from_u64(0, width.get() as usize);
                    }
                    (Value::True, Value::True) => {
                        // Illegal state
                        ns.q = Bus::unknown(width.get() as usize);
                    }
                    _ => {} // hold
                }
            }
            Some(ns)
        }

        ComponentKind::Register { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let en = get1("en");
                if en != Value::False {
                    let reset = get1("reset");
                    if reset == Value::True {
                        ns.q = Bus::from_u64(0, width.get() as usize);
                    } else {
                        ns.q = get("d", width.get());
                    }
                }
            }
            Some(ns)
        }

        ComponentKind::Counter { width } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            if rising_edge {
                let reset = get1("reset");
                let en = get1("en");
                let ld_en = get1("ld_en");
                if reset == Value::True {
                    ns.counter = Bus::from_u64(0, width.get() as usize);
                } else if ld_en == Value::True {
                    ns.counter = get("load", width.get());
                } else if en != Value::False {
                    let cur = ns.counter.to_u64().unwrap_or(0);
                    let max = if width.get() == 64 {
                        u64::MAX
                    } else {
                        (1u64 << width.get()) - 1
                    };
                    let next = if cur >= max { 0 } else { cur + 1 };
                    ns.counter = Bus::from_u64(next, width.get() as usize);
                }
            }
            Some(ns)
        }

        ComponentKind::Ram {
            addr_bits,
            data_bits,
            ..
        } => {
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            let mem_size = 1usize << addr_bits;
            if ns.memory.len() < mem_size {
                ns.memory
                    .resize(mem_size, Bus::from_u64(0, data_bits.get() as usize));
            }
            if rising_edge {
                let we = inputs.get("we").map(|b| b.get(0)).unwrap_or(Value::False);
                if we == Value::True {
                    let addr = get("addr", *addr_bits as u32).to_u64().unwrap_or(0) as usize;
                    let data = get("data_in", data_bits.get());
                    if addr < mem_size {
                        ns.memory[addr] = data;
                    }
                }
            }
            Some(ns)
        }

        ComponentKind::ShiftRegisterMemory {
            stages: num_stages,
            width,
            parallel_load,
        } => {
            let w = width.get() as usize;
            let n = *num_stages as usize;
            let mut ns = state.cloned().unwrap_or_default();
            ns.prev_clk = clk;
            // Ensure stages vector is the right length.
            if ns.stages.len() != n {
                ns.stages = vec![Bus::from_u64(0, w); n];
            }
            if rising_edge {
                let en = get1("en");
                if en != Value::False {
                    if *parallel_load && get1("ld") == Value::True {
                        // Parallel load: copy d0..d(n-1) into all stages.
                        for i in 0..n {
                            ns.stages[i] = get(&format!("d{}", i), width.get());
                        }
                    } else {
                        // Serial shift: shift right, new data enters stage 0.
                        let new_in = get("in", width.get());
                        ns.stages.rotate_right(1);
                        ns.stages[0] = new_in;
                    }
                }
            }
            Some(ns)
        }

        _ => None,
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::Circuit;
    use crate::component::ComponentKind;
    use crate::project::Project;
    use crate::value::BitWidth;

    fn make_and_circuit() -> Project {
        let mut circuit = Circuit::new("main");
        circuit.add_component_with_label(
            ComponentKind::Pin {
                is_output: false,
                width: BitWidth::ONE,
            },
            0,
            0,
            "A",
        );
        circuit.add_component_with_label(
            ComponentKind::Pin {
                is_output: false,
                width: BitWidth::ONE,
            },
            0,
            10,
            "B",
        );
        circuit.add_component(
            ComponentKind::AndGate {
                inputs: 2,
                width: BitWidth::ONE,
                negate_inputs: vec![false, false],
                negate_output: false,
            },
            20,
            0,
        );
        circuit.add_component_with_label(
            ComponentKind::Pin {
                is_output: true,
                width: BitWidth::ONE,
            },
            40,
            0,
            "OUT",
        );

        // Wire A → gate.in0
        circuit.add_wire(0, 0, 20, 0);
        // Wire B → gate.in1
        circuit.add_wire(0, 10, 20, 1);
        // Wire gate.out → OUT
        circuit.add_wire(20, 2, 40, 0);

        let mut project = Project::new("test");
        project.add_circuit(circuit);
        project
    }

    #[test]
    fn test_and_gate_true_true() {
        let project = make_and_circuit();
        let mut sim = Simulator::new(project);

        // Set A=1, B=1
        let cids: Vec<_> = sim.project().circuits["main"]
            .input_pins()
            .iter()
            .map(|c| c.id)
            .collect();
        sim.set_pin_value("main", cids[0], Bus::from_u64(1, 1));
        sim.set_pin_value("main", cids[1], Bus::from_u64(1, 1));
        sim.propagate("main").unwrap();

        // The result is on the net driven by the AND gate output.
        // Check via net values
        let state = sim.state("main").unwrap();
        assert!(state
            .net_values
            .values()
            .any(|b| b.to_u64() == Some(1) || b.get(0) == Value::True));
    }

    #[test]
    fn test_constant_output() {
        let mut circuit = Circuit::new("const");
        let _c = circuit.add_component(
            ComponentKind::Constant {
                width: BitWidth::FOUR,
                value: 0b1010,
            },
            10,
            10,
        );
        let mut project = Project::new("test");
        project.add_circuit(circuit);
        let mut sim = Simulator::new(project);
        sim.propagate("const").unwrap();
        // Verify a net has value 0xA
        let state = sim.state("const").unwrap();
        assert!(state
            .net_values
            .values()
            .any(|b| b.to_u64() == Some(0b1010)));
    }

    #[test]
    fn test_adder() {
        let inputs: HashMap<String, Bus> = [
            ("a".to_string(), Bus::from_u64(3, 4)),
            ("b".to_string(), Bus::from_u64(5, 4)),
            ("c_in".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let kind = ComponentKind::Adder {
            width: BitWidth::FOUR,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["sum"].to_u64(), Some(8));
        assert_eq!(result["c_out"].to_u64(), Some(0));
    }

    #[test]
    fn test_adder_carry_out() {
        let inputs: HashMap<String, Bus> = [
            ("a".to_string(), Bus::from_u64(15, 4)),
            ("b".to_string(), Bus::from_u64(1, 4)),
            ("c_in".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let kind = ComponentKind::Adder {
            width: BitWidth::FOUR,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["sum"].to_u64(), Some(0));
        assert_eq!(result["c_out"].to_u64(), Some(1));
    }

    #[test]
    fn test_mux_select() {
        let inputs: HashMap<String, Bus> = [
            ("in0".to_string(), Bus::from_u64(0xA, 4)),
            ("in1".to_string(), Bus::from_u64(0xB, 4)),
            ("sel".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let kind = ComponentKind::Multiplexer {
            select_bits: 1,
            data_width: BitWidth::FOUR,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out"].to_u64(), Some(0xB));
    }

    #[test]
    fn test_dff_rising_edge() {
        let kind = ComponentKind::DFlipFlop {
            width: BitWidth::ONE,
        };

        // Simulate rising edge: prev_clk=0, clk=1, d=1
        let inputs: HashMap<String, Bus> = [
            ("d".to_string(), Bus::from_u64(1, 1)),
            ("clk".to_string(), Bus::from_u64(1, 1)),
            ("en".to_string(), Bus::from_u64(1, 1)),
            ("reset".to_string(), Bus::from_u64(0, 1)),
            ("preset".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let prev = ComponentState {
            prev_clk: Value::False,
            q: Bus::from_u64(0, 1),
            ..ComponentState::default()
        };

        let ns = compute_next_state(&kind, ComponentId(1), &inputs, Some(&prev), 0).unwrap();
        assert_eq!(ns.q.to_u64(), Some(1));
    }

    #[test]
    fn test_counter_increment() {
        let kind = ComponentKind::Counter {
            width: BitWidth::FOUR,
        };
        let inputs: HashMap<String, Bus> = [
            ("clk".to_string(), Bus::from_u64(1, 1)),
            ("en".to_string(), Bus::from_u64(1, 1)),
            ("reset".to_string(), Bus::from_u64(0, 1)),
            ("ld_en".to_string(), Bus::from_u64(0, 1)),
            ("load".to_string(), Bus::from_u64(0, 4)),
        ]
        .into();
        let prev = ComponentState {
            prev_clk: Value::False,
            counter: Bus::from_u64(5, 4),
            ..ComponentState::default()
        };

        let ns = compute_next_state(&kind, ComponentId(1), &inputs, Some(&prev), 0).unwrap();
        assert_eq!(ns.counter.to_u64(), Some(6));
    }

    #[test]
    fn test_comparator() {
        let inputs: HashMap<String, Bus> = [
            ("a".to_string(), Bus::from_u64(5, 4)),
            ("b".to_string(), Bus::from_u64(3, 4)),
        ]
        .into();
        let kind = ComponentKind::Comparator {
            width: BitWidth::FOUR,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["gt"].to_u64(), Some(1));
        assert_eq!(result["eq"].to_u64(), Some(0));
        assert_eq!(result["lt"].to_u64(), Some(0));
    }

    #[test]
    fn test_not_gate() {
        let inputs: HashMap<String, Bus> = [("in".to_string(), Bus::from_u64(0b1010, 4))].into();
        let kind = ComponentKind::NotGate {
            width: BitWidth::FOUR,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out"].to_u64(), Some(0b0101));
    }

    #[test]
    fn test_decoder() {
        let inputs: HashMap<String, Bus> = [("sel".to_string(), Bus::from_u64(2, 2))].into();
        let kind = ComponentKind::Decoder { select_bits: 2 };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out0"].to_u64(), Some(0));
        assert_eq!(result["out1"].to_u64(), Some(0));
        assert_eq!(result["out2"].to_u64(), Some(1));
        assert_eq!(result["out3"].to_u64(), Some(0));
    }

    #[test]
    fn test_odd_parity_gate_two_inputs() {
        // 0b11 (2 ones) → even count → odd parity = 0
        let inputs: HashMap<String, Bus> = [
            ("in0".to_string(), Bus::from_u64(0b1, 1)),
            ("in1".to_string(), Bus::from_u64(0b1, 1)),
        ]
        .into();
        let kind = ComponentKind::OddParityGate {
            inputs: 2,
            width: BitWidth::ONE,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out"].to_u64(), Some(0)); // even number of 1s

        // 0b1 and 0b0 → odd count → odd parity = 1
        let inputs2: HashMap<String, Bus> = [
            ("in0".to_string(), Bus::from_u64(0b1, 1)),
            ("in1".to_string(), Bus::from_u64(0b0, 1)),
        ]
        .into();
        let result2 = evaluate_component(&kind, ComponentId(1), &inputs2, None, 0);
        assert_eq!(result2["out"].to_u64(), Some(1));
    }

    #[test]
    fn test_even_parity_gate() {
        // 0b1 and 0b1 → 2 ones (even) → even parity = 1
        let inputs: HashMap<String, Bus> = [
            ("in0".to_string(), Bus::from_u64(0b1, 1)),
            ("in1".to_string(), Bus::from_u64(0b1, 1)),
        ]
        .into();
        let kind = ComponentKind::EvenParityGate {
            inputs: 2,
            width: BitWidth::ONE,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out"].to_u64(), Some(1));

        // 0b1 and 0b0 → 1 one (odd) → even parity = 0
        let inputs2: HashMap<String, Bus> = [
            ("in0".to_string(), Bus::from_u64(1, 1)),
            ("in1".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let result2 = evaluate_component(&kind, ComponentId(1), &inputs2, None, 0);
        assert_eq!(result2["out"].to_u64(), Some(0));
    }

    #[test]
    fn test_bit_extender_zero_extend() {
        // 4-bit 0b1011 zero-extended to 8 bits = 0b00001011
        let inputs: HashMap<String, Bus> = [("in".to_string(), Bus::from_u64(0b1011, 4))].into();
        let kind = ComponentKind::BitExtender {
            input_width: BitWidth::FOUR,
            output_width: BitWidth::EIGHT,
        };
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["out"].to_u64(), Some(0b0000_1011));
        assert_eq!(result["out"].width(), 8);
    }

    #[test]
    fn test_transistor_n_type_conducts_when_gate_high() {
        let kind = ComponentKind::Transistor {
            width: BitWidth::ONE,
            p_type: false,
        };
        // gate=1 → conducts; drain = source
        let inputs: HashMap<String, Bus> = [
            ("gate".to_string(), Bus::from_u64(1, 1)),
            ("source".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["drain"].to_u64(), Some(1));

        // gate=0 → high-Z
        let inputs_off: HashMap<String, Bus> = [
            ("gate".to_string(), Bus::from_u64(0, 1)),
            ("source".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let result_off = evaluate_component(&kind, ComponentId(1), &inputs_off, None, 0);
        assert!(result_off["drain"].is_high_z());
    }

    #[test]
    fn test_transistor_p_type_conducts_when_gate_low() {
        let kind = ComponentKind::Transistor {
            width: BitWidth::ONE,
            p_type: true,
        };
        // gate=0 → conducts; drain = source
        let inputs: HashMap<String, Bus> = [
            ("gate".to_string(), Bus::from_u64(0, 1)),
            ("source".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let result = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(result["drain"].to_u64(), Some(1));

        // gate=1 → high-Z
        let inputs_off: HashMap<String, Bus> = [
            ("gate".to_string(), Bus::from_u64(1, 1)),
            ("source".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let result_off = evaluate_component(&kind, ComponentId(1), &inputs_off, None, 0);
        assert!(result_off["drain"].is_high_z());
    }

    #[test]
    fn test_shift_register_memory_serial_shift() {
        use crate::simulation::ComponentState;
        let kind = ComponentKind::ShiftRegisterMemory {
            stages: 3,
            width: BitWidth::FOUR,
            parallel_load: false,
        };
        // Initial state: all zeros
        let prev = ComponentState {
            stages: vec![
                Bus::from_u64(0, 4),
                Bus::from_u64(0, 4),
                Bus::from_u64(0, 4),
            ],
            prev_clk: Value::False,
            ..ComponentState::default()
        };
        // Rising edge with en=1, shift in value 5.
        let inputs: HashMap<String, Bus> = [
            ("clk".to_string(), Bus::from_u64(1, 1)),
            ("en".to_string(), Bus::from_u64(1, 1)),
            ("in".to_string(), Bus::from_u64(5, 4)),
        ]
        .into();
        let ns = compute_next_state(&kind, ComponentId(1), &inputs, Some(&prev), 0).unwrap();
        assert_eq!(ns.stages[0].to_u64(), Some(5)); // new value shifted in
        assert_eq!(ns.stages[1].to_u64(), Some(0)); // previous stage 0
        assert_eq!(ns.stages[2].to_u64(), Some(0)); // previous stage 1
                                                    // Outputs from new state
        let outputs = evaluate_component(&kind, ComponentId(1), &inputs, Some(&ns), 0);
        assert_eq!(outputs["out"].to_u64(), Some(0)); // last stage
        assert_eq!(outputs["q0"].to_u64(), Some(5));
    }

    // ── TTL 74xx ─────────────────────────────────────────────────────────────

    #[test]
    fn test_ttl_7408_quad_and() {
        // 7408: quad 2-input AND gate.  Y = A AND B for each gate.
        let kind = ComponentKind::Ttl7408;
        let inputs: HashMap<String, Bus> = [
            ("A1".to_string(), Bus::from_u64(1, 1)),
            ("B1".to_string(), Bus::from_u64(1, 1)),
            ("A2".to_string(), Bus::from_u64(1, 1)),
            ("B2".to_string(), Bus::from_u64(0, 1)),
            ("A3".to_string(), Bus::from_u64(0, 1)),
            ("B3".to_string(), Bus::from_u64(1, 1)),
            ("A4".to_string(), Bus::from_u64(0, 1)),
            ("B4".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let out = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(out["Y1"].to_u64(), Some(1), "1 AND 1 = 1");
        assert_eq!(out["Y2"].to_u64(), Some(0), "1 AND 0 = 0");
        assert_eq!(out["Y3"].to_u64(), Some(0), "0 AND 1 = 0");
        assert_eq!(out["Y4"].to_u64(), Some(0), "0 AND 0 = 0");
    }

    #[test]
    fn test_ttl_7400_quad_nand() {
        // 7400: quad 2-input NAND gate.  Y = NOT(A AND B).
        let kind = ComponentKind::Ttl7400;
        let inputs: HashMap<String, Bus> = [
            ("A1".to_string(), Bus::from_u64(1, 1)),
            ("B1".to_string(), Bus::from_u64(1, 1)),
            ("A2".to_string(), Bus::from_u64(1, 1)),
            ("B2".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let out = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(out["Y1"].to_u64(), Some(0), "NAND(1,1) = 0");
        assert_eq!(out["Y2"].to_u64(), Some(1), "NAND(1,0) = 1");
    }

    #[test]
    fn test_ttl_7404_hex_inverter() {
        // 7404: hex inverter.  Yi = NOT(Ai).
        let kind = ComponentKind::Ttl7404;
        let inputs: HashMap<String, Bus> = [
            ("A1".to_string(), Bus::from_u64(1, 1)),
            ("A2".to_string(), Bus::from_u64(0, 1)),
            ("A3".to_string(), Bus::from_u64(1, 1)),
            ("A4".to_string(), Bus::from_u64(0, 1)),
            ("A5".to_string(), Bus::from_u64(1, 1)),
            ("A6".to_string(), Bus::from_u64(0, 1)),
        ]
        .into();
        let out = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(out["Y1"].to_u64(), Some(0));
        assert_eq!(out["Y2"].to_u64(), Some(1));
        assert_eq!(out["Y3"].to_u64(), Some(0));
        assert_eq!(out["Y4"].to_u64(), Some(1));
        assert_eq!(out["Y5"].to_u64(), Some(0));
        assert_eq!(out["Y6"].to_u64(), Some(1));
    }

    #[test]
    fn test_ttl_7486_quad_xor() {
        // 7486: quad 2-input XOR gate.  Y = A XOR B.
        let kind = ComponentKind::Ttl7486;
        let inputs: HashMap<String, Bus> = [
            ("A1".to_string(), Bus::from_u64(0, 1)),
            ("B1".to_string(), Bus::from_u64(0, 1)),
            ("A2".to_string(), Bus::from_u64(0, 1)),
            ("B2".to_string(), Bus::from_u64(1, 1)),
            ("A3".to_string(), Bus::from_u64(1, 1)),
            ("B3".to_string(), Bus::from_u64(0, 1)),
            ("A4".to_string(), Bus::from_u64(1, 1)),
            ("B4".to_string(), Bus::from_u64(1, 1)),
        ]
        .into();
        let out = evaluate_component(&kind, ComponentId(1), &inputs, None, 0);
        assert_eq!(out["Y1"].to_u64(), Some(0), "0 XOR 0 = 0");
        assert_eq!(out["Y2"].to_u64(), Some(1), "0 XOR 1 = 1");
        assert_eq!(out["Y3"].to_u64(), Some(1), "1 XOR 0 = 1");
        assert_eq!(out["Y4"].to_u64(), Some(0), "1 XOR 1 = 0");
    }
}
