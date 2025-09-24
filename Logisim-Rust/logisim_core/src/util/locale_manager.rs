/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Locale management for internationalization
//! 
//! Simplified Rust port of LocaleManager.java

use crate::util::StringGetter;
use std::collections::HashMap;
use std::fmt;

/// Trait for locale change listeners
pub trait LocaleListener {
    fn locale_changed(&mut self);
}

/// A simple string getter that returns a fixed value
#[derive(Debug, Clone)]
pub struct FixedStringGetter {
    value: String,
}

impl FixedStringGetter {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl fmt::Display for FixedStringGetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl StringGetter for FixedStringGetter {}

/// A locale-aware string getter
#[derive(Debug, Clone)]
pub struct LocaleGetter {
    key: String,
    manager_id: String, // Reference to the manager
}

impl LocaleGetter {
    pub fn new(manager_id: String, key: String) -> Self {
        Self { key, manager_id }
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }
}

impl fmt::Display for LocaleGetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // In a full implementation, this would look up the string
        // from the manager. For now, return the key.
        write!(f, "{}", self.key)
    }
}

impl StringGetter for LocaleGetter {}

/// Simplified locale manager for Rust
/// 
/// This is a basic implementation that focuses on the core functionality
/// without the full complexity of the Java ResourceBundle system.
pub struct LocaleManager {
    id: String,
    strings: HashMap<String, String>,
    listeners: Vec<Box<dyn LocaleListener>>,
    current_locale: String,
}

impl LocaleManager {
    /// Create a new LocaleManager with the given identifier
    pub fn new(id: String) -> Self {
        Self {
            id,
            strings: HashMap::new(),
            listeners: Vec::new(),
            current_locale: "en".to_string(),
        }
    }

    /// Get a localized string by key
    pub fn get(&self, key: &str) -> String {
        self.strings.get(key).cloned().unwrap_or_else(|| key.to_string())
    }

    /// Get a formatted localized string
    pub fn get_formatted(&self, key: &str, args: &[&dyn fmt::Display]) -> String {
        let template = self.get(key);
        // Simple string replacement - in a full implementation this would
        // use proper formatting like Java's MessageFormat
        let mut result = template;
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, &arg.to_string());
        }
        result
    }

    /// Create a StringGetter for the given key
    pub fn getter(&self, key: &str) -> LocaleGetter {
        LocaleGetter::new(self.id.clone(), key.to_string())
    }

    /// Create a fixed string getter (for non-localizable strings)
    pub fn fixed_string(&self, value: &str) -> FixedStringGetter {
        FixedStringGetter::new(value.to_string())
    }

    /// Add a string to the locale
    pub fn add_string(&mut self, key: String, value: String) {
        self.strings.insert(key, value);
    }

    /// Load strings from a map
    pub fn load_strings(&mut self, strings: HashMap<String, String>) {
        self.strings.extend(strings);
    }

    /// Get the current locale
    pub fn current_locale(&self) -> &str {
        &self.current_locale
    }

    /// Set the current locale
    pub fn set_locale(&mut self, locale: String) {
        if self.current_locale != locale {
            self.current_locale = locale;
            self.notify_listeners();
        }
    }

    /// Add a locale listener
    pub fn add_listener(&mut self, listener: Box<dyn LocaleListener>) {
        self.listeners.push(listener);
    }

    /// Notify all listeners of locale change
    fn notify_listeners(&mut self) {
        for listener in &mut self.listeners {
            listener.locale_changed();
        }
    }

    /// Get all available locales (simplified)
    pub fn get_available_locales(&self) -> Vec<String> {
        vec!["en".to_string(), "es".to_string(), "fr".to_string(), "de".to_string()]
    }

    /// Check if a key exists
    pub fn has_key(&self, key: &str) -> bool {
        self.strings.contains_key(key)
    }

    /// Get all keys
    pub fn get_keys(&self) -> Vec<String> {
        self.strings.keys().cloned().collect()
    }
}

