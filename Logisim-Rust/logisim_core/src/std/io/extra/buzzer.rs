/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Buzzer Component
//!
//! Rust port of `com.cburch.logisim.std.io.extra.Buzzer`
//!
//! An audio output component with configurable waveforms, frequency, and volume.

use crate::{
    data::{Attribute, BitWidth, Bounds, Direction},
    signal::{Signal, Value},
    util::StringGetter,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

/// Waveform types for the buzzer
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum BuzzerWaveform {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    Noise,
}

impl BuzzerWaveform {
    /// Generate amplitude for given parameters
    pub fn generate_amplitude(&self, i: f64, hz: f64, pw: f64) -> f64 {
        match self {
            BuzzerWaveform::Sine => (i * hz * 2.0 * std::f64::consts::PI).sin(),
            BuzzerWaveform::Square => {
                if (hz * i) % 1.0 < pw {
                    1.0
                } else {
                    -1.0
                }
            }
            BuzzerWaveform::Triangle => {
                let sine_val = (i * hz * 2.0 * std::f64::consts::PI).sin();
                sine_val.asin() * 2.0 / std::f64::consts::PI
            }
            BuzzerWaveform::Sawtooth => 2.0 * ((hz * i) % 1.0) - 1.0,
            BuzzerWaveform::Noise => rand::random::<f64>() * 2.0 - 1.0,
        }
    }
}

/// Audio channel configuration
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AudioChannel {
    Both,
    Left,
    Right,
}

impl AudioChannel {
    /// Get channel mask for audio output
    pub fn get_mask(&self) -> u8 {
        match self {
            AudioChannel::Both => 3,   // 0b11
            AudioChannel::Left => 1,   // 0b01
            AudioChannel::Right => 2,  // 0b10
        }
    }
}

/// Buzzer component state data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuzzerData {
    /// Current frequency in Hz
    pub frequency: u32,
    /// Volume level (0.0 to 1.0)
    pub volume: f64,
    /// Pulse width for square wave (0.0 to 1.0)
    pub pulse_width: u32,
    /// Waveform type
    pub waveform: BuzzerWaveform,
    /// Audio channel configuration
    pub channel: AudioChannel,
    /// Smooth level for waveform filtering
    pub smooth_level: u32,
    /// Smooth width for filtering
    pub smooth_width: u32,
    /// Whether buzzer is currently enabled
    pub enabled: bool,
    /// Sample rate for audio generation
    pub sample_rate: u32,
}

impl BuzzerData {
    /// Create new buzzer data with default values
    pub fn new() -> Self {
        Self {
            frequency: 440, // A4 note
            volume: 0.5,
            pulse_width: 128, // 50% duty cycle
            waveform: BuzzerWaveform::Sine,
            channel: AudioChannel::Both,
            smooth_level: 2,
            smooth_width: 2,
            enabled: false,
            sample_rate: 44100,
        }
    }

    /// Set frequency (clamped to audible range)
    pub fn set_frequency(&mut self, freq: u32) {
        self.frequency = freq.clamp(20, 20000);
    }

    /// Set volume (clamped to valid range)
    pub fn set_volume(&mut self, vol: f64) {
        self.volume = vol.clamp(0.0, 1.0);
    }

    /// Enable/disable the buzzer
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl Default for BuzzerData {
    fn default() -> Self {
        Self::new()
    }
}

/// Audio thread manager for buzzer
#[derive(Debug)]
struct AudioThread {
    /// Flag to control audio thread
    is_active: Arc<AtomicBool>,
    /// Thread handle
    handle: Option<thread::JoinHandle<()>>,
}

impl AudioThread {
    /// Create new audio thread manager
    fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    /// Start audio thread with given parameters
    fn start(&mut self, data: BuzzerData) {
        self.stop(); // Stop any existing thread
        
        let is_active = Arc::clone(&self.is_active);
        is_active.store(true, Ordering::Relaxed);
        
        let is_active_clone = Arc::clone(&is_active);
        self.handle = Some(thread::spawn(move || {
            Self::audio_thread_function(data, is_active_clone);
        }));
    }

    /// Stop the audio thread
    fn stop(&mut self) {
        self.is_active.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }

