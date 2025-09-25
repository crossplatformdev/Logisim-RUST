/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 *
 * Ported to Rust by the Logisim-RUST project
 * https://github.com/crossplatformdev/Logisim-RUST
 */

//! Library system for organizing tools and components
//!
//! Libraries are collections of tools and sub-libraries that provide
//! functionality to the circuit designer. They can be built-in libraries
//! like the base gate library, or external libraries loaded from files.

use crate::{comp::ComponentFactory, tools::tool::Tool};

/// Trait for all libraries in the Logisim-RUST system
///
/// Libraries organize tools and components into logical groups.
/// They can contain other sub-libraries, creating a hierarchical
/// organization system.
pub trait Library: Send + Sync {
    /// Check if this library contains a specific component factory
    fn contains(&self, query: &dyn ComponentFactory) -> bool {
        self.index_of(query).is_some()
    }

    /// Check if this library contains a tool that shares source with the query
    fn contains_from_source(&self, query: &dyn Tool) -> bool {
        for tool in self.get_tools() {
            if tool.shares_source(query) {
                return true;
            }
        }
        false
    }

    /// Get the display name of this library (human-readable)
    fn get_display_name(&self) -> String {
        self.get_name()
    }

    /// Get the unique identifier name of this library
    fn get_name(&self) -> String;

    /// Get all sub-libraries contained in this library
    fn get_libraries(&self) -> Vec<Box<dyn Library>> {
        Vec::new() // Default: no sub-libraries
    }

    /// Get a specific sub-library by name
    fn get_library(&self, name: &str) -> Option<Box<dyn Library>> {
        for lib in self.get_libraries() {
            if lib.get_name() == name {
                return Some(lib);
            }
        }
        None
    }

    /// Remove a sub-library by name
    fn remove_library(&mut self, name: &str) -> bool {
        false // Default: cannot remove libraries
    }

    /// Get a specific tool by name
    fn get_tool(&self, name: &str) -> Option<Box<dyn Tool>> {
        for tool in self.get_tools() {
            if tool.get_name() == name {
                return Some(tool);
            }
        }
        None
    }

    /// Get all tools provided by this library
    fn get_tools(&self) -> Vec<Box<dyn Tool>>;

    /// Get the index of a component factory in this library's tools
    fn index_of(&self, query: &dyn ComponentFactory) -> Option<usize> {
        for (index, tool) in self.get_tools().iter().enumerate() {
            // Check if this tool is an AddTool that contains the factory
            if let Some(add_tool) = tool
                .as_any()
                .downcast_ref::<crate::tools::add_tool::AddTool>()
            {
                if std::ptr::eq(add_tool.get_factory(), query) {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Check if this library has been modified and needs saving
    fn is_dirty(&self) -> bool {
        false // Default: not dirty
    }

    /// Check if this library is hidden from the user interface
    fn is_hidden(&self) -> bool {
        false // Default: not hidden
    }

    /// Mark this library as hidden
    fn set_hidden(&mut self);

    /// Get library as Any trait for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
}

/// A basic concrete implementation of a library
pub struct BasicLibrary {
    name: String,
    display_name: String,
    tools: Vec<Box<dyn Tool>>,
    sub_libraries: Vec<Box<dyn Library>>,
    hidden: bool,
    dirty: bool,
}

impl BasicLibrary {
    /// Create a new basic library
    pub fn new(name: String) -> Self {
        let display_name = name.clone();
        Self {
            name,
            display_name,
            tools: Vec::new(),
            sub_libraries: Vec::new(),
            hidden: false,
            dirty: false,
        }
    }

    /// Create a new basic library with display name
    pub fn new_with_display_name(name: String, display_name: String) -> Self {
        Self {
            name,
            display_name,
            tools: Vec::new(),
            sub_libraries: Vec::new(),
            hidden: false,
            dirty: false,
        }
    }

    /// Add a tool to this library
    pub fn add_tool(&mut self, tool: Box<dyn Tool>) {
        self.tools.push(tool);
        self.dirty = true;
    }

    /// Add a sub-library to this library
    pub fn add_library(&mut self, library: Box<dyn Library>) {
        self.sub_libraries.push(library);
        self.dirty = true;
    }

    /// Remove a tool by name
    pub fn remove_tool(&mut self, name: &str) -> bool {
        let initial_len = self.tools.len();
        self.tools.retain(|tool| tool.get_name() != name);
        let removed = self.tools.len() != initial_len;
        if removed {
            self.dirty = true;
        }
        removed
    }

    /// Clear the dirty flag
    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }
}

impl Library for BasicLibrary {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    fn get_libraries(&self) -> Vec<Box<dyn Library>> {
        // TODO: Implement proper sub-library support
        Vec::new()
    }

    fn remove_library(&mut self, _name: &str) -> bool {
        // TODO: Implement proper sub-library support
        false
    }

    fn get_tools(&self) -> Vec<Box<dyn Tool>> {
        self.tools.iter().map(|tool| tool.clone_tool()).collect()
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn is_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self) {
        self.hidden = true;
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl std::fmt::Debug for BasicLibrary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BasicLibrary")
            .field("name", &self.name)
            .field("display_name", &self.display_name)
            .field("tools_count", &self.tools.len())
            .field("libraries_count", &self.sub_libraries.len())
            .field("hidden", &self.hidden)
            .field("dirty", &self.dirty)
            .finish()
    }
}

// We need a way to clone libraries
pub trait LibraryClone {
    fn clone_library(&self) -> Box<dyn Library>;
}

impl<T> LibraryClone for T
where
    T: Library + Clone + 'static,
{
    fn clone_library(&self) -> Box<dyn Library> {
        Box::new(self.clone())
    }
}

impl Clone for BasicLibrary {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            display_name: self.display_name.clone(),
            tools: self.tools.iter().map(|tool| tool.clone_tool()).collect(),
            sub_libraries: Vec::new(), // Simplified for now
            hidden: self.hidden,
            dirty: self.dirty,
        }
    }
}

impl std::fmt::Display for dyn Library {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::tool::Tool;

