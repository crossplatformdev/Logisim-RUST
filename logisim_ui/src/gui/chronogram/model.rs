//! Chronogram data model for tracking signals over time.
//!
//! This module provides the data structures and logic for capturing,
//! storing, and managing signal state changes during simulation.

use logisim_core::{signal::{Signal, Timestamp, Value, BusWidth}, netlist::NodeId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};

/// Information about a signal being tracked in the chronogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalInfo {
    /// Unique identifier for the signal
    pub id: NodeId,
    /// Human-readable name for the signal
    pub name: String,
    /// Width of the signal (1 for single bit, >1 for buses)
    pub width: BusWidth,
    /// Index for display ordering
    pub index: usize,
    /// Whether this signal is currently selected/highlighted
    pub selected: bool,
}

impl SignalInfo {
    /// Create a new signal info
    pub fn new(id: NodeId, name: String, width: BusWidth, index: usize) -> Self {
        Self {
            id,
            name,
            width,
            index,
            selected: false,
        }
    }
    
    /// Get formatted maximum value (for multi-bit signals)
    pub fn formatted_max_value(&self) -> String {
        if self.width.is_single_bit() {
            "1".to_string()
        } else {
            format!("{}", (1u64 << self.width.as_u32()) - 1)
        }
    }
    
    /// Get formatted minimum value
    pub fn formatted_min_value(&self) -> String {
        "0".to_string()
    }
}

/// Data for a single signal's state changes over time
#[derive(Debug, Clone, Default)]
pub struct SignalData {
    /// Map of timestamp to signal value
    pub values: BTreeMap<Timestamp, Signal>,
    /// The signal info
    pub info: Option<SignalInfo>,
}

impl SignalData {
    /// Create new signal data
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a signal value at a specific time
    pub fn add_value(&mut self, time: Timestamp, signal: Signal) {
        self.values.insert(time, signal);
    }
    
    /// Get the signal value at a specific time (or the last value before that time)
    pub fn get_value_at(&self, time: Timestamp) -> Option<&Signal> {
        self.values.range(..=time).next_back().map(|(_, signal)| signal)
    }
    
    /// Get an iterator over all value changes
    pub fn iter(&self) -> impl Iterator<Item = (&Timestamp, &Signal)> {
        self.values.iter()
    }
    
    /// Check if there are any recorded values
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// Get the time range of recorded data
    pub fn time_range(&self) -> Option<(Timestamp, Timestamp)> {
        if self.values.is_empty() {
            None
        } else {
            let min = *self.values.keys().next().unwrap();
            let max = *self.values.keys().next_back().unwrap();
            Some((min, max))
        }
    }
}

/// Main chronogram model managing all signal data and simulation state
#[derive(Debug, Default)]
pub struct ChronogramModel {
    /// Map of signal ID to signal data
    signals: HashMap<NodeId, SignalData>,
    /// Ordered list of signal infos for display
    signal_order: Vec<SignalInfo>,
    /// Current simulation start time
    start_time: Timestamp,
    /// Current simulation end time
    end_time: Timestamp,
    /// Time scale for displaying the chronogram
    time_scale: f64,
    /// Current cursor position in time
    cursor_time: Timestamp,
}

impl ChronogramModel {
    /// Create a new chronogram model
    pub fn new() -> Self {
        Self {
            time_scale: 1.0,
            ..Default::default()
        }
    }
    
    /// Add a signal to be tracked
    pub fn add_signal(&mut self, info: SignalInfo) {
        let id = info.id;
        self.signal_order.push(info.clone());
        self.signals.entry(id).or_insert_with(|| {
            let mut data = SignalData::new();
            data.info = Some(info);
            data
        });
    }
    
    /// Remove a signal from tracking
    pub fn remove_signal(&mut self, id: NodeId) {
        self.signals.remove(&id);
        self.signal_order.retain(|info| info.id != id);
    }
    
    /// Record a signal value change
    pub fn record_signal_change(&mut self, id: NodeId, time: Timestamp, signal: Signal) {
        if let Some(data) = self.signals.get_mut(&id) {
            data.add_value(time, signal);
            
            // Update time range
            if time < self.start_time || self.start_time == Timestamp(0) {
                self.start_time = time;
            }
            if time > self.end_time {
                self.end_time = time;
            }
        }
    }
    
    /// Get signal data by ID
    pub fn get_signal_data(&self, id: NodeId) -> Option<&SignalData> {
        self.signals.get(&id)
    }
    
    /// Get all signals in display order
    pub fn signals(&self) -> &[SignalInfo] {
        &self.signal_order
    }
    
    /// Get number of signals being tracked
    pub fn signal_count(&self) -> usize {
        self.signal_order.len()
    }
    
    /// Get a signal by index
    pub fn get_signal(&self, index: usize) -> Option<&SignalInfo> {
        self.signal_order.get(index)
    }
    
    /// Get start time
    pub fn start_time(&self) -> Timestamp {
        self.start_time
    }
    
    /// Get end time
    pub fn end_time(&self) -> Timestamp {
        self.end_time
    }
    
    /// Get time scale
    pub fn time_scale(&self) -> f64 {
        self.time_scale
    }
    
    /// Set time scale
    pub fn set_time_scale(&mut self, scale: f64) {
        self.time_scale = scale;
    }
    
    /// Get cursor time
    pub fn cursor_time(&self) -> Timestamp {
        self.cursor_time
    }
    
    /// Set cursor time
    pub fn set_cursor_time(&mut self, time: Timestamp) {
        self.cursor_time = time;
    }
    
    /// Clear all signal data (reset)
    pub fn clear(&mut self) {
        for data in self.signals.values_mut() {
            data.values.clear();
        }
        self.start_time = Timestamp(0);
        self.end_time = Timestamp(0);
        self.cursor_time = Timestamp(0);
    }
    
    /// Update signal selection
    pub fn set_signal_selected(&mut self, id: NodeId, selected: bool) {
        if let Some(info) = self.signal_order.iter_mut().find(|info| info.id == id) {
            info.selected = selected;
        }
    }
    
    /// Check if we have any data recorded
    pub fn has_data(&self) -> bool {
        self.signals.values().any(|data| !data.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use logisim_core::signal::Value;

    #[test]
    fn test_signal_info_creation() {
        let info = SignalInfo::new(NodeId(1), "test_signal".to_string(), BusWidth(1), 0);
        assert_eq!(info.name, "test_signal");
        assert_eq!(info.width, BusWidth(1));
        assert!(!info.selected);
    }

    #[test]
    fn test_signal_data_operations() {
        let mut data = SignalData::new();
        let signal = Signal::new_single(Value::High);
        
        data.add_value(Timestamp(10), signal.clone());
        
        assert!(!data.is_empty());
        assert_eq!(data.get_value_at(Timestamp(10)), Some(&signal));
        assert_eq!(data.get_value_at(Timestamp(5)), None);
        assert_eq!(data.get_value_at(Timestamp(15)), Some(&signal));
    }

    #[test]
    fn test_chronogram_model() {
        let mut model = ChronogramModel::new();
        let info = SignalInfo::new(NodeId(1), "clk".to_string(), BusWidth(1), 0);
        let signal = Signal::new_single(Value::High);
        
        model.add_signal(info);
        model.record_signal_change(NodeId(1), Timestamp(10), signal);
        
        assert_eq!(model.signal_count(), 1);
        assert_eq!(model.start_time(), Timestamp(10));
        assert_eq!(model.end_time(), Timestamp(10));
        assert!(model.has_data());
    }
}