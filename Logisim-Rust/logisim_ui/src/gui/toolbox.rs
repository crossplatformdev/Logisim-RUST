//! Toolbox implementation - equivalent to the Java Toolbox class

use eframe::egui::{self, CollapsingHeader, Ui};

/// Component toolbox for selecting tools and components
pub struct Toolbox {
    /// Currently selected tool
    selected_tool: ToolType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToolType {
    /// Edit/select tool (pointer)
    Edit,
    /// Wiring tool
    Wire,
    /// Text tool
    Text,
    /// Input pin
    InputPin,
    /// Output pin
    OutputPin,
    /// AND gate
    AndGate,
    /// OR gate
    OrGate,
    /// NOT gate
    NotGate,
    /// XOR gate
    XorGate,
    /// NAND gate
    NandGate,
    /// NOR gate
    NorGate,
    /// Buffer
    Buffer,
    /// Clock
    Clock,
    /// Constant
    Constant,
    /// Splitter
    Splitter,
    /// Multiplexer
    Multiplexer,
    /// Demultiplexer
    Demultiplexer,
    /// ROM
    Rom,
    /// RAM
    Ram,
    /// Register
    Register,
    /// Counter
    Counter,
    /// DFlipFlop
    DFlipFlop,
    /// JKFlipFlop
    JKFlipFlop,
    /// SRLatch
    SrLatch,
    
    // Arithmetic components
    /// Adder
    Adder,
    /// Subtractor
    Subtractor,
    /// Multiplier
    Multiplier,
    /// Divider
    Divider,
    /// Negator
    Negator,
    /// Comparator
    Comparator,
    /// Shifter
    Shifter,
    /// BitAdder
    BitAdder,
}

impl Toolbox {
    /// Create a new toolbox
    pub fn new() -> Self {
        Self {
            selected_tool: ToolType::Edit,
        }
    }

    /// Show the toolbox UI
    pub fn show(&mut self, ui: &mut Ui) {
        ui.heading("Toolbox");

        // Basic tools
        CollapsingHeader::new("Tools")
            .default_open(true)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::Edit, "🔗", "Edit/Select");
                self.tool_button(ui, ToolType::Wire, "─", "Wire");
                self.tool_button(ui, ToolType::Text, "T", "Text");
            });

        // I/O components
        CollapsingHeader::new("Input/Output")
            .default_open(true)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::InputPin, "▶", "Input Pin");
                self.tool_button(ui, ToolType::OutputPin, "◀", "Output Pin");
                self.tool_button(ui, ToolType::Clock, "⏱", "Clock");
                self.tool_button(ui, ToolType::Constant, "1", "Constant");
            });

        // Logic gates
        CollapsingHeader::new("Gates")
            .default_open(true)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::AndGate, "&", "AND Gate");
                self.tool_button(ui, ToolType::OrGate, "≥1", "OR Gate");
                self.tool_button(ui, ToolType::NotGate, "¬", "NOT Gate");
                self.tool_button(ui, ToolType::XorGate, "=1", "XOR Gate");
                self.tool_button(ui, ToolType::NandGate, "&̄", "NAND Gate");
                self.tool_button(ui, ToolType::NorGate, "≥̄1", "NOR Gate");
                self.tool_button(ui, ToolType::Buffer, "▷", "Buffer");
            });

        // Plexers
        CollapsingHeader::new("Plexers")
            .default_open(false)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::Splitter, "⊢", "Splitter");
                self.tool_button(ui, ToolType::Multiplexer, "MUX", "Multiplexer");
                self.tool_button(ui, ToolType::Demultiplexer, "DEMUX", "Demultiplexer");
            });

        // Memory
        CollapsingHeader::new("Memory")
            .default_open(false)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::Rom, "ROM", "ROM");
                self.tool_button(ui, ToolType::Ram, "RAM", "RAM");
                self.tool_button(ui, ToolType::Register, "REG", "Register");
                self.tool_button(ui, ToolType::Counter, "CTR", "Counter");
            });

        // Flip-flops and latches
        CollapsingHeader::new("Flip-Flops")
            .default_open(false)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::DFlipFlop, "D", "D Flip-Flop");
                self.tool_button(ui, ToolType::JKFlipFlop, "JK", "JK Flip-Flop");
                self.tool_button(ui, ToolType::SrLatch, "SR", "SR Latch");
            });

        // Arithmetic components
        CollapsingHeader::new("Arithmetic")
            .default_open(false)
            .show(ui, |ui| {
                self.tool_button(ui, ToolType::Adder, "+", "Adder");
                self.tool_button(ui, ToolType::Subtractor, "-", "Subtractor");
                self.tool_button(ui, ToolType::Multiplier, "×", "Multiplier");
                self.tool_button(ui, ToolType::Divider, "÷", "Divider");
                self.tool_button(ui, ToolType::Negator, "±", "Negator");
                self.tool_button(ui, ToolType::Comparator, "CMP", "Comparator");
                self.tool_button(ui, ToolType::Shifter, "<<", "Shifter");
                self.tool_button(ui, ToolType::BitAdder, "+1", "Bit Adder");
            });
    }

    /// Create a tool selection button
    fn tool_button(&mut self, ui: &mut Ui, tool: ToolType, icon: &str, tooltip: &str) {
        let selected = self.selected_tool == tool;

        let response = ui.selectable_label(selected, format!("{} {}", icon, tooltip));

        if response.clicked() {
            self.selected_tool = tool;
        }

        if response.hovered() {
            response.on_hover_text(tooltip);
        }
    }

    /// Get the currently selected tool
    pub fn selected_tool(&self) -> ToolType {
        self.selected_tool
    }

    /// Set the selected tool
    pub fn set_selected_tool(&mut self, tool: ToolType) {
        self.selected_tool = tool;
    }
}

impl Default for Toolbox {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolType {
    /// Get a human-readable name for the tool
    pub fn name(&self) -> &'static str {
        match self {
            ToolType::Edit => "Edit/Select",
            ToolType::Wire => "Wire",
            ToolType::Text => "Text",
            ToolType::InputPin => "Input Pin",
            ToolType::OutputPin => "Output Pin",
            ToolType::AndGate => "AND Gate",
            ToolType::OrGate => "OR Gate",
            ToolType::NotGate => "NOT Gate",
            ToolType::XorGate => "XOR Gate",
            ToolType::NandGate => "NAND Gate",
            ToolType::NorGate => "NOR Gate",
            ToolType::Buffer => "Buffer",
            ToolType::Clock => "Clock",
            ToolType::Constant => "Constant",
            ToolType::Splitter => "Splitter",
            ToolType::Multiplexer => "Multiplexer",
            ToolType::Demultiplexer => "Demultiplexer",
            ToolType::Rom => "ROM",
            ToolType::Ram => "RAM",
            ToolType::Register => "Register",
            ToolType::Counter => "Counter",
            ToolType::DFlipFlop => "D Flip-Flop",
            ToolType::JKFlipFlop => "JK Flip-Flop",
            ToolType::SrLatch => "SR Latch",
            
            // Arithmetic components
            ToolType::Adder => "Adder",
            ToolType::Subtractor => "Subtractor",
            ToolType::Multiplier => "Multiplier",
            ToolType::Divider => "Divider",
            ToolType::Negator => "Negator",
            ToolType::Comparator => "Comparator",
            ToolType::Shifter => "Shifter",
            ToolType::BitAdder => "Bit Adder",
        }
    }
}
