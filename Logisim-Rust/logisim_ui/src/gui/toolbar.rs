use crate::gui::i18n::tr;
use logisim_core::ComponentId;
/// Complete toolbar system with component palette and tool selection
/// Provides comprehensive tool management and component organization
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tool {
    Select,
    Wire,
    Text,
    Component(ComponentType),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentType {
    // Basic gates
    AndGate,
    OrGate,
    NotGate,
    XorGate,
    NandGate,
    NorGate,
    XnorGate,
    Buffer,

    // Input/Output
    InputPin,
    OutputPin,
    Button,
    Switch,
    Led,
    SevenSegmentDisplay,

    // Memory
    DFlipFlop,
    JKFlipFlop,
    SRLatch,
    DLatch,
    Register,
    Counter,
    Ram,
    Rom,

    // Arithmetic
    Adder,
    Subtractor,
    Multiplier,
    Divider,
    Comparator,

    // Plexers
    Multiplexer,
    Demultiplexer,
    Decoder,
    Encoder,
    PriorityEncoder,

    // Advanced
    Clock,
    Random,
    Splitter,
    Tunnel,
    PullResistor,
}

impl ComponentType {
    pub fn name(&self) -> String {
        let key = match self {
            ComponentType::AndGate => "component.and_gate",
            ComponentType::OrGate => "component.or_gate",
            ComponentType::NotGate => "component.not_gate",
            ComponentType::XorGate => "component.xor_gate",
            ComponentType::NandGate => "component.nand_gate",
            ComponentType::NorGate => "component.nor_gate",
            ComponentType::XnorGate => "component.xnor_gate",
            ComponentType::Buffer => "component.buffer",
            ComponentType::InputPin => "component.input_pin",
            ComponentType::OutputPin => "component.output_pin",
            ComponentType::Button => "component.button",
            ComponentType::Switch => "component.switch",
            ComponentType::Led => "component.led",
            ComponentType::SevenSegmentDisplay => "component.seven_segment_display",
            ComponentType::DFlipFlop => "component.d_flip_flop",
            ComponentType::JKFlipFlop => "component.jk_flip_flop",
            ComponentType::SRLatch => "component.sr_latch",
            ComponentType::DLatch => "component.d_latch",
            ComponentType::Register => "component.register",
            ComponentType::Counter => "component.counter",
            ComponentType::Ram => "component.ram",
            ComponentType::Rom => "component.rom",
            ComponentType::Adder => "component.adder",
            ComponentType::Subtractor => "component.subtractor",
            ComponentType::Multiplier => "component.multiplier",
            ComponentType::Divider => "component.divider",
            ComponentType::Comparator => "component.comparator",
            ComponentType::Multiplexer => "component.multiplexer",
            ComponentType::Demultiplexer => "component.demultiplexer",
            ComponentType::Decoder => "component.decoder",
            ComponentType::Encoder => "component.encoder",
            ComponentType::PriorityEncoder => "component.priority_encoder",
            ComponentType::Clock => "component.clock",
            ComponentType::Random => "component.random",
            ComponentType::Splitter => "component.splitter",
            ComponentType::Tunnel => "component.tunnel",
            ComponentType::PullResistor => "component.pull_resistor",
        };
        tr(key)
    }

    pub fn category(&self) -> ComponentCategory {
        match self {
            ComponentType::AndGate
            | ComponentType::OrGate
            | ComponentType::NotGate
            | ComponentType::XorGate
            | ComponentType::NandGate
            | ComponentType::NorGate
            | ComponentType::XnorGate
            | ComponentType::Buffer => ComponentCategory::Gates,

            ComponentType::InputPin
            | ComponentType::OutputPin
            | ComponentType::Button
            | ComponentType::Switch
            | ComponentType::Led
            | ComponentType::SevenSegmentDisplay => ComponentCategory::InputOutput,

            ComponentType::DFlipFlop
            | ComponentType::JKFlipFlop
            | ComponentType::SRLatch
            | ComponentType::DLatch
            | ComponentType::Register
            | ComponentType::Counter
            | ComponentType::Ram
            | ComponentType::Rom => ComponentCategory::Memory,

            ComponentType::Adder
            | ComponentType::Subtractor
            | ComponentType::Multiplier
            | ComponentType::Divider
            | ComponentType::Comparator => ComponentCategory::Arithmetic,

            ComponentType::Multiplexer
            | ComponentType::Demultiplexer
            | ComponentType::Decoder
            | ComponentType::Encoder
            | ComponentType::PriorityEncoder => ComponentCategory::Plexers,

            ComponentType::Clock
            | ComponentType::Random
            | ComponentType::Splitter
            | ComponentType::Tunnel
            | ComponentType::PullResistor => ComponentCategory::Wiring,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ComponentType::AndGate => "🔀",
            ComponentType::OrGate => "⚡",
            ComponentType::NotGate => "🚫",
            ComponentType::XorGate => "⊕",
            ComponentType::NandGate => "🔀̅",
            ComponentType::NorGate => "⚡̅",
            ComponentType::XnorGate => "⊕̅",
            ComponentType::Buffer => "▷",
            ComponentType::InputPin => "→",
            ComponentType::OutputPin => "←",
            ComponentType::Button => "⏺",
            ComponentType::Switch => "⎄",
            ComponentType::Led => "💡",
            ComponentType::SevenSegmentDisplay => "⬜",
            ComponentType::DFlipFlop => "D",
            ComponentType::JKFlipFlop => "JK",
            ComponentType::SRLatch => "SR",
            ComponentType::DLatch => "DL",
            ComponentType::Register => "📦",
            ComponentType::Counter => "🔢",
            ComponentType::Ram => "🗄",
            ComponentType::Rom => "📖",
            ComponentType::Adder => "➕",
            ComponentType::Subtractor => "➖",
            ComponentType::Multiplier => "✖",
            ComponentType::Divider => "➗",
            ComponentType::Comparator => "⚖",
            ComponentType::Multiplexer => "🔀",
            ComponentType::Demultiplexer => "🔁",
            ComponentType::Decoder => "🔓",
            ComponentType::Encoder => "🔒",
            ComponentType::PriorityEncoder => "🏆",
            ComponentType::Clock => "⏰",
            ComponentType::Random => "🎲",
            ComponentType::Splitter => "📎",
            ComponentType::Tunnel => "🚇",
            ComponentType::PullResistor => "⤴",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ComponentCategory {
    Gates,
    InputOutput,
    Memory,
    Arithmetic,
    Plexers,
    Wiring,
}

impl ComponentCategory {
    pub fn name(&self) -> String {
        let key = match self {
            ComponentCategory::Gates => "category.gates",
            ComponentCategory::InputOutput => "category.input_output",
            ComponentCategory::Memory => "category.memory",
            ComponentCategory::Arithmetic => "category.arithmetic",
            ComponentCategory::Plexers => "category.plexers",
            ComponentCategory::Wiring => "category.wiring",
        };
        tr(key)
    }

    pub fn components(&self) -> Vec<ComponentType> {
        match self {
            ComponentCategory::Gates => vec![
                ComponentType::AndGate,
                ComponentType::OrGate,
                ComponentType::NotGate,
                ComponentType::XorGate,
                ComponentType::NandGate,
                ComponentType::NorGate,
                ComponentType::XnorGate,
                ComponentType::Buffer,
            ],
            ComponentCategory::InputOutput => vec![
                ComponentType::InputPin,
                ComponentType::OutputPin,
                ComponentType::Button,
                ComponentType::Switch,
                ComponentType::Led,
                ComponentType::SevenSegmentDisplay,
            ],
            ComponentCategory::Memory => vec![
                ComponentType::DFlipFlop,
                ComponentType::JKFlipFlop,
                ComponentType::SRLatch,
                ComponentType::DLatch,
                ComponentType::Register,
                ComponentType::Counter,
                ComponentType::Ram,
                ComponentType::Rom,
            ],
            ComponentCategory::Arithmetic => vec![
                ComponentType::Adder,
                ComponentType::Subtractor,
                ComponentType::Multiplier,
                ComponentType::Divider,
                ComponentType::Comparator,
            ],
            ComponentCategory::Plexers => vec![
                ComponentType::Multiplexer,
                ComponentType::Demultiplexer,
                ComponentType::Decoder,
                ComponentType::Encoder,
                ComponentType::PriorityEncoder,
            ],
            ComponentCategory::Wiring => vec![
                ComponentType::Clock,
                ComponentType::Random,
                ComponentType::Splitter,
                ComponentType::Tunnel,
                ComponentType::PullResistor,
            ],
        }
    }
}

#[derive(Debug)]
pub struct ToolbarState {
    pub current_tool: Tool,
    pub categories: Vec<ComponentCategory>,
    pub expanded_categories: HashMap<ComponentCategory, bool>,
    pub recent_components: Vec<ComponentType>,
    pub favorite_components: Vec<ComponentType>,
    pub search_query: String,
    pub show_search: bool,
}

impl Default for ToolbarState {
    fn default() -> Self {
        let mut expanded_categories = HashMap::new();
        for category in ComponentCategory::all() {
            expanded_categories.insert(category, true);
        }

        Self {
            current_tool: Tool::Select,
            categories: ComponentCategory::all(),
            expanded_categories,
            recent_components: Vec::new(),
            favorite_components: vec![
                ComponentType::AndGate,
                ComponentType::OrGate,
                ComponentType::NotGate,
                ComponentType::InputPin,
                ComponentType::OutputPin,
            ],
            search_query: String::new(),
            show_search: false,
        }
    }
}

impl ComponentCategory {
    pub fn all() -> Vec<ComponentCategory> {
        vec![
            ComponentCategory::Gates,
            ComponentCategory::InputOutput,
            ComponentCategory::Memory,
            ComponentCategory::Arithmetic,
            ComponentCategory::Plexers,
            ComponentCategory::Wiring,
        ]
    }
}

impl ToolbarState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_tool(&mut self, tool: Tool) {
        // Add to recent components if it's a component tool
        if let Tool::Component(component_type) = &tool {
            self.add_recent_component(component_type.clone());
        }
        self.current_tool = tool;
    }

    pub fn is_tool_selected(&self, tool: &Tool) -> bool {
        &self.current_tool == tool
    }

    pub fn toggle_category(&mut self, category: ComponentCategory) {
        let expanded = self
            .expanded_categories
            .get(&category)
            .copied()
            .unwrap_or(true);
        self.expanded_categories.insert(category, !expanded);
    }

    pub fn is_category_expanded(&self, category: &ComponentCategory) -> bool {
        self.expanded_categories
            .get(category)
            .copied()
            .unwrap_or(true)
    }

    pub fn add_recent_component(&mut self, component: ComponentType) {
        // Remove if already exists
        self.recent_components.retain(|c| c != &component);

        // Add to front
        self.recent_components.insert(0, component);

        // Limit to 10 recent components
        if self.recent_components.len() > 10 {
            self.recent_components.truncate(10);
        }
    }

    pub fn toggle_favorite(&mut self, component: ComponentType) {
        if let Some(index) = self
            .favorite_components
            .iter()
            .position(|c| c == &component)
        {
            self.favorite_components.remove(index);
        } else {
            self.favorite_components.push(component);
        }
    }

    pub fn is_favorite(&self, component: &ComponentType) -> bool {
        self.favorite_components.contains(component)
    }

    pub fn search_components(&self, query: &str) -> Vec<ComponentType> {
        if query.is_empty() {
            return Vec::new();
        }

        let query_lower = query.to_lowercase();
        let mut results = Vec::new();

        for category in &self.categories {
            for component in category.components() {
                let name_lower = component.name().to_lowercase();
                if name_lower.contains(&query_lower) {
                    results.push(component);
                }
            }
        }

        results
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if !self.show_search {
            self.search_query.clear();
        }
    }

    pub fn clear_search(&mut self) {
        self.search_query.clear();
    }
}

/// Keyboard shortcuts for tools
#[derive(Debug, Clone)]
pub struct KeyboardShortcut {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

impl KeyboardShortcut {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            ctrl: false,
            shift: false,
            alt: false,
        }
    }

    pub fn with_ctrl(mut self) -> Self {
        self.ctrl = true;
        self
    }

    pub fn with_shift(mut self) -> Self {
        self.shift = true;
        self
    }

    pub fn with_alt(mut self) -> Self {
        self.alt = true;
        self
    }

    pub fn to_string(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        parts.push(&self.key);
        parts.join("+")
    }
}

pub fn get_tool_shortcuts() -> HashMap<Tool, KeyboardShortcut> {
    let mut shortcuts = HashMap::new();

    shortcuts.insert(Tool::Select, KeyboardShortcut::new("S"));
    shortcuts.insert(Tool::Wire, KeyboardShortcut::new("W"));
    shortcuts.insert(Tool::Text, KeyboardShortcut::new("T"));
    shortcuts.insert(
        Tool::Component(ComponentType::AndGate),
        KeyboardShortcut::new("A"),
    );
    shortcuts.insert(
        Tool::Component(ComponentType::OrGate),
        KeyboardShortcut::new("O"),
    );
    shortcuts.insert(
        Tool::Component(ComponentType::NotGate),
        KeyboardShortcut::new("N"),
    );
    shortcuts.insert(
        Tool::Component(ComponentType::InputPin),
        KeyboardShortcut::new("I"),
    );
    shortcuts.insert(
        Tool::Component(ComponentType::OutputPin),
        KeyboardShortcut::new("P"),
    );

    shortcuts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toolbar_state() {
        let mut toolbar = ToolbarState::new();

        assert_eq!(toolbar.current_tool, Tool::Select);

        toolbar.set_tool(Tool::Component(ComponentType::AndGate));
        assert_eq!(
            toolbar.current_tool,
            Tool::Component(ComponentType::AndGate)
        );
        assert_eq!(toolbar.recent_components.len(), 1);
        assert_eq!(toolbar.recent_components[0], ComponentType::AndGate);
    }

    #[test]
    fn test_component_categories() {
        let gates = ComponentCategory::Gates;
        let components = gates.components();

        assert!(components.contains(&ComponentType::AndGate));
        assert!(components.contains(&ComponentType::OrGate));
        assert!(!components.contains(&ComponentType::InputPin));
    }

    #[test]
    fn test_component_search() {
        let toolbar = ToolbarState::new();

        let results = toolbar.search_components("and");
        assert!(results.contains(&ComponentType::AndGate));
        assert!(results.contains(&ComponentType::NandGate));

        let empty_results = toolbar.search_components("");
        assert!(empty_results.is_empty());
    }

    #[test]
    fn test_favorites() {
        let mut toolbar = ToolbarState::new();

        assert!(toolbar.is_favorite(&ComponentType::AndGate));

        toolbar.toggle_favorite(ComponentType::AndGate);
        assert!(!toolbar.is_favorite(&ComponentType::AndGate));

        toolbar.toggle_favorite(ComponentType::AndGate);
        assert!(toolbar.is_favorite(&ComponentType::AndGate));
    }

    #[test]
    fn test_keyboard_shortcuts() {
        let shortcuts = get_tool_shortcuts();

        assert!(shortcuts.contains_key(&Tool::Select));
        assert!(shortcuts.contains_key(&Tool::Wire));

        let select_shortcut = &shortcuts[&Tool::Select];
        assert_eq!(select_shortcut.key, "S");
        assert!(!select_shortcut.ctrl);
    }
}
