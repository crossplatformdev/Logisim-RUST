/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Plexers Library implementation
//!
//! This module provides the PlexersLibrary which contains all plexer components:
//! multiplexers, demultiplexers, decoders, encoders, and bit selectors.

use crate::{
    data::{BitWidth, Bounds, Direction, Location},
    tools::{Library, Tool, AddTool},
    component::{ComponentFactory, ComponentId},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plexers library containing multiplexers, demultiplexers, decoders, etc.
#[derive(Debug)]
pub struct PlexersLibrary {
    /// Library name
    name: String,
    /// Display name
    display_name: String,
    /// Tools provided by this library
    tools: Vec<Box<dyn Tool>>,
    /// Whether the library is hidden
    hidden: bool,
}

impl PlexersLibrary {
    /// Unique identifier for the library
    pub const ID: &'static str = "Plexers";
    
    /// Default propagation delay for plexer components (in simulation time units)
    pub const DELAY: u64 = 3;

    /// Create a new PlexersLibrary instance
    pub fn new() -> Self {
        let mut library = PlexersLibrary {
            name: Self::ID.to_string(),
            display_name: "Plexers".to_string(),
            tools: Vec::new(),
            hidden: false,
        };
        library.initialize_tools();
        library
    }

    /// Initialize all plexer tools in the library
    fn initialize_tools(&mut self) {
        use super::{
            bit_selector::BitSelector,
            decoder::Decoder,
            demultiplexer::Demultiplexer,
            multiplexer::Multiplexer,
            priority_encoder::PriorityEncoder,
        };

        // Create factory functions for each component type
        let multiplexer_factory = Box::new(MultiplexerFactory);
        let demultiplexer_factory = Box::new(DemultiplexerFactory);
        let decoder_factory = Box::new(DecoderFactory);
        let priority_encoder_factory = Box::new(PriorityEncoderFactory);
        let bit_selector_factory = Box::new(BitSelectorFactory);

        // Create AddTool instances for each component
        self.tools.push(Box::new(AddTool::new(multiplexer_factory)));
        self.tools.push(Box::new(AddTool::new(demultiplexer_factory)));
        self.tools.push(Box::new(AddTool::new(decoder_factory)));
        self.tools.push(Box::new(AddTool::new(priority_encoder_factory)));
        self.tools.push(Box::new(AddTool::new(bit_selector_factory)));
    }

    /// Check if a location is within the bounds considering the component's facing direction
    pub fn contains(loc: Location, bounds: Bounds, facing: Direction) -> bool {
        if !bounds.contains_location(loc) {
            return false;
        }

        let x = loc.x();
        let y = loc.y();
        let x0 = bounds.x();
        let x1 = x0 + bounds.width();
        let y0 = bounds.y();
        let y1 = y0 + bounds.height();

        match facing {
            Direction::North | Direction::South => {
                if x < x0 + 5 || x > x1 - 5 {
                    if facing == Direction::South {
                        y < y0 + 5
                    } else {
                        y > y1 - 5
                    }
                } else {
                    true
                }
            }
            Direction::East | Direction::West => {
                if y < y0 + 5 || y > y1 - 5 {
                    if facing == Direction::East {
                        x < x0 + 5
                    } else {
                        x > x1 - 5
                    }
                } else {
                    true
                }
            }
        }
    }

    /// Draw a trapezoid shape for plexer components
    pub fn draw_trapezoid(bounds: Bounds, facing: Direction, facing_lean: i32) -> Vec<(i32, i32)> {
        let width = bounds.width();
        let height = bounds.height();
        let x0 = bounds.x();
        let x1 = x0 + width;
        let y0 = bounds.y();
        let y1 = y0 + height;
        
        let mut points = vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
        
        match facing {
            Direction::West => {
                points[0].1 += facing_lean;
                points[3].1 -= facing_lean;
            }
            Direction::North => {
                points[0].0 += facing_lean;
                points[1].0 -= facing_lean;
            }
            Direction::South => {
                points[2].0 -= facing_lean;
                points[3].0 += facing_lean;
            }
            Direction::East => {
                points[1].1 += facing_lean;
                points[2].1 -= facing_lean;
            }
        }
        
        points
    }
}

impl Default for PlexersLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Library for PlexersLibrary {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    fn get_tools(&self) -> Vec<Box<dyn Tool>> {
        // Return clones of all tools
        self.tools.iter().map(|tool| tool.clone_tool()).collect()
    }

    fn set_hidden(&mut self) {
        self.hidden = true;
    }

    fn is_hidden(&self) -> bool {
        self.hidden
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// Component factory implementations

#[derive(Debug, Clone)]
struct MultiplexerFactory;

impl ComponentFactory for MultiplexerFactory {
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(super::multiplexer::Multiplexer::new(id))
    }

    fn get_name(&self) -> &str {
        "Multiplexer"
    }

    fn get_display_name(&self) -> String {
        "Multiplexer".to_string()
    }

    fn get_description(&self) -> String {
        "Data selector - routes one of several inputs to output".to_string()
    }
}

#[derive(Debug, Clone)]
struct DemultiplexerFactory;

impl ComponentFactory for DemultiplexerFactory {
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(super::demultiplexer::Demultiplexer::new(id))
    }

    fn get_name(&self) -> &str {
        "Demultiplexer"
    }

    fn get_display_name(&self) -> String {
        "Demultiplexer".to_string()
    }

    fn get_description(&self) -> String {
        "Data router - routes input to one of several outputs".to_string()
    }
}

#[derive(Debug, Clone)]
struct DecoderFactory;

impl ComponentFactory for DecoderFactory {
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(super::decoder::Decoder::new(id))
    }

    fn get_name(&self) -> &str {
        "Decoder"
    }

    fn get_display_name(&self) -> String {
        "Decoder".to_string()
    }

    fn get_description(&self) -> String {
        "Address decoder - activates one output based on binary input".to_string()
    }
}

