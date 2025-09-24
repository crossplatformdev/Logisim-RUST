/// Complete chronogram implementation - waveform/timing view
/// Provides full waveform display, export, navigation, and analysis capabilities

use std::collections::HashMap;
use logisim_core::{Signal, Value, ComponentId, Timestamp};

#[derive(Debug, Clone)]
pub struct SignalTrace {
    pub name: String,
    pub component_id: ComponentId,
    pub pin_name: String,
    pub values: Vec<(Timestamp, Value)>,
    pub visible: bool,
    pub color: [u8; 3], // RGB color
    pub grouped: bool,
}

#[derive(Debug, Clone)]
pub struct SignalGroup {
    pub name: String,
    pub signals: Vec<String>,
    pub expanded: bool,
    pub color: [u8; 3],
    pub bus_display: BusDisplayMode,
}

#[derive(Debug, Clone)]
pub enum BusDisplayMode {
    Binary,
    Hexadecimal,
    Decimal,
    Octal,
}

#[derive(Debug, Clone)]
pub struct TimeCursor {
    pub position: Timestamp,
    pub label: String,
    pub color: [u8; 3],
}

#[derive(Debug, Clone)]
pub struct MeasurementCursor {
    pub start: Timestamp,
    pub end: Timestamp,
    pub label: String,
    pub show_duration: bool,
    pub show_frequency: bool,
}

#[derive(Debug)]
pub struct ChronogramView {
    pub signals: HashMap<String, SignalTrace>,
    pub groups: HashMap<String, SignalGroup>,
    pub time_cursors: Vec<TimeCursor>,
    pub measurements: Vec<MeasurementCursor>,
    pub time_scale: f64, // nanoseconds per pixel
    pub time_offset: Timestamp,
    pub signal_height: f32,
    pub auto_scroll: bool,
    pub capture_enabled: bool,
    pub sample_rate: Timestamp,
    pub zoom_level: f64,
    pub selected_signals: Vec<String>,
}

impl Default for ChronogramView {
    fn default() -> Self {
        Self {
            signals: HashMap::new(),
            groups: HashMap::new(),
            time_cursors: Vec::new(),
            measurements: Vec::new(),
            time_scale: 1.0, // 1 ns per pixel
            time_offset: Timestamp(0),
            signal_height: 20.0,
            auto_scroll: true,
            capture_enabled: true,
            sample_rate: Timestamp(1), // Sample every nanosecond
            zoom_level: 1.0,
            selected_signals: Vec::new(),
        }
    }
}

impl ChronogramView {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add signal trace to chronogram
    pub fn add_signal(&mut self, name: String, component_id: ComponentId, pin_name: String) {
        let trace = SignalTrace {
            name: name.clone(),
            component_id,
            pin_name,
            values: Vec::new(),
            visible: true,
            color: self.get_next_color(),
            grouped: false,
        };
        self.signals.insert(name, trace);
    }

    /// Remove signal trace
    pub fn remove_signal(&mut self, name: &str) {
        self.signals.remove(name);
        self.selected_signals.retain(|s| s != name);
    }

    /// Update signal value at current time
    pub fn update_signal(&mut self, name: &str, time: Timestamp, value: Value) {
        if let Some(signal) = self.signals.get_mut(name) {
            // Only add if value changed or this is first sample
            if signal.values.is_empty() || signal.values.last().unwrap().1 != value {
                signal.values.push((time, value));
                
                // Limit trace length to prevent memory issues
                if signal.values.len() > 10000 {
                    signal.values.remove(0);
                }
            }
        }
    }

    /// Create signal group
    pub fn create_group(&mut self, name: String, signal_names: Vec<String>) {
        let group = SignalGroup {
            name: name.clone(),
            signals: signal_names.clone(),
            expanded: true,
            color: self.get_next_color(),
            bus_display: BusDisplayMode::Binary,
        };
        
        // Mark signals as grouped
        for signal_name in &signal_names {
            if let Some(signal) = self.signals.get_mut(signal_name) {
                signal.grouped = true;
            }
        }
        
        self.groups.insert(name, group);
    }

    /// Add time cursor
    pub fn add_time_cursor(&mut self, time: Timestamp, label: String) -> usize {
        let cursor = TimeCursor {
            position: time,
            label,
            color: [255, 0, 0], // Red
        };
        self.time_cursors.push(cursor);
        self.time_cursors.len() - 1
    }

