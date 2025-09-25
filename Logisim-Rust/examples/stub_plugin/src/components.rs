//! Custom component implementations for the stub plugin
//!
//! This module contains the actual component implementations that demonstrate
//! how to create custom components for Logisim-RUST.
//!
//! # API Stability Warning  
//! These components use **UNSTABLE** APIs that may change without notice.

use logisim_core::*;
use std::collections::HashMap;

/// A custom XOR gate with enhanced features
/// 
/// This demonstrates a simple combinational logic component with debugging capabilities.
#[derive(Debug, Clone)]
pub struct CustomXor {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    debug_mode: bool,
    operation_count: u64,
}

impl CustomXor {
    /// Create a new custom XOR gate
    pub fn new(id: ComponentId) -> Self {
        let mut pins = HashMap::new();
        pins.insert("A".to_string(), Pin::new_input("A", BusWidth(1)));
        pins.insert("B".to_string(), Pin::new_input("B", BusWidth(1)));
        pins.insert("Y".to_string(), Pin::new_output("Y", BusWidth(1)));

        Self {
            id,
            pins,
            debug_mode: false,
            operation_count: 0,
        }
    }
    
    /// Enable debug mode for this component
    pub fn enable_debug_mode(&mut self) {
        self.debug_mode = true;
        log::info!("Debug mode enabled for CustomXOR {}", self.id);
    }
    
    /// Disable debug mode for this component
    pub fn disable_debug_mode(&mut self) {
        self.debug_mode = false;
        log::info!("Debug mode disabled for CustomXOR {}", self.id);
    }
    
    /// Get the number of operations performed by this component
    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }
}

impl Component for CustomXor {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "CustomXOR"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, current_time: Timestamp) -> UpdateResult {
        self.operation_count += 1;
        
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

        let output = match (a, b) {
            (Value::Low, Value::Low) => Value::Low,
            (Value::Low, Value::High) => Value::High,
            (Value::High, Value::Low) => Value::High,
            (Value::High, Value::High) => Value::Low,
            _ => Value::Unknown,
        };

        if self.debug_mode {
            log::debug!(
                "CustomXOR {} at time {}: {} XOR {} = {} (op #{})",
                self.id, current_time.0, a, b, output, self.operation_count
            );
        }

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
        self.operation_count = 0;
        
        if self.debug_mode {
            log::debug!("CustomXOR {} reset", self.id);
        }
    }

    fn propagation_delay(&self) -> u64 {
        2 // 2 time units for custom XOR gate
    }
}

/// A custom counter component with configurable bit width
/// 
/// This demonstrates a sequential logic component with state and clock handling.
#[derive(Debug, Clone)]
pub struct CustomCounter {
    id: ComponentId,
    pins: HashMap<String, Pin>,
    bit_width: u32,
    count_value: u32,
    max_value: u32,
    last_clock_state: Value,
    debug_mode: bool,
}

impl CustomCounter {
    /// Create a new custom counter
    pub fn new(id: ComponentId, bit_width: u32) -> Self {
        let max_value = if bit_width >= 32 {
            u32::MAX
        } else {
            (1u32 << bit_width) - 1
        };
        
        let mut pins = HashMap::new();
        pins.insert("CLK".to_string(), Pin::new_input("CLK", BusWidth(1)));
        pins.insert("EN".to_string(), Pin::new_input("EN", BusWidth(1)));
        pins.insert("RST".to_string(), Pin::new_input("RST", BusWidth(1)));
        pins.insert("Q".to_string(), Pin::new_output("Q", BusWidth(bit_width)));
        pins.insert("CARRY".to_string(), Pin::new_output("CARRY", BusWidth(1)));

        Self {
            id,
            pins,
            bit_width,
            count_value: 0,
            max_value,
            last_clock_state: Value::Unknown,
            debug_mode: false,
        }
    }
    
    /// Enable debug mode for this component
    pub fn enable_debug_mode(&mut self) {
        self.debug_mode = true;
        log::info!("Debug mode enabled for CustomCounter {}", self.id);
    }
    
    /// Get the current count value
    pub fn get_count_value(&self) -> u32 {
        self.count_value
    }
    
