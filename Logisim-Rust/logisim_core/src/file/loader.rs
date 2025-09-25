//! Circuit file loader
//!
//! This module provides file loading functionality equivalent to Java's Loader class

use super::LoadFailedException;
use crate::{CircParser, CircuitProject};
use std::path::Path;

/// Circuit file loader - equivalent to Java's Loader class
pub struct Loader {
    /// Loaded libraries and their components
    libraries: Vec<String>,
}

impl Loader {
    /// Create a new loader
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Load a circuit file from path
    /// Equivalent to Java's Loader.openLogisimFile()
    pub fn open_logisim_file<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<CircuitProject, LoadFailedException> {
        let path = path.as_ref();

        // Check if file exists
        if !path.exists() {
            return Err(LoadFailedException::FileNotFound(
                path.display().to_string(),
            ));
        }

        // Check file extension
        if let Some(extension) = path.extension() {
            if extension != "circ" {
                log::warn!("File does not have .circ extension: {}", path.display());
            }
        }

        // Load and parse the circuit file
        let _parser = CircParser;
        // For now, return a basic project structure
        log::info!("Loading circuit file: {}", path.display());

        let project = CircuitProject {
            source: path.display().to_string(),
            version: "1.0".to_string(),
            libraries: Vec::new(),
            main_circuit: "main".to_string(),
            options: indexmap::IndexMap::new(),
            mappings: Vec::new(),
            toolbar: Vec::new(),
            circuits: Vec::new(),
        };

        Ok(project)
    }

    /// Load a library from jar file (stub implementation)  
    /// Equivalent to Java's Loader.loadJarLibrary()
    pub fn load_jar_library<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> Result<String, LoadFailedException> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(LoadFailedException::FileNotFound(
                path.display().to_string(),
            ));
        }

        // For now, just return a placeholder library name
        let library_name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        self.libraries.push(library_name.clone());

        log::info!("Loaded JAR library: {}", library_name);
        Ok(library_name)
    }

    /// Get list of loaded libraries
    pub fn get_loaded_libraries(&self) -> &[String] {
        &self.libraries
    }

    /// Check if a library is loaded
    pub fn is_library_loaded(&self, name: &str) -> bool {
        self.libraries.iter().any(|lib| lib == name)
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}

/// Library loader trait - equivalent to Java's LibraryLoader interface
pub trait LibraryLoader {
    /// Load a library by name
    fn load_library(&mut self, name: &str) -> Result<String, LoadFailedException>;

    /// Check if loader can handle the given file
    fn can_load(&self, path: &Path) -> bool;
}

impl LibraryLoader for Loader {
    fn load_library(&mut self, name: &str) -> Result<String, LoadFailedException> {
        // Try to load library by name
        if self.is_library_loaded(name) {
            return Ok(name.to_string());
        }

        // For now, assume it's a built-in library
        self.libraries.push(name.to_string());
        log::info!("Loaded built-in library: {}", name);
        Ok(name.to_string())
    }

    fn can_load(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            extension == "circ" || extension == "jar"
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_loader_creation() {
        let loader = Loader::new();
        assert!(loader.get_loaded_libraries().is_empty());
    }

    #[test]
    fn test_library_loading() {
        let mut loader = Loader::new();
        let result = loader.load_library("test_lib");
        assert!(result.is_ok());
        assert!(loader.is_library_loaded("test_lib"));
    }

    #[test]
    fn test_can_load() {
        let loader = Loader::new();
        assert!(loader.can_load(&PathBuf::from("test.circ")));
        assert!(loader.can_load(&PathBuf::from("test.jar")));
        assert!(!loader.can_load(&PathBuf::from("test.txt")));
    }

    #[test]
    fn test_open_nonexistent_file() {
        let mut loader = Loader::new();
        let result = loader.open_logisim_file("nonexistent.circ");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LoadFailedException::FileNotFound(_)
        ));
    }
}