    /// Add measurement between two time points
    pub fn add_measurement(&mut self, start: Timestamp, end: Timestamp, label: String) -> usize {
        let measurement = MeasurementCursor {
            start,
            end,
            label,
            show_duration: true,
            show_frequency: true,
        };
        self.measurements.push(measurement);
        self.measurements.len() - 1
    }

    /// Zoom in/out
    pub fn zoom(&mut self, factor: f64, center_time: Timestamp) {
        let old_scale = self.time_scale;
        self.time_scale *= factor;
        self.time_scale = self.time_scale.max(0.001).min(1000.0); // Limit zoom range
        
        // Adjust offset to zoom around center
        let scale_ratio = self.time_scale / old_scale;
        let center_offset = center_time.0 as f64 - self.time_offset.0 as f64;
        self.time_offset = Timestamp((self.time_offset.0 as f64 + center_offset * (1.0 - scale_ratio)) as u64);
    }

    /// Pan view horizontally
    pub fn pan(&mut self, delta_time: i64) {
        let new_offset = self.time_offset.0 as i64 + delta_time;
        self.time_offset = Timestamp(new_offset.max(0) as u64);
    }

    /// Convert time to screen X coordinate
    pub fn time_to_x(&self, time: Timestamp) -> f32 {
        ((time.0 as f64 - self.time_offset.0 as f64) / self.time_scale) as f32
    }

    /// Convert screen X coordinate to time
    pub fn x_to_time(&self, x: f32) -> Timestamp {
        Timestamp((self.time_offset.0 as f64 + x as f64 * self.time_scale) as u64)
    }

    /// Search for signals by name or pattern
    pub fn search_signals(&self, pattern: &str) -> Vec<String> {
        let pattern_lower = pattern.to_lowercase();
        self.signals
            .keys()
            .filter(|name| name.to_lowercase().contains(&pattern_lower))
            .cloned()
            .collect()
    }