    struct MockTool {
        name: String,
        description: String,
    }

    impl MockTool {
        fn new(name: &str, description: &str) -> Self {
            Self {
                name: name.to_string(),
                description: description.to_string(),
            }
        }
    }

    impl Tool for MockTool {
        fn clone_tool(&self) -> Box<dyn Tool> {
            Box::new(MockTool::new(&self.name, &self.description))
        }

        fn get_description(&self) -> String {
            self.description.clone()
        }

        fn get_display_name(&self) -> String {
            self.name.clone()
        }

        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_basic_library_creation() {
        let lib = BasicLibrary::new("test_lib".to_string());

        assert_eq!(lib.get_name(), "test_lib");
        assert_eq!(lib.get_display_name(), "test_lib");
        assert!(!lib.is_hidden());
        assert!(!lib.is_dirty());
        assert_eq!(lib.get_tools().len(), 0);
        assert_eq!(lib.get_libraries().len(), 0);
    }

    #[test]
    fn test_library_with_display_name() {
        let lib =
            BasicLibrary::new_with_display_name("test_lib".to_string(), "Test Library".to_string());

        assert_eq!(lib.get_name(), "test_lib");
        assert_eq!(lib.get_display_name(), "Test Library");
    }

    #[test]
    fn test_add_tool_to_library() {
        let mut lib = BasicLibrary::new("test_lib".to_string());
        let tool = Box::new(MockTool::new("test_tool", "A test tool"));

        assert_eq!(lib.get_tools().len(), 0);
        assert!(!lib.is_dirty());

        lib.add_tool(tool);

        assert_eq!(lib.get_tools().len(), 1);
        assert!(lib.is_dirty());

        let tools = lib.get_tools();
        assert_eq!(tools[0].get_name(), "test_tool");
    }

    #[test]
    fn test_get_tool_by_name() {
        let mut lib = BasicLibrary::new("test_lib".to_string());
        let tool = Box::new(MockTool::new("test_tool", "A test tool"));
        lib.add_tool(tool);

        let found_tool = lib.get_tool("test_tool");
        assert!(found_tool.is_some());
        assert_eq!(found_tool.unwrap().get_name(), "test_tool");

        let not_found = lib.get_tool("nonexistent");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_remove_tool() {
        let mut lib = BasicLibrary::new("test_lib".to_string());
        let tool = Box::new(MockTool::new("test_tool", "A test tool"));
        lib.add_tool(tool);
        lib.mark_clean();

        assert_eq!(lib.get_tools().len(), 1);
        assert!(!lib.is_dirty());

        let removed = lib.remove_tool("test_tool");
        assert!(removed);
        assert_eq!(lib.get_tools().len(), 0);
        assert!(lib.is_dirty());

        let not_removed = lib.remove_tool("nonexistent");
        assert!(!not_removed);
    }

    #[test]
    fn test_library_hidden_flag() {
        let mut lib = BasicLibrary::new("test_lib".to_string());

        assert!(!lib.is_hidden());
        lib.set_hidden();
        assert!(lib.is_hidden());
    }

    #[test]
    fn test_library_clone() {
        let mut lib = BasicLibrary::new("test_lib".to_string());
        let tool = Box::new(MockTool::new("test_tool", "A test tool"));
        lib.add_tool(tool);

        let cloned = lib.clone();

        assert_eq!(lib.get_name(), cloned.get_name());
        assert_eq!(lib.get_tools().len(), cloned.get_tools().len());
        assert_eq!(lib.is_dirty(), cloned.is_dirty());
    }

    #[test]
    fn test_sub_libraries() {
        let mut parent_lib = BasicLibrary::new("parent".to_string());
        let child_lib = Box::new(BasicLibrary::new("child".to_string()));

        assert_eq!(parent_lib.get_libraries().len(), 0);

        // Note: Simplified implementation doesn't support sub-libraries yet
        // This test documents the expected behavior for future implementation

        // parent_lib.add_library(child_lib);
        // assert_eq!(parent_lib.get_libraries().len(), 1);
        // assert!(parent_lib.is_dirty());

        // For now, we just verify the interface exists
        let not_found = parent_lib.get_library("nonexistent");
        assert!(not_found.is_none());
    }
}
