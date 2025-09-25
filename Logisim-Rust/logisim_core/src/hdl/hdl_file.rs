/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL file I/O operations
//!
//! Provides functionality to read and write HDL files with proper error handling
//! and integration with HDL content editors.

use super::strings::Strings;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Result as IoResult, Write};
use std::path::Path;

/// HDL file operations for reading and writing HDL source files
pub struct HdlFile;

/// Trait for HDL content editors that can display and edit HDL text
pub trait HdlContentEditor {
    /// Set the text content of the editor
    fn set_text(&mut self, content: &str);

    /// Get the current text content from the editor
    fn get_text(&self) -> String;
}

impl HdlFile {
    /// Open and read an HDL file, setting its contents in the provided editor
    ///
    /// # Arguments
    /// * `file_path` - Path to the HDL file to open
    /// * `editor` - HDL content editor to receive the file contents
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(std::io::Error)` if file reading fails
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    ///
    /// struct MockEditor {
    ///     content: String,
    /// }
    ///
    /// impl HdlContentEditor for MockEditor {
    ///     fn set_text(&mut self, content: &str) {
    ///         self.content = content.to_string();
    ///     }
    ///     
    ///     fn get_text(&self) -> String {
    ///         self.content.clone()
    ///     }
    /// }
    ///
    /// let mut editor = MockEditor { content: String::new() };
    /// // HdlFile::open(Path::new("test.vhdl"), &mut editor)?;
    /// ```
    pub fn open<P: AsRef<Path>>(file_path: P, editor: &mut dyn HdlContentEditor) -> IoResult<()> {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        let mut content = String::new();
        for line_result in reader.lines() {
            let line = line_result.map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    Strings::get("hdlFileReaderError"),
                )
            })?;
            content.push_str(&line);
            content.push('\n'); // Use consistent line ending
        }

        editor.set_text(&content);
        Ok(())
    }

    /// Save the contents of an HDL editor to a file
    ///
    /// # Arguments
    /// * `file_path` - Path where the HDL file should be saved
    /// * `editor` - HDL content editor containing the content to save
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(std::io::Error)` if file writing fails
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    ///
    /// let editor = MockEditor {
    ///     content: "-- VHDL code here".to_string()
    /// };
    /// // HdlFile::save(Path::new("output.vhdl"), &editor)?;
    /// ```
    pub fn save<P: AsRef<Path>>(file_path: P, editor: &dyn HdlContentEditor) -> IoResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)?;

        let mut writer = BufWriter::new(file);
        let data = editor.get_text();

        writer.write_all(data.as_bytes()).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::WriteZero,
                Strings::get("hdlFileWriterError"),
            )
        })?;

        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    struct TestEditor {
        content: String,
    }

    impl HdlContentEditor for TestEditor {
        fn set_text(&mut self, content: &str) {
            self.content = content.to_string();
        }

        fn get_text(&self) -> String {
            self.content.clone()
        }
    }

    #[test]
    fn test_hdl_file_open() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "-- VHDL test content\nentity test is\nend test;";
        fs::write(temp_file.path(), test_content).unwrap();

        let mut editor = TestEditor {
            content: String::new(),
        };

        HdlFile::open(temp_file.path(), &mut editor).unwrap();
        assert!(editor.get_text().contains("VHDL test content"));
        assert!(editor.get_text().contains("entity test is"));
    }

    #[test]
    fn test_hdl_file_save() {
        let temp_file = NamedTempFile::new().unwrap();
        let test_content = "-- Saved VHDL content\narchitecture rtl of test is\nbegin\nend rtl;";

        let editor = TestEditor {
            content: test_content.to_string(),
        };

        HdlFile::save(temp_file.path(), &editor).unwrap();

        let saved_content = fs::read_to_string(temp_file.path()).unwrap();
        assert_eq!(saved_content, test_content);
    }

    #[test]
    fn test_hdl_file_round_trip() {
        let temp_file = NamedTempFile::new().unwrap();
        let original_content =
            "library ieee;\nuse ieee.std_logic_1164.all;\n\nentity counter is\nend counter;";

        // Save content
        let editor = TestEditor {
            content: original_content.to_string(),
        };
        HdlFile::save(temp_file.path(), &editor).unwrap();

        // Load content back
        let mut loaded_editor = TestEditor {
            content: String::new(),
        };
        HdlFile::open(temp_file.path(), &mut loaded_editor).unwrap();

        // Should match (note: line endings are normalized to \n)
        let expected = original_content.replace('\n', "\n");
        assert!(loaded_editor.get_text().trim_end() == expected.trim_end());
    }
}
