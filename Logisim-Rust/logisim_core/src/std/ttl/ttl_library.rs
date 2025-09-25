/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! TTL Library implementation
//! 
//! This is the Rust port of TtlLibrary.java, providing the library of
//! TTL integrated circuit components for Logisim.

use crate::{
    component::ComponentFactory,
    tools::{Library, Tool},
};
use super::{
    ttl7400::Ttl7400,
    ttl7402::Ttl7402,
    ttl7404::Ttl7404,
    ttl7408::Ttl7408,
    ttl7410::Ttl7410,
};

/// TTL component library containing all TTL integrated circuits
/// 
/// This library provides access to the complete set of TTL ICs supported
/// by Logisim, organized in a way that matches the original Java implementation
/// for project file compatibility.
#[derive(Debug, Clone)]
pub struct TtlLibrary {
    tools: Vec<Box<dyn Tool>>,
}

impl TtlLibrary {
    /// Unique identifier for the TTL library
    /// 
    /// This MUST match the Java implementation to maintain project file compatibility.
    pub const ID: &'static str = "TTL";
    
    /// Create a new TTL library instance
    pub fn new() -> Self {
        Self {
            tools: Self::create_tools(),
        }
    }
    
    /// Create all TTL component tools
    fn create_tools() -> Vec<Box<dyn Tool>> {
        vec![
            // Basic logic gates
            Box::new(Ttl7400::new()), // Quad 2-input NAND gate
            Box::new(Ttl7402::new()), // Quad 2-input NOR gate  
            Box::new(Ttl7404::new()), // Hex inverter
            Box::new(Ttl7408::new()), // Quad 2-input AND gate
            Box::new(Ttl7410::new()), // Triple 3-input NAND gate
            
            // TODO: Add remaining TTL ICs following the Java implementation order:
            // Ttl7411, Ttl7413, Ttl7414, Ttl7418, Ttl7419, Ttl7420, Ttl7421,
            // Ttl7424, Ttl7427, Ttl7430, Ttl7432, Ttl7434, Ttl7436, Ttl7442,
            // Ttl7443, Ttl7444, Ttl7447, Ttl7451, Ttl7454, Ttl7458, Ttl7464,
            // Ttl7474, Ttl7485, Ttl7486, Ttl7487, Ttl74125, Ttl74138, Ttl74139,
            // Ttl74151, Ttl74153, Ttl74157, Ttl74158, Ttl74161, Ttl74163,
            // Ttl74164, Ttl74165, Ttl74166, Ttl74175, Ttl74181, Ttl74182,
            // Ttl74192, Ttl74193, Ttl74194, Ttl74240, Ttl74241, Ttl74244,
            // Ttl74245, Ttl74266, Ttl74273, Ttl74283, Ttl74299, Ttl74377,
            // Ttl74381, Ttl74541, Ttl74670, Ttl747266
        ]
    }
}

impl Default for TtlLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl Library for TtlLibrary {
    fn get_id(&self) -> &'static str {
        Self::ID
    }
    
    fn get_display_name(&self) -> &str {
        "TTL"
    }
    
    fn get_tools(&self) -> &[Box<dyn Tool>] {
        &self.tools
    }
    
    fn get_description(&self) -> &str {
        "TTL (Transistor-Transistor Logic) integrated circuits for digital logic design"
    }
}

/// Factory descriptions for TTL components
/// 
/// This provides metadata for each TTL component, including display names
/// and icons, matching the Java implementation structure.
pub struct TtlFactoryDescription {
    pub factory: Box<dyn ComponentFactory>,
    pub display_name: String,
    pub icon_name: String,
}

impl TtlFactoryDescription {
    pub fn new(
        factory: Box<dyn ComponentFactory>,
        display_name: String, 
        icon_name: String,
    ) -> Self {
        Self {
            factory,
            display_name,
            icon_name,
        }
    }
}

/// Complete list of TTL component descriptions
/// 
/// This matches the DESCRIPTIONS array from the Java TtlLibrary class
/// to ensure proper component registration and display.
pub fn get_ttl_descriptions() -> Vec<TtlFactoryDescription> {
    vec![
        TtlFactoryDescription::new(
            Box::new(Ttl7400::new()),
            "TTL7400".to_string(),
            "ttl.gif".to_string(),
        ),
        TtlFactoryDescription::new(
            Box::new(Ttl7402::new()),
            "TTL7402".to_string(),
            "ttl.gif".to_string(),
        ),
        TtlFactoryDescription::new(
            Box::new(Ttl7404::new()),
            "TTL7404".to_string(),
            "ttl.gif".to_string(),
        ),
        TtlFactoryDescription::new(
            Box::new(Ttl7408::new()),
            "TTL7408".to_string(),
            "ttl.gif".to_string(),
        ),
        TtlFactoryDescription::new(
            Box::new(Ttl7410::new()),
            "TTL7410".to_string(),
            "ttl.gif".to_string(),
        ),
        // TODO: Add descriptions for all remaining TTL ICs
    ]
}