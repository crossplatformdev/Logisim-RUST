/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! File utility functions
//! 
//! Rust port of FileUtil.java

use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

/// File utility functions equivalent to Java's FileUtil class
pub struct FileUtil;

impl FileUtil {
    /// Correct a path to ensure it ends with a path separator
    /// Equivalent to Java's correctPath(String path)
    pub fn correct_path(path: &str) -> String {
        if path.ends_with(std::path::MAIN_SEPARATOR) {
            path.to_string()
        } else {
            format!("{}{}", path, std::path::MAIN_SEPARATOR)
        }
    }

    /// Create a temporary file with the given content, prefix, and suffix
    /// Equivalent to Java's createTmpFile(String content, String prefix, String suffix)
    pub fn create_tmp_file(content: &str, prefix: &str, suffix: &str) -> io::Result<PathBuf> {
        // Create a temporary file name
        let temp_dir = std::env::temp_dir();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let filename = format!("{}{}{}", prefix, timestamp, suffix);
        let temp_path = temp_dir.join(filename);

        // Write content to the file
        let file = File::create(&temp_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes())?;
        writer.flush()?;

        Ok(temp_path)
    }

    /// Create a temporary file with auto-generated name
    pub fn create_temp_file() -> io::Result<PathBuf> {
        Self::create_tmp_file("", "logisim_", ".tmp")
    }

    /// Read all bytes from a reader
    /// Equivalent to Java's getBytes(InputStream is)
    pub fn get_bytes<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Read all bytes from a file
    pub fn read_file_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        Self::get_bytes(&mut file)
    }

    /// Read all text from a file as UTF-8
    pub fn read_file_text<P: AsRef<Path>>(path: P) -> io::Result<String> {
        let bytes = Self::read_file_bytes(path)?;
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Write bytes to a file
    pub fn write_file_bytes<P: AsRef<Path>>(path: P, bytes: &[u8]) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(bytes)?;
        file.flush()
    }

    /// Write text to a file as UTF-8
    pub fn write_file_text<P: AsRef<Path>>(path: P, text: &str) -> io::Result<()> {
        Self::write_file_bytes(path, text.as_bytes())
    }

    /// Append text to a file
    pub fn append_file_text<P: AsRef<Path>>(path: P, text: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(text.as_bytes())?;
        file.flush()
    }

    /// Check if a file exists and is readable
    pub fn is_readable<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_file()
    }

    /// Check if a directory exists and is readable
    pub fn is_directory<P: AsRef<Path>>(path: P) -> bool {
        path.as_ref().exists() && path.as_ref().is_dir()
    }

    /// Get file extension (without the dot)
    pub fn get_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Change file extension
    pub fn change_extension<P: AsRef<Path>>(path: P, new_extension: &str) -> PathBuf {
        let path = path.as_ref();
        let mut new_path = path.to_path_buf();
        new_path.set_extension(new_extension);
        new_path
    }

    /// Get file name without extension
    pub fn get_name_without_extension<P: AsRef<Path>>(path: P) -> Option<String> {
        path.as_ref()
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(|s| s.to_string())
    }

    /// Ensure directory exists, creating it if necessary
    pub fn ensure_directory<P: AsRef<Path>>(path: P) -> io::Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_correct_path() {
        let path1 = "/home/user";
        let path2 = "/home/user/";
        
        let corrected1 = FileUtil::correct_path(path1);
        let corrected2 = FileUtil::correct_path(path2);
        
        assert!(corrected1.ends_with(std::path::MAIN_SEPARATOR));
        assert!(corrected2.ends_with(std::path::MAIN_SEPARATOR));
        assert_eq!(corrected1, corrected2);
    }

    #[test]
    fn test_create_tmp_file() {
        let content = "Hello, World!";
        let result = FileUtil::create_tmp_file(content, "test_", ".txt");
        
        assert!(result.is_ok());
        let temp_path = result.unwrap();
        assert!(temp_path.exists());
        
        // Verify content
        let read_content = fs::read_to_string(&temp_path).unwrap();
        assert_eq!(read_content, content);
        
        // Cleanup
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_create_temp_file() {
        let result = FileUtil::create_temp_file();
        assert!(result.is_ok());
        
        let temp_path = result.unwrap();
        assert!(temp_path.exists());
        
        // Cleanup
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_get_bytes() {
        let data = b"Hello, World!";
        let cursor = std::io::Cursor::new(data);
        
        let result = FileUtil::get_bytes(cursor);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data);
    }

    #[test]
    fn test_file_operations() {
        let temp_path = std::env::temp_dir().join("test_file_util.txt");
        let content = "Test content for file operations";
        
        // Write file
        let write_result = FileUtil::write_file_text(&temp_path, content);
        assert!(write_result.is_ok());
        assert!(temp_path.exists());
        
        // Read file
        let read_result = FileUtil::read_file_text(&temp_path);
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), content);
        
        // Read bytes
        let bytes_result = FileUtil::read_file_bytes(&temp_path);
        assert!(bytes_result.is_ok());
        assert_eq!(bytes_result.unwrap(), content.as_bytes());
        
        // Append to file
        let append_text = "\nAppended content";
        let append_result = FileUtil::append_file_text(&temp_path, append_text);
        assert!(append_result.is_ok());
        
        let final_content = FileUtil::read_file_text(&temp_path).unwrap();
        assert_eq!(final_content, format!("{}{}", content, append_text));
        
        // Check file properties
        assert!(FileUtil::is_readable(&temp_path));
        assert!(!FileUtil::is_directory(&temp_path));
        
        // Cleanup
        fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_file_extension_operations() {
        let path = Path::new("/home/user/document.txt");
        
        assert_eq!(FileUtil::get_extension(path), Some("txt".to_string()));
        assert_eq!(FileUtil::get_name_without_extension(path), Some("document".to_string()));
        
        let new_path = FileUtil::change_extension(path, "pdf");
        assert_eq!(new_path.to_str(), Some("/home/user/document.pdf"));
    }

    #[test]
    fn test_ensure_directory() {
        let temp_dir = std::env::temp_dir().join("test_ensure_directory");
        
        // Directory shouldn't exist initially
        assert!(!temp_dir.exists());
        
        // Create directory
        let result = FileUtil::ensure_directory(&temp_dir);
        assert!(result.is_ok());
        assert!(temp_dir.exists());
        assert!(FileUtil::is_directory(&temp_dir));
        
        // Should not error if directory already exists
        let result2 = FileUtil::ensure_directory(&temp_dir);
        assert!(result2.is_ok());
        
        // Cleanup
        fs::remove_dir(&temp_dir).ok();
    }

    #[test]
    fn test_file_properties() {
        // Test with non-existent file
        let non_existent = Path::new("/non/existent/file.txt");
        assert!(!FileUtil::is_readable(non_existent));
        assert!(!FileUtil::is_directory(non_existent));
        
        // Test with actual directory (temp dir should exist)
        let temp_dir = std::env::temp_dir();
        assert!(FileUtil::is_directory(&temp_dir));
        assert!(!FileUtil::is_readable(&temp_dir)); // Directory is not a readable file
    }
}