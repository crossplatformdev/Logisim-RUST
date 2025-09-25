/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! HDL-specific internationalization strings

use std::collections::HashMap;
use std::sync::OnceLock;

/// HDL-specific string resources
pub struct Strings;

static STRING_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

impl Strings {
    fn initialize() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("hdlFileReaderError".to_string(), "Error reading HDL file".to_string());
        map.insert("hdlFileWriterError".to_string(), "Error writing HDL file".to_string());
        map
    }

    pub fn get(key: &str) -> String {
        let strings = STRING_MAP.get_or_init(Self::initialize);
        strings.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strings_get() {
        let result = Strings::get("hdlFileReaderError");
        assert_eq!(result, "Error reading HDL file");
    }
}
