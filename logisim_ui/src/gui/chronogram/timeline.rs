//! Timeline component for the chronogram display.
//!
//! This module handles time axis rendering, navigation, and time-related
//! UI interactions for the chronogram view.

use crate::gui::chronogram::constants::*;
use egui::{Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2};
use logisim_core::signal::Timestamp;

/// Timeline component for displaying time axis and handling time navigation
#[derive(Debug)]
pub struct Timeline {
    /// Current zoom level (pixels per time unit)
    zoom: f32,
    /// Current scroll offset in time units
    scroll_offset: Timestamp,
    /// Width of the timeline in pixels
    width: f32,
    /// Current cursor position in time
    cursor_time: Option<Timestamp>,
    /// Minimum zoom level
    min_zoom: f32,
    /// Maximum zoom level
    max_zoom: f32,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    /// Create a new timeline
    pub fn new() -> Self {
        Self {
            zoom: DEFAULT_TICK_WIDTH,
            scroll_offset: Timestamp(0),
            width: 800.0,
            cursor_time: None,
            min_zoom: 0.1,
            max_zoom: 100.0,
        }
    }

    /// Set the width of the timeline
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    /// Get current zoom level
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(self.min_zoom, self.max_zoom);
    }

    /// Zoom in/out around a specific point
    pub fn zoom_at_point(&mut self, zoom_delta: f32, point_x: f32) {
        let old_zoom = self.zoom;
        let new_zoom = (old_zoom * zoom_delta).clamp(self.min_zoom, self.max_zoom);

        if new_zoom != old_zoom {
            // Adjust scroll to keep the point under the cursor
            let time_at_point = self.pixel_to_time(point_x);
            self.zoom = new_zoom;
            let new_pixel = self.time_to_pixel(time_at_point);
            let offset_delta = new_pixel - point_x;
            self.scroll_offset = Timestamp(
                self.scroll_offset
                    .as_u64()
                    .saturating_sub((offset_delta / self.zoom) as u64),
            );
        }
    }

    /// Get current scroll offset
    pub fn scroll_offset(&self) -> Timestamp {
        self.scroll_offset
    }

    /// Set scroll offset
    pub fn set_scroll_offset(&mut self, offset: Timestamp) {
        self.scroll_offset = offset;
    }

    /// Scroll by a delta in time units
    pub fn scroll_by(&mut self, delta: i64) {
        let new_offset = if delta < 0 {
            self.scroll_offset.as_u64().saturating_sub((-delta) as u64)
        } else {
            self.scroll_offset.as_u64().saturating_add(delta as u64)
        };
        self.scroll_offset = Timestamp(new_offset);
    }

    /// Convert time to pixel position
    pub fn time_to_pixel(&self, time: Timestamp) -> f32 {
        (time.as_u64() as f32 - self.scroll_offset.as_u64() as f32) * self.zoom
    }

    /// Convert pixel position to time
    pub fn pixel_to_time(&self, pixel: f32) -> Timestamp {
        Timestamp((pixel / self.zoom + self.scroll_offset.as_u64() as f32) as u64)
    }

    /// Get the time range currently visible
    pub fn visible_time_range(&self) -> (Timestamp, Timestamp) {
        let start = self.scroll_offset;
        let end = Timestamp(start.as_u64() + (self.width / self.zoom) as u64);
        (start, end)
    }

    /// Set cursor time
    pub fn set_cursor_time(&mut self, time: Option<Timestamp>) {
        self.cursor_time = time;
    }

    /// Get cursor time
    pub fn cursor_time(&self) -> Option<Timestamp> {
        self.cursor_time
    }

    /// Render the timeline header
    pub fn render_header(&mut self, ui: &mut Ui, rect: Rect) -> Response {
        let response = ui.allocate_rect(rect, Sense::click_and_drag());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // Background
            painter.rect_filled(rect, 0.0, Color32::from_gray(240));

            // Calculate time ticks
            let (start_time, end_time) = self.visible_time_range();
            let time_span = end_time.as_u64() - start_time.as_u64();

            // Determine tick spacing based on zoom level
            let target_tick_pixels = 50.0; // Target pixels between major ticks
            let tick_time_span = (target_tick_pixels / self.zoom) as u64;
            let tick_interval = self.calculate_nice_interval(tick_time_span);

            // Draw ticks and labels
            let first_tick = (start_time.as_u64() / tick_interval) * tick_interval;
            let mut tick_time = first_tick;

            while tick_time <= end_time.as_u64() + tick_interval {
                let x = rect.left() + self.time_to_pixel(Timestamp(tick_time));

                if x >= rect.left() && x <= rect.right() {
                    // Major tick line
                    painter.line_segment(
                        [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                        Stroke::new(1.0, Color32::from_gray(100)),
                    );

                    // Tick label
                    let label = format!("{}", tick_time);
                    painter.text(
                        Pos2::new(x + 2.0, rect.top() + 2.0),
                        egui::Align2::LEFT_TOP,
                        &label,
                        egui::FontId::proportional(10.0),
                        Color32::from_gray(60),
                    );
                }

                tick_time += tick_interval;
            }

            // Draw cursor if present
            if let Some(cursor_time) = self.cursor_time {
                let cursor_x = rect.left() + self.time_to_pixel(cursor_time);
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

        // Handle interactions
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let time = self.pixel_to_time(pos.x - rect.left());
                self.cursor_time = Some(time);
            }
        }

        if response.dragged() {
            let delta = response.drag_delta();
            self.scroll_by(-(delta.x / self.zoom) as i64);
        }

        response
    }

    /// Calculate a "nice" interval for tick marks
    fn calculate_nice_interval(&self, target_interval: u64) -> u64 {
        if target_interval == 0 {
            return 1;
        }

        let magnitude = 10u64.pow((target_interval as f64).log10().floor() as u32);
        let normalized = target_interval / magnitude;

        let nice_normalized = if normalized <= 1 {
            1
        } else if normalized <= 2 {
            2
        } else if normalized <= 5 {
            5
        } else {
            10
        };

        nice_normalized * magnitude
    }

    /// Handle mouse wheel for zooming
    pub fn handle_zoom(&mut self, zoom_delta: f32, mouse_pos: f32) {
        if zoom_delta != 0.0 {
            let zoom_factor = if zoom_delta > 0.0 { 1.2 } else { 1.0 / 1.2 };
            self.zoom_at_point(zoom_factor, mouse_pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new();
        assert_eq!(timeline.zoom(), DEFAULT_TICK_WIDTH);
        assert_eq!(timeline.scroll_offset(), Timestamp(0));
    }

    #[test]
    fn test_time_pixel_conversion() {
        let mut timeline = Timeline::new();
        timeline.set_zoom(2.0); // 2 pixels per time unit
        timeline.set_scroll_offset(Timestamp(10));

        // Time 15 should be at pixel position (15-10)*2 = 10
        assert_eq!(timeline.time_to_pixel(Timestamp(15)), 10.0);

        // Pixel 10 should correspond to time 10 + 10/2 = 15
        assert_eq!(timeline.pixel_to_time(10.0), Timestamp(15));
    }

    #[test]
    fn test_zoom_limits() {
        let mut timeline = Timeline::new();
        timeline.set_zoom(1000.0); // Above max
        assert_eq!(timeline.zoom(), timeline.max_zoom);

        timeline.set_zoom(0.01); // Below min
        assert_eq!(timeline.zoom(), timeline.min_zoom);
    }

    #[test]
    fn test_nice_interval_calculation() {
        let timeline = Timeline::new();

        assert_eq!(timeline.calculate_nice_interval(1), 1);
        assert_eq!(timeline.calculate_nice_interval(15), 20);
        assert_eq!(timeline.calculate_nice_interval(35), 50);
        assert_eq!(timeline.calculate_nice_interval(150), 200);
    }
}