    /// Filter signals by value at specific time
    pub fn filter_by_value(&self, time: Timestamp, value: Value) -> Vec<String> {
        self.signals
            .iter()
            .filter(|(_, signal)| {
                signal.values
                    .iter()
                    .rev()
                    .find(|(t, _)| *t <= time)
                    .map(|(_, v)| *v == value)
                    .unwrap_or(false)
            })
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Export waveform data to CSV format
    pub fn export_csv(&self, start_time: Timestamp, end_time: Timestamp) -> String {
        let mut csv = String::new();
        
        // Header
        csv.push_str("Time");
        for name in self.signals.keys() {
            csv.push(',');
            csv.push_str(name);
        }
        csv.push('\n');
        
        // Sample data at regular intervals
        let sample_interval = Timestamp(((end_time.0 - start_time.0) / 1000).max(1));
        let mut current_time = start_time;
        
        while current_time <= end_time {
            csv.push_str(&current_time.0.to_string());
            
            for signal in self.signals.values() {
                csv.push(',');
                let value = signal.values
                    .iter()
                    .rev()
                    .find(|(t, _)| *t <= current_time)
                    .map(|(_, v)| format!("{:?}", v))
                    .unwrap_or_else(|| "Unknown".to_string());
                csv.push_str(&value);
            }
            csv.push('\n');
            
            current_time = Timestamp(current_time.0 + sample_interval.0);
        }
        
        csv
    }

    /// Export waveform data to VCD (Value Change Dump) format
    pub fn export_vcd(&self, start_time: Timestamp, end_time: Timestamp) -> String {
        let mut vcd = String::new();
        
        // VCD header
        vcd.push_str("$date\n");
        vcd.push_str(&format!("    {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));
        vcd.push_str("$end\n");
        vcd.push_str("$version\n    Logisim-RUST Chronogram\n$end\n");
        vcd.push_str("$timescale\n    1ns\n$end\n");
        
        // Variable definitions
        vcd.push_str("$scope module top $end\n");
        for (i, name) in self.signals.keys().enumerate() {
            let var_id = format!("{}", char::from(b'!' + (i % 95) as u8));
            vcd.push_str(&format!("$var wire 1 {} {} $end\n", var_id, name));
        }
        vcd.push_str("$upscope $end\n");
        vcd.push_str("$enddefinitions $end\n");
        
        // Initial values
        vcd.push_str("$dumpvars\n");
        for (i, signal) in self.signals.values().enumerate() {
            let var_id = format!("{}", char::from(b'!' + (i % 95) as u8));
            let initial_value = signal.values.first()
                .map(|(_, v)| match v {
                    Value::High => "1",
                    Value::Low => "0",
                    _ => "x",
                })
                .unwrap_or("x");
            vcd.push_str(&format!("{}{}\n", initial_value, var_id));
        }
        vcd.push_str("$end\n");
        
        // Value changes
        for signal in self.signals.values() {
            for (time, value) in &signal.values {
                if *time >= start_time && *time <= end_time {
                    let vcd_value = match value {
                        Value::High => "1",
                        Value::Low => "0",
                        _ => "x",
                    };
                    vcd.push_str(&format!("#{}\n", time.0));
                    // Find variable ID for this signal
                    let var_id = self.signals.keys()
                        .position(|n| n == &signal.name)
                        .map(|i| format!("{}", char::from(b'!' + (i % 95) as u8)))
                        .unwrap_or_else(|| "!".to_string());
                    vcd.push_str(&format!("{}{}\n", vcd_value, var_id));
                }
            }
        }
        
        vcd
    }

    /// Get next color for new signals
    fn get_next_color(&self) -> [u8; 3] {
        let colors = [
            [255, 0, 0],   // Red
            [0, 255, 0],   // Green
            [0, 0, 255],   // Blue
            [255, 255, 0], // Yellow
            [255, 0, 255], // Magenta
            [0, 255, 255], // Cyan
            [255, 128, 0], // Orange
            [128, 0, 255], // Purple
        ];
        colors[self.signals.len() % colors.len()]
    }

    /// Auto-fit time range to show all signals
    pub fn auto_fit(&mut self) {
        if self.signals.is_empty() {
            return;
        }

        let mut min_time = Timestamp(u64::MAX);
        let mut max_time = Timestamp(0);

        for signal in self.signals.values() {
            if let Some((first_time, _)) = signal.values.first() {
                min_time = Timestamp(min_time.0.min(first_time.0));
            }
            if let Some((last_time, _)) = signal.values.last() {
                max_time = Timestamp(max_time.0.max(last_time.0));
            }
        }

        if min_time.0 < u64::MAX && max_time.0 > 0 {
            self.time_offset = min_time;
            let duration = max_time.0 - min_time.0;
            // Adjust scale to fit in reasonable screen width (e.g., 800 pixels)
            self.time_scale = duration as f64 / 800.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chronogram_creation() {
        let mut chronogram = ChronogramView::new();
        
        chronogram.add_signal("clk".to_string(), ComponentId(1), "output".to_string());
        assert!(chronogram.signals.contains_key("clk"));
        
        chronogram.update_signal("clk", Timestamp(0), Value::Low);
        chronogram.update_signal("clk", Timestamp(10), Value::High);
        chronogram.update_signal("clk", Timestamp(20), Value::Low);
        
        assert_eq!(chronogram.signals["clk"].values.len(), 3);
    }

    #[test]
    fn test_time_conversion() {
        let chronogram = ChronogramView::new();
        
        let time = Timestamp(100);
        let x = chronogram.time_to_x(time);
        let back_to_time = chronogram.x_to_time(x);
        
        assert_eq!(time.0, back_to_time.0);
    }

    #[test]
    fn test_signal_search() {
        let mut chronogram = ChronogramView::new();
        
        chronogram.add_signal("clock".to_string(), ComponentId(1), "out".to_string());
        chronogram.add_signal("data".to_string(), ComponentId(2), "out".to_string());
        chronogram.add_signal("reset".to_string(), ComponentId(3), "out".to_string());
        
        let results = chronogram.search_signals("cl");
        assert_eq!(results.len(), 1);
        assert!(results.contains(&"clock".to_string()));
    }

    #[test]
    fn test_csv_export() {
        let mut chronogram = ChronogramView::new();
        
        chronogram.add_signal("sig1".to_string(), ComponentId(1), "out".to_string());
        chronogram.update_signal("sig1", Timestamp(0), Value::Low);
        chronogram.update_signal("sig1", Timestamp(10), Value::High);
        
        let csv = chronogram.export_csv(Timestamp(0), Timestamp(20));
        assert!(csv.contains("Time,sig1"));
        assert!(csv.contains("Low"));
        assert!(csv.contains("High"));
    }
}