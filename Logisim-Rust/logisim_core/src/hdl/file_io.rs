//! HDL File I/O
//!
//! HDL file loading and saving operations.
//! This module ports functionality from Java com.cburch.hdl.HdlFile and
//! com.cburch.logisim.vhdl.file.HdlFile.

use crate::hdl::content::HdlContentEditor;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use thiserror::Error;

/// HDL file I/O errors
#[derive(Error, Debug)]
pub enum HdlFileError {
    #[error("File read error: {0}")]
    ReadError(String),
    #[error("File write error: {0}")]
    WriteError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Path error: {0}")]
    PathError(String),
}

/// HDL file I/O result type
pub type HdlFileResult<T> = Result<T, HdlFileError>;

/// HDL File operations
/// 
/// Equivalent to Java HdlFile classes from both com.cburch.hdl and com.cburch.logisim.vhdl.file
pub struct HdlFile;

impl HdlFile {
    /// Open and load HDL file content into an editor
    /// 
    /// Equivalent to Java com.cburch.hdl.HdlFile.open()
    pub fn open<P: AsRef<Path>>(
        file_path: P, 
        editor: &mut dyn HdlContentEditor
    ) -> HdlFileResult<()> {
        let file = File::open(&file_path).map_err(|e| {
            HdlFileError::ReadError(format!("Failed to open file: {}", e))
        })?;

        let reader = BufReader::new(file);
        let mut content = String::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| {
                HdlFileError::ReadError(format!("Failed to read line: {}", e))
            })?;
            
            content.push_str(&line);
            content.push_str(&Self::get_line_separator());
        }

        editor.set_text(content);
        Ok(())
    }

    /// Save HDL content from editor to file
    /// 
    /// Equivalent to Java com.cburch.hdl.HdlFile.save()
    pub fn save<P: AsRef<Path>>(
        file_path: P,
        editor: &dyn HdlContentEditor
    ) -> HdlFileResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .map_err(|e| {
                HdlFileError::WriteError(format!("Failed to create file: {}", e))
            })?;

        let mut writer = BufWriter::new(file);
        let data = editor.get_text();
        
        writer.write_all(data.as_bytes()).map_err(|e| {
            HdlFileError::WriteError(format!("Failed to write data: {}", e))
        })?;

        writer.flush().map_err(|e| {
            HdlFileError::WriteError(format!("Failed to flush data: {}", e))
        })?;

        Ok(())
    }

    /// Load HDL file content as string
    /// 
    /// Equivalent to Java com.cburch.logisim.vhdl.file.HdlFile.load()
    pub fn load<P: AsRef<Path>>(file_path: P) -> HdlFileResult<String> {
        let file = File::open(&file_path).map_err(|e| {
            HdlFileError::ReadError(format!("Failed to open file: {}", e))
        })?;

        let reader = BufReader::new(file);
        let mut content = String::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| {
                HdlFileError::ReadError(format!("Failed to read line: {}", e))
            })?;
            
            content.push_str(&line);
            content.push_str(&Self::get_line_separator());
        }

        Ok(content)
    }

    /// Save string content to HDL file
    /// 
    /// Equivalent to Java com.cburch.logisim.vhdl.file.HdlFile.save()
    pub fn save_text<P: AsRef<Path>>(file_path: P, text: &str) -> HdlFileResult<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .map_err(|e| {
                HdlFileError::WriteError(format!("Failed to create file: {}", e))
            })?;

        let mut writer = BufWriter::new(file);
        
        writer.write_all(text.as_bytes()).map_err(|e| {
            HdlFileError::WriteError(format!("Failed to write text: {}", e))
        })?;

        writer.flush().map_err(|e| {
            HdlFileError::WriteError(format!("Failed to flush text: {}", e))
        })?;

        Ok(())
    }

    /// Get the system line separator
    /// 
    /// Equivalent to Java System.getProperty("line.separator")
    fn get_line_separator() -> &'static str {
        if cfg!(windows) {
            "\r\n"
        } else {
            "\n"
        }
    }

    /// Check if file exists and is readable
    pub fn can_read<P: AsRef<Path>>(file_path: P) -> bool {
        file_path.as_ref().exists() && file_path.as_ref().is_file()
    }

    /// Check if file can be written to (or created)
    pub fn can_write<P: AsRef<Path>>(file_path: P) -> bool {
        let path = file_path.as_ref();
        
        if path.exists() {
            // Check if existing file is writable
            path.metadata()
                .map(|metadata| !metadata.permissions().readonly())
                .unwrap_or(false)
        } else {
            // Check if parent directory exists and is writable
            path.parent()
                .map(|parent| parent.exists() && parent.is_dir())
                .unwrap_or(false)
        }
    }

    /// Get file extension
    pub fn get_extension<P: AsRef<Path>>(file_path: P) -> Option<String> {
        file_path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
    }

    /// Check if file is a VHDL file
    pub fn is_vhdl_file<P: AsRef<Path>>(file_path: P) -> bool {
        matches!(Self::get_extension(file_path).as_deref(), Some("vhd") | Some("vhdl"))
    }

    /// Check if file is a BLIF file
    pub fn is_blif_file<P: AsRef<Path>>(file_path: P) -> bool {
        matches!(Self::get_extension(file_path).as_deref(), Some("blif"))
    }

    /// Check if file is a Verilog file
    pub fn is_verilog_file<P: AsRef<Path>>(file_path: P) -> bool {
        matches!(Self::get_extension(file_path).as_deref(), Some("v") | Some("sv"))
    }

    /// Get HDL file type from extension
    pub fn get_hdl_type<P: AsRef<Path>>(file_path: P) -> Option<HdlFileType> {
        match Self::get_extension(file_path).as_deref() {
            Some("vhd") | Some("vhdl") => Some(HdlFileType::Vhdl),
            Some("blif") => Some(HdlFileType::Blif),
            Some("v") | Some("sv") => Some(HdlFileType::Verilog),
            _ => None,
        }
    }
}

