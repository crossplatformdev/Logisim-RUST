/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL Content Editor
//!
//! Equivalent to Java HdlContentEditor.java
//! Provides editing functionality for HDL content.

use crate::hdl::HdlContent;
use std::collections::HashMap;

/// HDL Content Editor
/// 
/// Provides editing capabilities for HDL content components.
/// Equivalent to Java HdlContentEditor.
#[derive(Debug)]
pub struct HdlContentEditor {
    content: String,
    modified: bool,
    syntax_errors: Vec<String>,
    line_numbers: bool,
}

impl HdlContentEditor {
    /// Create a new HDL content editor
    pub fn new() -> Self {
        Self {
            content: String::new(),
            modified: false,
            syntax_errors: Vec::new(),
            line_numbers: true,
        }
    }

    /// Create a new HDL content editor with initial content
    pub fn with_content(content: String) -> Self {
        Self {
            content,
            modified: false,
            syntax_errors: Vec::new(),
            line_numbers: true,
        }
    }

    /// Get the current content
    pub fn get_content(&self) -> &str {
        &self.content
    }

    /// Set the content
    pub fn set_content(&mut self, content: String) {
        if self.content != content {
            self.content = content;
            self.modified = true;
            self.syntax_errors.clear();
        }
    }

    /// Check if content has been modified
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark content as saved
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Enable or disable line numbers
    pub fn set_line_numbers(&mut self, enabled: bool) {
        self.line_numbers = enabled;
    }

    /// Check if line numbers are enabled
    pub fn has_line_numbers(&self) -> bool {
        self.line_numbers
    }

    /// Get syntax errors
    pub fn get_syntax_errors(&self) -> &[String] {
        &self.syntax_errors
    }

    /// Validate syntax and populate error list
    pub fn validate_syntax(&mut self) -> bool {
        self.syntax_errors.clear();
        
        // Basic validation - in a real implementation this would be more comprehensive
        if self.content.is_empty() {
            self.syntax_errors.push("Content cannot be empty".to_string());
            return false;
        }

        // Check for balanced parentheses/brackets
        let mut paren_count = 0;
        let mut bracket_count = 0;
        
        for ch in self.content.chars() {
            match ch {
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }

        if paren_count != 0 {
            self.syntax_errors.push("Unbalanced parentheses".to_string());
        }

        if bracket_count != 0 {
            self.syntax_errors.push("Unbalanced brackets".to_string());
        }

        self.syntax_errors.is_empty()
    }

    /// Insert text at current cursor position (simplified implementation)
    pub fn insert_text(&mut self, text: &str) {
        self.content.push_str(text);
        self.modified = true;
        self.syntax_errors.clear();
    }

    /// Replace all occurrences of a pattern with replacement text
    pub fn replace_all(&mut self, pattern: &str, replacement: &str) -> usize {
        let original = self.content.clone();
        self.content = self.content.replace(pattern, replacement);
        
        if self.content != original {
            self.modified = true;
            self.syntax_errors.clear();
        }

        // Count occurrences (simplified)
        original.matches(pattern).count()
    }

    /// Get line count
    pub fn get_line_count(&self) -> usize {
        if self.content.is_empty() {
            0
        } else {
            self.content.lines().count()
        }
    }

    /// Get character count
    pub fn get_char_count(&self) -> usize {
        self.content.len()
    }

    /// Clear all content
    pub fn clear(&mut self) {
        if !self.content.is_empty() {
            self.content.clear();
            self.modified = true;
            self.syntax_errors.clear();
        }
    }
}

impl Default for HdlContentEditor {
    fn default() -> Self {
        Self::new()
    }
}

/// Editor state for saving/restoring
#[derive(Debug, Clone)]
pub struct EditorState {
    pub content: String,
    pub modified: bool,
    pub line_numbers: bool,
}

impl HdlContentEditor {
    /// Save current editor state
    pub fn save_state(&self) -> EditorState {
        EditorState {
            content: self.content.clone(),
            modified: self.modified,
            line_numbers: self.line_numbers,
        }
    }

    /// Restore editor state
    pub fn restore_state(&mut self, state: EditorState) {
        self.content = state.content;
        self.modified = state.modified;
        self.line_numbers = state.line_numbers;
        self.syntax_errors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_creation() {
        let editor = HdlContentEditor::new();
        assert!(editor.get_content().is_empty());
        assert!(!editor.is_modified());
        assert!(editor.has_line_numbers());
    }

    #[test]
    fn test_content_modification() {
        let mut editor = HdlContentEditor::new();
        editor.set_content("test content".to_string());
        
        assert_eq!(editor.get_content(), "test content");
        assert!(editor.is_modified());
        
        editor.mark_saved();
        assert!(!editor.is_modified());
    }

    #[test]
    fn test_syntax_validation() {
        let mut editor = HdlContentEditor::new();
        editor.set_content("test(content)".to_string());
        
        assert!(editor.validate_syntax());
        assert!(editor.get_syntax_errors().is_empty());
        
        editor.set_content("test(content".to_string());
        assert!(!editor.validate_syntax());
        assert!(!editor.get_syntax_errors().is_empty());
    }

    #[test]
    fn test_text_operations() {
        let mut editor = HdlContentEditor::new();
        editor.set_content("hello world".to_string());
        
        let replacements = editor.replace_all("hello", "hi");
        assert_eq!(replacements, 1);
        assert_eq!(editor.get_content(), "hi world");
        
        editor.insert_text("!");
        assert_eq!(editor.get_content(), "hi world!");
    }

    #[test]
    fn test_state_management() {
        let mut editor = HdlContentEditor::new();
        editor.set_content("test".to_string());
        editor.set_line_numbers(false);
        
        let state = editor.save_state();
        
        editor.clear();
        assert!(editor.get_content().is_empty());
        
        editor.restore_state(state);
        assert_eq!(editor.get_content(), "test");
        assert!(!editor.has_line_numbers());
    }
}