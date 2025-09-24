//! Integration tests for chronogram functionality

#[cfg(feature = "gui")]
use logisim_core::{
    netlist::NodeId,
    signal::{BusWidth, Signal, Timestamp, Value},
    Simulation,
};
#[cfg(feature = "gui")]
use logisim_ui::gui::chronogram::{
    ChronogramModel, ChronogramPanel, SignalInfo, Timeline, Waveform,
};

#[cfg(feature = "gui")]
#[test]
fn test_chronogram_model_creation() {
    let model = ChronogramModel::new();
    assert_eq!(model.signal_count(), 0);
    assert!(!model.has_data());
}

#[cfg(feature = "gui")]
#[test]
fn test_signal_info_creation() {
    let info = SignalInfo::new(NodeId(1), "test_clk".to_string(), BusWidth(1), 0);
    assert_eq!(info.name, "test_clk");
    assert_eq!(info.width, BusWidth(1));
    assert!(!info.selected);
}

#[cfg(feature = "gui")]
#[test]
fn test_chronogram_panel_creation() {
    let panel = ChronogramPanel::new();
    assert!(!panel.is_recording());
}

#[cfg(feature = "gui")]
#[test]
fn test_timeline_creation() {
    let timeline = Timeline::new();
    assert_eq!(timeline.zoom(), 10.0); // DEFAULT_TICK_WIDTH
    assert_eq!(timeline.scroll_offset(), Timestamp(0));
}

#[cfg(feature = "gui")]
#[test]
fn test_waveform_creation() {
    let waveform = Waveform::new();
    assert!(!waveform.is_selected());
}

#[cfg(feature = "gui")]
#[test]
fn test_signal_recording() {
    let mut model = ChronogramModel::new();
    let info = SignalInfo::new(NodeId(1), "clk".to_string(), BusWidth(1), 0);

    model.add_signal(info);

    let signal = Signal::new_single(Value::High);
    model.record_signal_change(NodeId(1), Timestamp(10), signal);

    assert_eq!(model.signal_count(), 1);
    assert!(model.has_data());
    assert_eq!(model.start_time(), Timestamp(10));
    assert_eq!(model.end_time(), Timestamp(10));
}

#[cfg(feature = "gui")]
#[test]
fn test_chronogram_panel_with_simulation() {
    let mut panel = ChronogramPanel::new();
    let simulation = Simulation::new();

    panel.start_recording(&simulation);
    assert!(panel.is_recording());

    panel.stop_recording();
    assert!(!panel.is_recording());
}

#[cfg(feature = "gui")]
#[test]
fn test_export_functionality() {
    let panel = ChronogramPanel::new();
    let export_text = panel.export_to_text();

    assert!(export_text.contains("Logisim-RUST Chronogram Export"));
    assert!(export_text.contains("Time Range:"));
    assert!(export_text.contains("Signals: 0"));
}

#[cfg(feature = "gui")]
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

#[cfg(feature = "gui")]
#[test]
fn test_signal_value_formatting() {
    let waveform = Waveform::new();

    // Single bit signals
    let single_high = Signal::new_single(Value::High);
    assert_eq!(waveform.format_signal_value(&single_high), "1");

    let single_low = Signal::new_single(Value::Low);
    assert_eq!(waveform.format_signal_value(&single_low), "0");

    // Multi-bit signal (4-bit: 1011 = 13 in decimal)
    let multi_bit = Signal::new_bus(vec![Value::High, Value::High, Value::Low, Value::High]);
    assert_eq!(waveform.format_signal_value(&multi_bit), "13");
}
