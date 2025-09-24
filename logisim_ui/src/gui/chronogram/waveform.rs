//! Waveform rendering and display logic.
//!
//! This module handles the rendering of individual signal waveforms,
//! including digital signals, buses, and special states.

use crate::gui::chronogram::{constants::*, model::SignalData, timeline::Timeline};
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use logisim_core::signal::{Signal, Timestamp, Value};

/// Color scheme for waveform rendering
#[derive(Debug, Clone)]
pub struct WaveformColors {
    /// High signal level color
    pub high: Color32,
    /// Low signal level color  
    pub low: Color32,
    /// Unknown state color
    pub unknown: Color32,
    /// Error state color
    pub error: Color32,
    /// Transition edge color
    pub edge: Color32,
    /// Background color
    pub background: Color32,
    /// Selected signal highlight color
    pub selected: Color32,
    /// Bus value text color
    pub text: Color32,
}

impl Default for WaveformColors {
    fn default() -> Self {
        Self {
            high: Color32::from_rgb(0, 128, 0),           // Green for high
            low: Color32::from_rgb(128, 0, 0),            // Red for low
            unknown: Color32::from_rgb(128, 128, 0),      // Yellow for unknown
            error: Color32::from_rgb(255, 0, 255),        // Magenta for error
            edge: Color32::from_rgb(0, 0, 0),             // Black for edges
            background: Color32::from_rgb(255, 255, 255), // White background
            selected: Color32::from_rgb(200, 200, 255),   // Light blue for selection
            text: Color32::from_rgb(0, 0, 0),             // Black text
        }
    }
}

/// Waveform renderer for a single signal
#[derive(Debug)]
pub struct Waveform {
    /// Colors for rendering
    colors: WaveformColors,
    /// Whether this waveform is currently selected
    selected: bool,
    /// Font size for text labels
    font_size: f32,
}

impl Default for Waveform {
    fn default() -> Self {
        Self::new()
    }
}

impl Waveform {
    /// Create a new waveform renderer
    pub fn new() -> Self {
        Self {
            colors: WaveformColors::default(),
            selected: false,
            font_size: 10.0,
        }
    }

    /// Set selection state
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Check if selected
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Set colors
    pub fn set_colors(&mut self, colors: WaveformColors) {
        self.colors = colors;
    }

    /// Render a waveform for the given signal data
    pub fn render(
        &self,
        ui: &mut Ui,
        rect: Rect,
        signal_data: &SignalData,
        timeline: &Timeline,
    ) -> Response {
        let response = ui.allocate_rect(rect, Sense::click());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background
            let bg_color = if self.selected {
                self.colors.selected
            } else {
                self.colors.background
            };
            painter.rect_filled(rect, 0.0, bg_color);

            // Get visible time range
            let (start_time, end_time) = timeline.visible_time_range();

            // Calculate waveform geometry
            let high_y = rect.top() + GAP;
            let low_y = rect.bottom() - GAP;
            let mid_y = (high_y + low_y) / 2.0;

            // Render the waveform
            if let Some(info) = &signal_data.info {
                if info.width.is_single_bit() {
                    self.render_digital_signal(
                        painter,
                        rect,
                        signal_data,
                        timeline,
                        high_y,
                        low_y,
                        mid_y,
                        start_time,
                        end_time,
                    );
                } else {
                    self.render_bus_signal(
                        painter,
                        rect,
                        signal_data,
                        timeline,
                        high_y,
                        low_y,
                        mid_y,
                        start_time,
                        end_time,
                    );
                }
            }

            // Draw cursor if present
            if let Some(cursor_time) = timeline.cursor_time() {
                let cursor_x = rect.left() + timeline.time_to_pixel(cursor_time);
                if cursor_x >= rect.left() && cursor_x <= rect.right() {
                    painter.line_segment(
                        [
                            Pos2::new(cursor_x, rect.top()),
                            Pos2::new(cursor_x, rect.bottom()),
                        ],
                        Stroke::new(2.0, Color32::RED),
                    );
                }
            }
        }