    /// Get the maximum count value
    pub fn get_max_value(&self) -> u32 {
        self.max_value
    }
    
    /// Set the count value (for testing or initialization)
    pub fn set_count_value(&mut self, value: u32) {
        self.count_value = value.min(self.max_value);
    }
}

impl Component for CustomCounter {
    fn id(&self) -> ComponentId {
        self.id
    }

    fn name(&self) -> &str {
        "CustomCounter"
    }

    fn pins(&self) -> &HashMap<String, Pin> {
        &self.pins
    }

    fn pins_mut(&mut self) -> &mut HashMap<String, Pin> {
        &mut self.pins
    }

    fn update(&mut self, _current_time: Timestamp) -> UpdateResult {
        // Generate output signals based on current state
        let q_signal = if self.bit_width == 1 {
            Signal::new_single(if self.count_value == 1 { Value::High } else { Value::Low })
        } else {
            Signal::from_u64(self.count_value as u64, BusWidth(self.bit_width))
        };
        
        let carry_signal = Signal::new_single(
            if self.count_value == self.max_value { Value::High } else { Value::Low }
        );

        let mut result = UpdateResult::new();
        result.add_output("Q".to_string(), q_signal.clone());
        result.add_output("CARRY".to_string(), carry_signal.clone());
        result.set_delay(self.propagation_delay());

        // Update internal pin states
        if let Some(pin) = self.pins.get_mut("Q") {
            let _ = pin.set_signal(q_signal);
        }
        if let Some(pin) = self.pins.get_mut("CARRY") {
            let _ = pin.set_signal(carry_signal);
        }

        result
    }

    fn reset(&mut self) {
        self.count_value = 0;
        self.last_clock_state = Value::Unknown;
        
        for pin in self.pins.values_mut() {
            if pin.direction == PinDirection::Output {
                if pin.name == "Q" {
                    pin.signal = if self.bit_width == 1 {
                        Signal::new_single(Value::Low)
                    } else {
                        Signal::from_u64(0, BusWidth(self.bit_width))
                    };
                } else if pin.name == "CARRY" {
                    pin.signal = Signal::new_single(Value::Low);
                }
            } else {
                pin.signal = Signal::unknown(pin.width);
            }
        }
        
        if self.debug_mode {
            log::debug!("CustomCounter {} reset", self.id);
        }
    }

    fn is_sequential(&self) -> bool {
        true
    }

    fn clock_edge(&mut self, edge: ClockEdge, current_time: Timestamp) -> UpdateResult {
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

            if reset == Value::High {
                self.count_value = 0;
                if self.debug_mode {
                    log::debug!("CustomCounter {} reset via RST pin at time {}", self.id, current_time.0);
                }
            } else if enable == Value::High {
                let old_value = self.count_value;
                self.count_value = if self.count_value >= self.max_value {
                    0 // Wrap around
                } else {
                    self.count_value + 1
                };
                
                if self.debug_mode {
                    log::debug!(
                        "CustomCounter {} at time {}: {} -> {} (carry: {})",
                        self.id, current_time.0, old_value, self.count_value,
                        if old_value == self.max_value { "high" } else { "low" }
                    );
                }
            }

            return self.update(current_time);
        }

        UpdateResult::new()
    }

    fn propagation_delay(&self) -> u64 {
        3 // 3 time units for counter
    }
}

/// Helper trait for components that can be configured via parameters
pub trait ParameterConfigurable {
    /// Configure the component with the given parameters
    fn configure(&mut self, params: &HashMap<String, String>) -> Result<(), String>;
    
    /// Get the current configuration as parameters
    fn get_configuration(&self) -> HashMap<String, String>;
}

impl ParameterConfigurable for CustomXor {
    fn configure(&mut self, params: &HashMap<String, String>) -> Result<(), String> {
        if let Some(debug_mode) = params.get("debug_mode") {
            match debug_mode.as_str() {
                "true" => self.enable_debug_mode(),
                "false" => self.disable_debug_mode(),
                _ => return Err("debug_mode must be 'true' or 'false'".to_string()),
            }
        }
        
        Ok(())
    }
    
    fn get_configuration(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("debug_mode".to_string(), self.debug_mode.to_string());
        config
    }
}