    /// Audio thread function (placeholder - would use actual audio library)
    fn audio_thread_function(data: BuzzerData, is_active: Arc<AtomicBool>) {
        // This is a placeholder implementation
        // In a real implementation, this would use an audio library like rodio or cpal
        while is_active.load(Ordering::Relaxed) && data.enabled {
            // Simulate audio processing delay
            thread::sleep(Duration::from_millis(10));
            
            // Here we would:
            // 1. Generate audio samples based on data.waveform
            // 2. Apply volume and channel settings
            // 3. Send to audio output device
            
            // For now, just continue the loop
        }
    }
}

impl Drop for AudioThread {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Buzzer component implementation
///
/// An audio output component that can generate various waveforms.
/// Supports configurable frequency, volume, waveform type, and audio channels.
#[derive(Debug)]
pub struct Buzzer {
    /// Component identifier
    id: ComponentId,
    /// Current buzzer state
    data: BuzzerData,
    /// Component attributes
    attributes: HashMap<String, Attribute>,
    /// Audio thread manager
    audio_thread: AudioThread,
}

impl Clone for Buzzer {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            data: self.data.clone(),
            attributes: self.attributes.clone(),
            audio_thread: AudioThread::new(), // New audio thread for clone
        }
    }
}

impl Buzzer {
    /// Create a new buzzer component
    pub fn new(id: ComponentId) -> Self {
        let mut attributes = HashMap::new();
        
        // Initialize default attributes
        attributes.insert(
            "facing".to_string(),
            Attribute::Direction(Direction::West),
        );
        attributes.insert(
            "volume_width".to_string(),
            Attribute::BitWidth(BitWidth::new(7)),
        );
        attributes.insert(
            "frequency_measure".to_string(),
            Attribute::String("Hz".to_string()),
        );
        attributes.insert(
            "waveform".to_string(),
            Attribute::String("Sine".to_string()),
        );
        attributes.insert(
            "channel".to_string(),
            Attribute::String("Both".to_string()),
        );
        attributes.insert(
            "smooth_level".to_string(),
            Attribute::Integer(2),
        );
        attributes.insert(
            "smooth_width".to_string(),
            Attribute::Integer(2),
        );

        Self {
            id,
            data: BuzzerData::new(),
            attributes,
            audio_thread: AudioThread::new(),
        }
    }

    /// Get the current buzzer data
    pub fn get_data(&self) -> &BuzzerData {
        &self.data
    }

    /// Get mutable reference to buzzer data
    pub fn get_data_mut(&mut self) -> &mut BuzzerData {
        &mut self.data
    }

    /// Update audio output based on current state
    fn update_audio(&mut self) {
        if self.data.enabled && self.data.frequency >= 20 && self.data.frequency <= 20000 {
            self.audio_thread.start(self.data.clone());
        } else {
            self.audio_thread.stop();
        }
    }

    /// Parse waveform from string
    fn parse_waveform(s: &str) -> BuzzerWaveform {
        match s {
            "Sine" => BuzzerWaveform::Sine,
            "Square" => BuzzerWaveform::Square,
            "Triangle" => BuzzerWaveform::Triangle,
            "Sawtooth" => BuzzerWaveform::Sawtooth,
            "Noise" => BuzzerWaveform::Noise,
            _ => BuzzerWaveform::Sine,
        }
    }

    /// Parse audio channel from string
    fn parse_channel(s: &str) -> AudioChannel {
        match s {
            "Both" => AudioChannel::Both,
            "Left" => AudioChannel::Left,
            "Right" => AudioChannel::Right,
            _ => AudioChannel::Both,
        }
    }

    /// Get the component's display name
    pub fn display_name() -> StringGetter {
        StringGetter::new("buzzerComponent")
    }

    /// Get the component's factory ID
    pub fn factory_id() -> &'static str {
        "Buzzer"
    }
}

impl Component for Buzzer {
    fn get_id(&self) -> ComponentId {
        self.id
    }