/// HDL file types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdlFileType {
    /// VHDL files (.vhd, .vhdl)
    Vhdl,
    /// BLIF files (.blif)
    Blif,
    /// Verilog files (.v, .sv)
    Verilog,
}

impl HdlFileType {
    /// Get default extension for file type
    pub fn default_extension(&self) -> &'static str {
        match self {
            HdlFileType::Vhdl => "vhdl",
            HdlFileType::Blif => "blif",
            HdlFileType::Verilog => "v",
        }
    }

    /// Get all extensions for file type
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            HdlFileType::Vhdl => &["vhd", "vhdl"],
            HdlFileType::Blif => &["blif"],
            HdlFileType::Verilog => &["v", "sv"],
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            HdlFileType::Vhdl => "VHDL",
            HdlFileType::Blif => "BLIF",
            HdlFileType::Verilog => "Verilog",
        }
    }
}

/// HDL file filter for file dialogs
#[derive(Debug, Clone)]
pub struct HdlFileFilter {
    file_type: HdlFileType,
    description: String,
}

impl HdlFileFilter {
    /// Create a new HDL file filter
    pub fn new(file_type: HdlFileType) -> Self {
        let extensions = file_type.extensions();
        let ext_str = extensions.join(", .");
        let description = format!("{} Files (*.{})", file_type.display_name(), ext_str);
        
        Self {
            file_type,
            description,
        }
    }

    /// Get the file type
    pub fn file_type(&self) -> HdlFileType {
        self.file_type
    }

    /// Get the description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Check if file matches this filter
    pub fn accepts<P: AsRef<Path>>(&self, file_path: P) -> bool {
        if let Some(file_type) = HdlFile::get_hdl_type(file_path) {
            file_type == self.file_type
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdl::content::BasicHdlContentEditor;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_file_type_detection() {
        assert_eq!(HdlFile::get_hdl_type("test.vhdl"), Some(HdlFileType::Vhdl));
        assert_eq!(HdlFile::get_hdl_type("test.vhd"), Some(HdlFileType::Vhdl));
        assert_eq!(HdlFile::get_hdl_type("test.blif"), Some(HdlFileType::Blif));
        assert_eq!(HdlFile::get_hdl_type("test.v"), Some(HdlFileType::Verilog));
        assert_eq!(HdlFile::get_hdl_type("test.sv"), Some(HdlFileType::Verilog));
        assert_eq!(HdlFile::get_hdl_type("test.txt"), None);
    }

    #[test]
    fn test_file_type_methods() {
        let vhdl = HdlFileType::Vhdl;
        assert_eq!(vhdl.default_extension(), "vhdl");
        assert_eq!(vhdl.display_name(), "VHDL");
        assert_eq!(vhdl.extensions(), &["vhd", "vhdl"]);
    }

    #[test]
    fn test_file_filter() {
        let filter = HdlFileFilter::new(HdlFileType::Vhdl);
        assert_eq!(filter.file_type(), HdlFileType::Vhdl);
        assert!(filter.description().contains("VHDL"));
        
        assert!(filter.accepts("test.vhdl"));
        assert!(filter.accepts("test.vhd"));
        assert!(!filter.accepts("test.blif"));
    }

    #[test]
    fn test_file_io_operations() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.vhdl");
        
        let test_content = "entity test is\nend test;";
        
        // Test save_text
        HdlFile::save_text(&file_path, test_content)?;
        assert!(file_path.exists());
        
        // Test load
        let loaded_content = HdlFile::load(&file_path)?;
        assert!(loaded_content.contains("entity test"));
        
        // Test with editor
        let mut editor = BasicHdlContentEditor::new();
        HdlFile::open(&file_path, &mut editor)?;
        assert!(editor.get_text().contains("entity test"));
        
        Ok(())
    }

    #[test]
    fn test_file_permissions() {
        // Test with non-existent file
        assert!(!HdlFile::can_read("non_existent_file.vhdl"));
        
        // Test current directory (should be writable)
        assert!(HdlFile::can_write("test_file.vhdl"));
    }

    #[test]
    fn test_line_separator() {
        let separator = HdlFile::get_line_separator();
        if cfg!(windows) {
            assert_eq!(separator, "\r\n");
        } else {
            assert_eq!(separator, "\n");
        }
    }

    #[test]
    fn test_extension_helpers() {
        assert!(HdlFile::is_vhdl_file("test.vhdl"));
        assert!(HdlFile::is_vhdl_file("test.vhd"));
        assert!(!HdlFile::is_vhdl_file("test.blif"));
        
        assert!(HdlFile::is_blif_file("test.blif"));
        assert!(!HdlFile::is_blif_file("test.vhdl"));
        
        assert!(HdlFile::is_verilog_file("test.v"));
        assert!(HdlFile::is_verilog_file("test.sv"));
        assert!(!HdlFile::is_verilog_file("test.vhdl"));
    }
}