impl ParameterConfigurable for CustomCounter {
    fn configure(&mut self, params: &HashMap<String, String>) -> Result<(), String> {
        if let Some(debug_mode) = params.get("debug_mode") {
            match debug_mode.as_str() {
                "true" => self.enable_debug_mode(),
                "false" => self.debug_mode = false,
                _ => return Err("debug_mode must be 'true' or 'false'".to_string()),
            }
        }
        
        if let Some(initial_value) = params.get("initial_value") {
            let value: u32 = initial_value.parse()
                .map_err(|_| "initial_value must be a valid integer".to_string())?;
            if value > self.max_value {
                return Err(format!("initial_value {} exceeds maximum {}", value, self.max_value));
            }
            self.count_value = value;
        }
        
        Ok(())
    }
    
    fn get_configuration(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("debug_mode".to_string(), self.debug_mode.to_string());
        config.insert("bit_width".to_string(), self.bit_width.to_string());
        config.insert("initial_value".to_string(), self.count_value.to_string());
        config.insert("max_value".to_string(), self.max_value.to_string());
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_xor_creation() {
        let xor_gate = CustomXor::new(ComponentId::new(1));
        assert_eq!(xor_gate.id(), ComponentId::new(1));
        assert_eq!(xor_gate.name(), "CustomXOR");
        assert_eq!(xor_gate.pins().len(), 3);
        assert_eq!(xor_gate.get_operation_count(), 0);
    }

    #[test]
    fn test_custom_xor_logic() {
        let mut xor_gate = CustomXor::new(ComponentId::new(1));
        
        // Test XOR truth table
        let test_cases = [
            (Value::Low, Value::Low, Value::Low),
            (Value::Low, Value::High, Value::High),
            (Value::High, Value::Low, Value::High),
            (Value::High, Value::High, Value::Low),
        ];

        for (a, b, expected) in test_cases {
            xor_gate.get_pin_mut("A").unwrap().set_signal(Signal::new_single(a)).unwrap();
            xor_gate.get_pin_mut("B").unwrap().set_signal(Signal::new_single(b)).unwrap();

            let result = xor_gate.update(Timestamp(0));
            let output = result.outputs.get("Y").unwrap();
            assert_eq!(
                output.as_single(),
                Some(expected),
                "XOR({}, {}) should be {}",
                a, b, expected
            );
        }
        
        assert_eq!(xor_gate.get_operation_count(), 4);
    }

    #[test]
    fn test_custom_counter_creation() {
        let counter = CustomCounter::new(ComponentId::new(2), 4);
        assert_eq!(counter.id(), ComponentId::new(2));
        assert_eq!(counter.name(), "CustomCounter");
        assert_eq!(counter.pins().len(), 5); // CLK, EN, RST, Q, CARRY
        assert_eq!(counter.get_count_value(), 0);
        assert_eq!(counter.get_max_value(), 15); // 2^4 - 1
    }

    #[test]
    fn test_custom_counter_counting() {
        let mut counter = CustomCounter::new(ComponentId::new(2), 3); // 3-bit counter (max 7)
        
        // Enable the counter
        counter.get_pin_mut("EN").unwrap().set_signal(Signal::new_single(Value::High)).unwrap();
        counter.get_pin_mut("RST").unwrap().set_signal(Signal::new_single(Value::Low)).unwrap();
        
        // Test counting sequence
        for expected_count in 0..=7 {
            assert_eq!(counter.get_count_value(), expected_count);
            
            let _result = counter.clock_edge(ClockEdge::Rising, Timestamp(0));
        }
        
        // Should wrap around to 0
        assert_eq!(counter.get_count_value(), 0);
    }

    #[test]
    fn test_parameter_configuration() {
        let mut xor_gate = CustomXor::new(ComponentId::new(1));
        
        let mut params = HashMap::new();
        params.insert("debug_mode".to_string(), "true".to_string());
        
        assert!(xor_gate.configure(&params).is_ok());
        assert!(xor_gate.debug_mode);
        
        let config = xor_gate.get_configuration();
        assert_eq!(config.get("debug_mode"), Some(&"true".to_string()));
    }
}