/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! String utility functions and traits
//! 
//! Rust port of StringUtil.java and StringGetter.java

use std::fmt;

/// Trait equivalent to Java's StringGetter interface
pub trait StringGetter: fmt::Display {
    /// Get the string value
    fn get_string(&self) -> String {
        self.to_string()
    }
}

/// Simple implementation of StringGetter for constant strings
#[derive(Debug, Clone)]
pub struct ConstantStringGetter {
    value: String,
}

impl ConstantStringGetter {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl fmt::Display for ConstantStringGetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl StringGetter for ConstantStringGetter {}

/// String utility functions equivalent to Java's StringUtil class
pub struct StringUtil;

impl StringUtil {
    /// Create a constant StringGetter with the given value
    pub fn constant_getter(value: String) -> ConstantStringGetter {
        ConstantStringGetter::new(value)
    }

    /// Convert a number to hex string with specified bit width
    /// Equivalent to Java's toHexString(int bits, long value)
    pub fn to_hex_string(bits: u32, value: u64) -> String {
        let masked_value = if bits < 64 {
            value & ((1u64 << bits) - 1)
        } else {
            value
        };
        
        let len = (bits + 3) / 4;
        let hex_str = format!("{:0width$x}", masked_value, width = len as usize);
        
        // Ensure we don't exceed the expected length
        if hex_str.len() > len as usize {
            hex_str[hex_str.len() - len as usize..].to_string()
        } else {
            hex_str
        }
    }

    /// Check if a string is null or empty
    /// Equivalent to Java's isNullOrEmpty(CharSequence str)
    pub fn is_null_or_empty(s: Option<&str>) -> bool {
        match s {
            None => true,
            Some(s) => s.is_empty(),
        }
    }

    /// Check if a string is not empty and not null
    /// Equivalent to Java's isNotEmpty(CharSequence seq)
    pub fn is_not_empty(s: Option<&str>) -> bool {
        match s {
            None => false,
            Some(s) => !s.is_empty(),
        }
    }

    /// Null-safe version of starts_with
    /// Equivalent to Java's startsWith(String seq, String prefix)
    pub fn starts_with(s: Option<&str>, prefix: &str) -> bool {
        match s {
            None => false,
            Some(s) => s.starts_with(prefix),
        }
    }

    /// Resize string to fit within maximum width (simplified version)
    pub fn resize_string(value: &str, max_width: usize) -> String {
        if value.len() <= max_width {
            value.to_string()
        } else if value.len() < 4 {
            value.to_string()
        } else {
            let truncated_len = max_width.saturating_sub(3);
            format!("{}...", &value[..truncated_len])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_getter() {
        let getter = StringUtil::constant_getter("test".to_string());
        assert_eq!(getter.to_string(), "test");
        assert_eq!(getter.get_string(), "test");
    }

    #[test]
    fn test_to_hex_string() {
        assert_eq!(StringUtil::to_hex_string(4, 15), "f");
        assert_eq!(StringUtil::to_hex_string(8, 255), "ff");
        assert_eq!(StringUtil::to_hex_string(16, 65535), "ffff");
        assert_eq!(StringUtil::to_hex_string(4, 16), "0"); // 16 & 0xF = 0
    }

    #[test]
    fn test_is_null_or_empty() {
        assert!(StringUtil::is_null_or_empty(None));
        assert!(StringUtil::is_null_or_empty(Some("")));
        assert!(!StringUtil::is_null_or_empty(Some("test")));
    }

    #[test]
    fn test_is_not_empty() {
        assert!(!StringUtil::is_not_empty(None));
        assert!(!StringUtil::is_not_empty(Some("")));
        assert!(StringUtil::is_not_empty(Some("test")));
    }

    #[test]
    fn test_starts_with() {
        assert!(!StringUtil::starts_with(None, "test"));
        assert!(StringUtil::starts_with(Some("test123"), "test"));
        assert!(!StringUtil::starts_with(Some("abc"), "test"));
    }

    #[test]
    fn test_resize_string() {
        assert_eq!(StringUtil::resize_string("short", 10), "short");
        assert_eq!(StringUtil::resize_string("verylongstring", 5), "ve...");
        assert_eq!(StringUtil::resize_string("abc", 5), "abc");
    }
}