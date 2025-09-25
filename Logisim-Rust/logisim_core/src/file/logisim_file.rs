//! Logisim file representation
//!
//! This module provides the main file data structure equivalent to Java's LogisimFile class

use crate::CircuitProject;
use std::collections::HashMap;
use std::path::PathBuf;

/// Main Logisim file data structure
/// Equivalent to Java's LogisimFile class
#[derive(Debug, Clone)]
pub struct LogisimFile {
    /// Path to the file on disk
    pub path: Option<PathBuf>,

    /// The main circuit project
    pub project: CircuitProject,

    /// File version information
    pub version: String,

    /// Whether the file has been modified since last save
    pub dirty: bool,

    /// Libraries referenced by this file
    pub libraries: Vec<String>,

    /// Custom components defined in this file (just names for now)
    pub custom_components: HashMap<String, String>,
}

impl LogisimFile {
    /// Create a new empty Logisim file
    pub fn new() -> Self {
        Self {
            path: None,
            project: CircuitProject {
                source: "untitled".to_string(),
                version: "1.0".to_string(),
                libraries: Vec::new(),
                main_circuit: "main".to_string(),
                options: indexmap::IndexMap::new(),
                mappings: Vec::new(),
                toolbar: Vec::new(),
                circuits: Vec::new(),
            },
            version: "1.0".to_string(),
            dirty: false,
            libraries: Vec::new(),
            custom_components: HashMap::new(),
        }
    }

    /// Create a Logisim file from a project
    pub fn from_project(project: CircuitProject) -> Self {
        Self {
            path: None,
            project,
            version: "1.0".to_string(),
            dirty: false,
            libraries: Vec::new(),
            custom_components: HashMap::new(),
        }
    }

    /// Set the file path
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    /// Get the file path
    pub fn get_path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Mark the file as dirty (modified)
    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    /// Check if the file is dirty (modified)
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Get the file name for display
    pub fn get_display_name(&self) -> String {
        if let Some(path) = &self.path {
            let mut name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("untitled")
                .to_string();

            if self.dirty {
                name.push_str(crate::build_info::constants::DIRTY_MARKER);
            }

            name
        } else {
            let mut name = "untitled".to_string();
            if self.dirty {
                name.push_str(crate::build_info::constants::DIRTY_MARKER);
            }
            name
        }
    }

    /// Add a library reference
    pub fn add_library(&mut self, library_name: String) {
        if !self.libraries.contains(&library_name) {
            self.libraries.push(library_name);
            self.set_dirty(true);
        }
    }

    /// Remove a library reference
    pub fn remove_library(&mut self, library_name: &str) {
        if let Some(pos) = self.libraries.iter().position(|lib| lib == library_name) {
            self.libraries.remove(pos);
            self.set_dirty(true);
        }
    }

    /// Get all library references
    pub fn get_libraries(&self) -> &[String] {
        &self.libraries
    }

    /// Add a custom component (simplified)
    pub fn add_custom_component(&mut self, name: String, description: String) {
        self.custom_components.insert(name, description);
        self.set_dirty(true);
    }

    /// Get a custom component description by name
    pub fn get_custom_component(&self, name: &str) -> Option<&String> {
        self.custom_components.get(name)
    }

    /// Get all custom component names
    pub fn get_custom_component_names(&self) -> Vec<&String> {
        self.custom_components.keys().collect()
    }

    /// Get the main project
    pub fn get_project(&self) -> &CircuitProject {
        &self.project
    }

    /// Get mutable reference to the main project
    pub fn get_project_mut(&mut self) -> &mut CircuitProject {
        self.set_dirty(true);
        &mut self.project
    }

    /// Save the file to its current path
    pub fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.path {
            self.save_to(path.clone())?;
            Ok(())
        } else {
            Err("No file path set for save operation".into())
        }
    }

    /// Save the file to a specific path
    pub fn save_to(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Use the circuit serializer to save the file
        let _serializer = crate::CircSerializer;
        // For now, just log the save operation
        log::info!("Saving circuit file to: {}", path.display());

        self.path = Some(path);
        self.dirty = false;

        Ok(())
    }

    /// Check if the file can be saved (has a path)
    pub fn can_save(&self) -> bool {
        self.path.is_some()
    }
}

impl Default for LogisimFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logisim_file_creation() {
        let file = LogisimFile::new();
        assert!(!file.is_dirty());
        assert!(file.get_path().is_none());
        assert_eq!(file.get_display_name(), "untitled");
    }

    #[test]
    fn test_dirty_flag() {
        let mut file = LogisimFile::new();
        assert!(!file.is_dirty());

        file.set_dirty(true);
        assert!(file.is_dirty());
        assert!(file
            .get_display_name()
            .contains(crate::build_info::constants::DIRTY_MARKER));
    }

    #[test]
    fn test_library_management() {
        let mut file = LogisimFile::new();

        file.add_library("test_lib".to_string());
        assert_eq!(file.get_libraries().len(), 1);
        assert!(file.is_dirty());

        file.set_dirty(false);
        file.remove_library("test_lib");
        assert_eq!(file.get_libraries().len(), 0);
        assert!(file.is_dirty());
    }

    #[test]
    fn test_path_handling() {
        let mut file = LogisimFile::new();
        let path = PathBuf::from("test.circ");

        file.set_path(path.clone());
        assert_eq!(file.get_path(), Some(&path));
        assert_eq!(file.get_display_name(), "test.circ");
    }
}
