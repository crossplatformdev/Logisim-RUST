/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Gray components library
//!
//! The library of Gray code components that the user can access.
//! Equivalent to Java's com.cburch.gray.Components class.

use super::{GrayCounter, GrayIncrementer, SimpleGrayCounter};

/// The library of Gray code components that the user can access.
///
/// This is equivalent to Java's Components class in the com.cburch.gray package.
pub struct GrayComponents {
    /// The list of all tools contained in this library.
    tools: Vec<Box<dyn ComponentTool>>,
}

/// Trait for component tools that can be added to the library
pub trait ComponentTool: Send + Sync {
    fn get_name(&self) -> &str;
    fn get_display_name(&self) -> &str;
}

impl GrayComponents {
    /// Unique identifier of the library, used as reference in project files.
    /// Do NOT change as it will prevent project files from loading.
    pub const ID: &'static str = "Gray Tools";

    /// Constructs an instance of this library.
    ///
    /// This constructor is how Logisim accesses first when it opens the JAR file:
    /// It looks for a no-arguments constructor method of the user-designated class.
    pub fn new() -> Self {
        let tools: Vec<Box<dyn ComponentTool>> = vec![
            Box::new(GrayIncrementer::new()),
            Box::new(SimpleGrayCounter::new()),
            Box::new(GrayCounter::new()),
        ];

        Self { tools }
    }

    /// Returns the name of the library that the user will see.
    pub fn display_name(&self) -> &str {
        "Gray Tools"
    }

    /// Returns the unique identifier of this library.
    pub fn id(&self) -> &str {
        Self::ID
    }

    /// Returns a list of all the tools available in this library.
    pub fn get_tools(&self) -> &[Box<dyn ComponentTool>] {
        &self.tools
    }
}

impl Default for GrayComponents {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_components_creation() {
        let components = GrayComponents::new();
        assert_eq!(components.display_name(), "Gray Tools");
        assert_eq!(components.id(), "Gray Tools");
        assert_eq!(components.get_tools().len(), 3);
    }

    #[test]
    fn test_default_implementation() {
        let components = GrayComponents::default();
        assert_eq!(components.get_tools().len(), 3);
    }
}