impl Default for LocaleManager {
    fn default() -> Self {
        let mut manager = Self::new("default".to_string());
        
        // Add some basic English strings
        manager.add_string("ok".to_string(), "OK".to_string());
        manager.add_string("cancel".to_string(), "Cancel".to_string());
        manager.add_string("yes".to_string(), "Yes".to_string());
        manager.add_string("no".to_string(), "No".to_string());
        manager.add_string("error".to_string(), "Error".to_string());
        manager.add_string("warning".to_string(), "Warning".to_string());
        manager.add_string("info".to_string(), "Information".to_string());
        
        manager
    }
}

/// Global locale manager instance
static mut GLOBAL_LOCALE_MANAGER: Option<LocaleManager> = None;
static mut GLOBAL_MANAGER_INIT: std::sync::Once = std::sync::Once::new();

/// Get the global locale manager
pub fn get_global_locale_manager() -> &'static mut LocaleManager {
    unsafe {
        GLOBAL_MANAGER_INIT.call_once(|| {
            GLOBAL_LOCALE_MANAGER = Some(LocaleManager::default());
        });
        GLOBAL_LOCALE_MANAGER.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        called: bool,
    }

    impl TestListener {
        fn new() -> Self {
            Self { called: false }
        }
    }

    impl LocaleListener for TestListener {
        fn locale_changed(&mut self) {
            self.called = true;
        }
    }

    #[test]
    fn test_locale_manager_creation() {
        let manager = LocaleManager::new("test".to_string());
        assert_eq!(manager.current_locale(), "en");
        assert_eq!(manager.get("nonexistent"), "nonexistent");
    }

    #[test]
    fn test_string_operations() {
        let mut manager = LocaleManager::new("test".to_string());
        
        manager.add_string("hello".to_string(), "Hello, World!".to_string());
        assert_eq!(manager.get("hello"), "Hello, World!");
        assert!(manager.has_key("hello"));
        assert!(!manager.has_key("nonexistent"));
        
        let keys = manager.get_keys();
        assert!(keys.contains(&"hello".to_string()));
    }

    #[test]
    fn test_formatted_strings() {
        let mut manager = LocaleManager::new("test".to_string());
        
        manager.add_string("greeting".to_string(), "Hello, {0}! You have {1} messages.".to_string());
        
        let name = "Alice";
        let count = 5;
        let formatted = manager.get_formatted("greeting", &[&name, &count]);
        assert_eq!(formatted, "Hello, Alice! You have 5 messages.");
    }

    #[test]
    fn test_string_getters() {
        let manager = LocaleManager::new("test".to_string());
        
        let getter = manager.getter("test_key");
        assert_eq!(getter.get_key(), "test_key");
        assert_eq!(getter.to_string(), "test_key"); // Falls back to key
        
        let fixed = manager.fixed_string("Fixed Value");
        assert_eq!(fixed.to_string(), "Fixed Value");
    }

    #[test]
    fn test_locale_change() {
        let mut manager = LocaleManager::new("test".to_string());
        
        // Test locale change
        manager.set_locale("es".to_string());
        assert_eq!(manager.current_locale(), "es");
        
        // Test available locales
        let locales = manager.get_available_locales();
        assert!(locales.contains(&"en".to_string()));
        assert!(locales.contains(&"es".to_string()));
    }

    #[test]
    fn test_load_strings() {
        let mut manager = LocaleManager::new("test".to_string());
        
        let mut strings = HashMap::new();
        strings.insert("key1".to_string(), "value1".to_string());
        strings.insert("key2".to_string(), "value2".to_string());
        
        manager.load_strings(strings);
        
        assert_eq!(manager.get("key1"), "value1");
        assert_eq!(manager.get("key2"), "value2");
        assert!(manager.has_key("key1"));
        assert!(manager.has_key("key2"));
    }

    #[test]
    fn test_default_manager() {
        let manager = LocaleManager::default();
        
        // Should have some basic strings
        assert_eq!(manager.get("ok"), "OK");
        assert_eq!(manager.get("cancel"), "Cancel");
        assert_eq!(manager.get("yes"), "Yes");
        assert_eq!(manager.get("no"), "No");
        
        assert!(manager.has_key("ok"));
        assert!(manager.has_key("error"));
    }

    #[test]
    fn test_global_manager() {
        let manager = get_global_locale_manager();
        assert!(manager.has_key("ok"));
        
        manager.add_string("test_global".to_string(), "Global Test".to_string());
        assert_eq!(manager.get("test_global"), "Global Test");
    }
}