    fn get_type_name(&self) -> &'static str {
        "Buzzer"
    }

    fn get_bounds(&self) -> Bounds {
        // Buzzer bounds: 40x40 circular component
        let facing = self.get_attribute("facing")
            .and_then(|attr| attr.as_direction())
            .unwrap_or(Direction::West);
            
        match facing {
            Direction::East | Direction::West => Bounds::new(-40, -20, 40, 40),
            Direction::North | Direction::South => Bounds::new(-20, 0, 40, 40),
        }
    }

    fn get_attribute(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    fn set_attribute(&mut self, name: String, value: Attribute) {
        // Update internal data based on attribute changes
        match name.as_str() {
            "waveform" => {
                if let Some(wf_str) = value.as_string() {
                    self.data.waveform = Self::parse_waveform(wf_str);
                }
            }
            "channel" => {
                if let Some(ch_str) = value.as_string() {
                    self.data.channel = Self::parse_channel(ch_str);
                }
            }
            "smooth_level" => {
                if let Some(&level) = value.as_integer() {
                    self.data.smooth_level = level.max(0) as u32;
                }
            }
            "smooth_width" => {
                if let Some(&width) = value.as_integer() {
                    self.data.smooth_width = width.max(1) as u32;
                }
            }
            _ => {}
        }
        
        self.attributes.insert(name, value);
    }

    fn get_input_count(&self) -> usize {
        4 // frequency, enable, volume, pulse_width
    }

    fn get_output_count(&self) -> usize {
        0
    }

    fn propagate(&mut self, inputs: &[Signal]) -> Vec<Signal> {
        if inputs.len() >= 4 {
            // Input 0: Frequency (14-bit)
            if let Some(freq_val) = inputs[0].value.as_u64() {
                let freq = if self.get_attribute("frequency_measure")
                    .and_then(|attr| attr.as_string())
                    .map(|s| s == "dHz")
                    .unwrap_or(false)
                {
                    (freq_val / 10) as u32
                } else {
                    freq_val as u32
                };
                self.data.set_frequency(freq);
            }

            // Input 1: Enable (1-bit)
            self.data.set_enabled(inputs[1].value.is_high());

            // Input 2: Volume (variable width)
            if let Some(vol_val) = inputs[2].value.as_u64() {
                let volume_width = self.get_attribute("volume_width")
                    .and_then(|attr| attr.as_bit_width())
                    .unwrap_or(BitWidth::new(7));
                let max_val = (1u64 << volume_width.width()) - 1;
                self.data.set_volume((vol_val as f64 * 32767.0) / max_val as f64 / 32767.0);
            }

            // Input 3: Pulse width (8-bit)
            if let Some(pw_val) = inputs[3].value.as_u64() {
                self.data.pulse_width = pw_val as u32;
            }
        }

        // Update audio based on new input values
        self.update_audio();

        vec![] // No outputs
    }

    fn is_interactive(&self) -> bool {
        false // Buzzer is not directly interactive (controlled by inputs)
    }

    fn clone_component(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Drop for Buzzer {
    fn drop(&mut self) {
        // Ensure audio thread is stopped when component is dropped
        self.audio_thread.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buzzer_creation() {
        let buzzer = Buzzer::new(ComponentId::new(1));
        assert_eq!(buzzer.get_id(), ComponentId::new(1));
        assert_eq!(buzzer.get_type_name(), "Buzzer");
        assert!(!buzzer.is_interactive());
        assert_eq!(buzzer.get_input_count(), 4);
        assert_eq!(buzzer.get_output_count(), 0);
    }

    #[test]
    fn test_buzzer_data() {
        let mut data = BuzzerData::new();
        assert_eq!(data.frequency, 440);
        assert_eq!(data.volume, 0.5);
        assert!(!data.enabled);

        data.set_frequency(1000);
        assert_eq!(data.frequency, 1000);

        data.set_volume(0.8);
        assert_eq!(data.volume, 0.8);

        data.set_enabled(true);
        assert!(data.enabled);
    }

    #[test]
    fn test_waveform_generation() {
        let waveforms = [
            BuzzerWaveform::Sine,
            BuzzerWaveform::Square,
            BuzzerWaveform::Triangle,
            BuzzerWaveform::Sawtooth,
            BuzzerWaveform::Noise,
        ];

        for waveform in &waveforms {
            let amplitude = waveform.generate_amplitude(0.0, 440.0, 0.5);
            assert!(amplitude >= -1.0 && amplitude <= 1.0);
        }
    }

    #[test]
    fn test_frequency_clamping() {
        let mut data = BuzzerData::new();
        
        // Test lower bound
        data.set_frequency(10); // Below audible range
        assert_eq!(data.frequency, 20);
        
        // Test upper bound
        data.set_frequency(30000); // Above audible range
        assert_eq!(data.frequency, 20000);
        
        // Test valid range
        data.set_frequency(1000);
        assert_eq!(data.frequency, 1000);
    }

    #[test]
    fn test_buzzer_propagation() {
        let mut buzzer = Buzzer::new(ComponentId::new(1));
        
        let inputs = vec![
            Signal::new(Value::known(1000, BitWidth::new(14))), // frequency
            Signal::new(Value::high(BitWidth::new(1))),          // enable
            Signal::new(Value::known(64, BitWidth::new(7))),     // volume
            Signal::new(Value::known(128, BitWidth::new(8))),    // pulse width
        ];
        
        let outputs = buzzer.propagate(&inputs);
        assert_eq!(outputs.len(), 0); // Buzzer has no outputs
        
        // Check that internal state was updated
        assert!(buzzer.get_data().enabled);
        assert_eq!(buzzer.get_data().frequency, 1000);
    }

    #[test]
    fn test_audio_channel_mask() {
        assert_eq!(AudioChannel::Both.get_mask(), 3);
        assert_eq!(AudioChannel::Left.get_mask(), 1);
        assert_eq!(AudioChannel::Right.get_mask(), 2);
    }
}