#[derive(Debug, Clone)]
struct PriorityEncoderFactory;

impl ComponentFactory for PriorityEncoderFactory {
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(super::priority_encoder::PriorityEncoder::new(id))
    }

    fn get_name(&self) -> &str {
        "Priority Encoder"
    }

    fn get_display_name(&self) -> String {
        "Priority Encoder".to_string()
    }

    fn get_description(&self) -> String {
        "Outputs index of highest priority active input".to_string()
    }
}

#[derive(Debug, Clone)]
struct BitSelectorFactory;

impl ComponentFactory for BitSelectorFactory {
    fn create_component(&self, id: ComponentId) -> Box<dyn crate::Component> {
        Box::new(super::bit_selector::BitSelector::new(id))
    }

    fn get_name(&self) -> &str {
        "Bit Selector"
    }

    fn get_display_name(&self) -> String {
        "Bit Selector".to_string()
    }

    fn get_description(&self) -> String {
        "Selects specific bits from a bus".to_string()
    }
}

// Plexer-specific attributes and types

/// Size options for plexer components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlexerSize {
    Narrow,
    Wide,
}

impl PlexerSize {
    pub fn width(&self) -> i32 {
        match self {
            PlexerSize::Narrow => 20,
            PlexerSize::Wide => 40,
        }
    }
}

/// Disabled output behavior for plexers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisabledBehavior {
    Floating,  // High impedance
    Zero,      // Output zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plexers_library_creation() {
        let library = PlexersLibrary::new();
        assert_eq!(library.get_name(), "Plexers");
        assert_eq!(library.get_display_name(), "Plexers");
        
        let tools = library.get_tools();
        assert_eq!(tools.len(), 5); // 5 plexer components
    }

    #[test]
    fn test_plexers_library_tools() {
        let library = PlexersLibrary::new();
        let tools = library.get_tools();
        
        // Check that we have the expected number of tools
        assert_eq!(tools.len(), 5);
        
        // All tools should be AddTool instances
        for tool in tools {
            // Check that it's an AddTool by trying to downcast
            assert!(tool.as_any().downcast_ref::<AddTool>().is_some());
        }
    }

    #[test]
    fn test_bounds_checking() {
        let bounds = Bounds::new(10, 10, 40, 30);
        let loc_inside = Location::new(20, 20);
        let loc_outside = Location::new(5, 5);
        
        assert!(PlexersLibrary::contains(loc_inside, bounds, Direction::North));
        assert!(!PlexersLibrary::contains(loc_outside, bounds, Direction::North));
    }

    #[test]
    fn test_trapezoid_drawing() {
        let bounds = Bounds::new(0, 0, 40, 30);
        let points = PlexersLibrary::draw_trapezoid(bounds, Direction::East, 5);
        
        assert_eq!(points.len(), 4);
        // Check that facing lean is applied correctly for East direction
        assert_eq!(points[1].1, 5);  // Top-right point should be moved down
        assert_eq!(points[2].1, 25); // Bottom-right point should be moved up
    }

    #[test]
    fn test_plexer_size() {
        assert_eq!(PlexerSize::Narrow.width(), 20);
        assert_eq!(PlexerSize::Wide.width(), 40);
    }

    #[test]
    fn test_component_factories() {
        let mux_factory = MultiplexerFactory;
        assert_eq!(mux_factory.get_name(), "Multiplexer");
        assert!(mux_factory.get_description().contains("selector"));
        
        let demux_factory = DemultiplexerFactory;
        assert_eq!(demux_factory.get_name(), "Demultiplexer");
        assert!(demux_factory.get_description().contains("router"));
        
        let decoder_factory = DecoderFactory;
        assert_eq!(decoder_factory.get_name(), "Decoder");
        assert!(decoder_factory.get_description().contains("decoder"));
        
        let encoder_factory = PriorityEncoderFactory;
        assert_eq!(encoder_factory.get_name(), "Priority Encoder");
        assert!(encoder_factory.get_description().contains("priority"));
        
        let selector_factory = BitSelectorFactory;
        assert_eq!(selector_factory.get_name(), "Bit Selector");
        assert!(selector_factory.get_description().contains("bits"));
    }
}