        response
    }

    /// Render a digital (single-bit) signal
    fn render_digital_signal(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        signal_data: &SignalData,
        timeline: &Timeline,
        high_y: f32,
        low_y: f32,
        _mid_y: f32,
        start_time: Timestamp,
        end_time: Timestamp,
    ) {
        let mut last_value: Option<Value> = None;
        let mut last_x = rect.left();

        // Get initial value at start time
        if let Some(initial_signal) = signal_data.get_value_at(start_time) {
            if let Some(initial_value) = initial_signal.get_bit(0) {
                last_value = Some(initial_value);
            }
        }

        // Iterate through value changes in the visible time range
        for (time, signal) in signal_data.iter() {
            if time.as_u64() > end_time.as_u64() {
                break;
            }

            let x = rect.left() + timeline.time_to_pixel(*time);

            if time.as_u64() >= start_time.as_u64() && x >= rect.left() && x <= rect.right() {
                if let Some(value) = signal.get_bit(0) {
                    // Draw horizontal line for previous value
                    if let Some(prev_value) = last_value {
                        let y = self.value_to_y(prev_value, high_y, low_y);
                        painter.line_segment(
                            [Pos2::new(last_x, y), Pos2::new(x, y)],
                            Stroke::new(1.0, self.value_color(prev_value)),
                        );
                    }

                    // Draw vertical transition line
                    if let Some(prev_value) = last_value {
                        if prev_value != value {
                            let prev_y = self.value_to_y(prev_value, high_y, low_y);
                            let new_y = self.value_to_y(value, high_y, low_y);
                            painter.line_segment(
                                [Pos2::new(x, prev_y), Pos2::new(x, new_y)],
                                Stroke::new(1.0, self.colors.edge),
                            );
                        }
                    }

                    last_value = Some(value);
                    last_x = x;
                }
            }
        }

        // Draw final horizontal line to end of visible area
        if let Some(value) = last_value {
            let y = self.value_to_y(value, high_y, low_y);
            painter.line_segment(
                [Pos2::new(last_x, y), Pos2::new(rect.right(), y)],
                Stroke::new(1.0, self.value_color(value)),
            );
        }
    }

    /// Render a bus (multi-bit) signal
    fn render_bus_signal(
        &self,
        painter: &egui::Painter,
        rect: Rect,
        signal_data: &SignalData,
        timeline: &Timeline,
        high_y: f32,
        low_y: f32,
        mid_y: f32,
        start_time: Timestamp,
        end_time: Timestamp,
    ) {
        let mut last_signal: Option<&Signal> = None;
        let mut last_x = rect.left();

        // Get initial value at start time
        if let Some(initial_signal) = signal_data.get_value_at(start_time) {
            last_signal = Some(initial_signal);
        }

        // Iterate through value changes
        for (time, signal) in signal_data.iter() {
            if time.as_u64() > end_time.as_u64() {
                break;
            }

            let x = rect.left() + timeline.time_to_pixel(*time);

            if time.as_u64() >= start_time.as_u64() && x >= rect.left() && x <= rect.right() {
                // Draw bus section for previous value
                if let Some(prev_signal) = last_signal {
                    self.draw_bus_section(painter, last_x, x, high_y, low_y, mid_y, prev_signal);
                }

                // Draw transition
                if let Some(prev_signal) = last_signal {
                    if prev_signal != signal {
                        self.draw_bus_transition(painter, x, high_y, low_y);
                    }
                }

                last_signal = Some(signal);
                last_x = x;
            }
        }

        // Draw final bus section to end of visible area
        if let Some(signal) = last_signal {
            self.draw_bus_section(painter, last_x, rect.right(), high_y, low_y, mid_y, signal);
        }
    }

    /// Draw a bus section with constant value
    fn draw_bus_section(
        &self,
        painter: &egui::Painter,
        x1: f32,
        x2: f32,
        high_y: f32,
        low_y: f32,
        mid_y: f32,
        signal: &Signal,
    ) {
        // Draw top and bottom lines
        painter.line_segment(
            [Pos2::new(x1, high_y), Pos2::new(x2, high_y)],
            Stroke::new(1.0, self.colors.edge),
        );
        painter.line_segment(
            [Pos2::new(x1, low_y), Pos2::new(x2, low_y)],
            Stroke::new(1.0, self.colors.edge),
        );

        // Draw value text in the middle if there's enough space
        let width = x2 - x1;
        if width > 20.0 {
            let value_text = self.format_signal_value_internal(signal);
            painter.text(
                Pos2::new(x1 + width / 2.0, mid_y),
                egui::Align2::CENTER_CENTER,
                &value_text,
                egui::FontId::proportional(self.font_size),
                self.colors.text,
            );
        }
    }

    /// Draw a bus transition (X shape)
    fn draw_bus_transition(&self, painter: &egui::Painter, x: f32, high_y: f32, low_y: f32) {
        // Draw X-shaped transition
        painter.line_segment(
            [Pos2::new(x - 2.0, high_y), Pos2::new(x + 2.0, low_y)],
            Stroke::new(1.0, self.colors.edge),
        );
        painter.line_segment(
            [Pos2::new(x - 2.0, low_y), Pos2::new(x + 2.0, high_y)],
            Stroke::new(1.0, self.colors.edge),
        );
    }

    /// Convert a digital value to Y coordinate
    fn value_to_y(&self, value: Value, high_y: f32, low_y: f32) -> f32 {
        match value {
            Value::High => high_y,
            Value::Low => low_y,
            Value::Unknown | Value::Error => (high_y + low_y) / 2.0,
        }
    }

    /// Get color for a digital value
    fn value_color(&self, value: Value) -> Color32 {
        match value {
            Value::High => self.colors.high,
            Value::Low => self.colors.low,
            Value::Unknown => self.colors.unknown,
            Value::Error => self.colors.error,
        }
    }

    /// Format a signal value for display (public method)
    pub fn format_signal_value(&self, signal: &Signal) -> String {
        self.format_signal_value_internal(signal)
    }

    /// Format a signal value for display (internal implementation)
    fn format_signal_value_internal(&self, signal: &Signal) -> String {
        // Convert signal to numeric value for display
        let mut value = 0u64;
        for (i, bit_value) in signal.values().iter().enumerate() {
            match bit_value {
                Value::High => value |= 1 << i,
                Value::Low => {} // Already 0
                Value::Unknown => return "?".to_string(),
                Value::Error => return "X".to_string(),
            }
        }

        // Format based on bus width
        if signal.is_single_bit() {
            match signal.get_bit(0) {
                Some(Value::High) => "1".to_string(),
                Some(Value::Low) => "0".to_string(),
                Some(Value::Unknown) => "?".to_string(),
                Some(Value::Error) => "X".to_string(),
                None => "".to_string(),
            }
        } else {
            // For multi-bit, show hex for wider buses, decimal for narrow ones
            if signal.values().len() > 4 {
                format!("{:X}", value)
            } else {
                format!("{}", value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gui::chronogram::model::SignalInfo;
    use logisim_core::signal::BusWidth;

    #[test]
    fn test_waveform_creation() {
        let waveform = Waveform::new();
        assert!(!waveform.is_selected());
    }

    #[test]
    fn test_waveform_selection() {
        let mut waveform = Waveform::new();
        waveform.set_selected(true);
        assert!(waveform.is_selected());
    }

    #[test]
    fn test_value_to_y() {
        let waveform = Waveform::new();
        let high_y = 10.0;
        let low_y = 40.0;

        assert_eq!(waveform.value_to_y(Value::High, high_y, low_y), high_y);
        assert_eq!(waveform.value_to_y(Value::Low, high_y, low_y), low_y);
        assert_eq!(waveform.value_to_y(Value::Unknown, high_y, low_y), 25.0);
    }

    #[test]
    fn test_signal_value_formatting() {
        let waveform = Waveform::new();

        // Single bit signals
        let single_high = Signal::new_single(Value::High);
        assert_eq!(waveform.format_signal_value(&single_high), "1");

        let single_low = Signal::new_single(Value::Low);
        assert_eq!(waveform.format_signal_value(&single_low), "0");

        // Multi-bit signal (4-bit)
        let multi_bit = Signal::new_bus(vec![Value::High, Value::Low, Value::High, Value::Low]); // 0101 = 5
        assert_eq!(waveform.format_signal_value(&multi_bit), "5");
    }
}
