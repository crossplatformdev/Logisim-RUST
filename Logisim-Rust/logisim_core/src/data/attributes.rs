/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Attribute system for component configuration
//!
//! Rust port of Attribute.java, AttributeSet.java, and related files

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

/// Trait for attribute values that can be stored and retrieved
pub trait AttributeValue: Any + fmt::Debug + Clone + Send + Sync {
    /// Convert to a display string
    fn to_display_string(&self) -> String;

    /// Convert to a standard string (for serialization)
    fn to_standard_string(&self) -> String {
        self.to_display_string()
    }

    /// Parse from a string
    fn parse_from_string(s: &str) -> Result<Self, String>
    where
        Self: Sized;
}

/// Unique identifier for an attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttributeId {
    name: String,
    type_id: TypeId,
}

impl AttributeId {
    pub fn new<T: AttributeValue>(name: String) -> Self {
        Self {
            name,
            type_id: TypeId::of::<T>(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for AttributeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// An attribute definition with metadata
#[derive(Debug, Clone)]
pub struct Attribute<T: AttributeValue> {
    id: AttributeId,
    display_name: Option<String>,
    hidden: bool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: AttributeValue> Attribute<T> {
    /// Create a new attribute
    pub fn new(name: String) -> Self {
        Self {
            id: AttributeId::new::<T>(name),
            display_name: None,
            hidden: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a new attribute with display name
    pub fn new_with_display(name: String, display_name: String) -> Self {
        Self {
            id: AttributeId::new::<T>(name),
            display_name: Some(display_name),
            hidden: false,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create a hidden attribute
    pub fn new_hidden(name: String) -> Self {
        Self {
            id: AttributeId::new::<T>(name),
            display_name: None,
            hidden: true,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Get the attribute ID
    pub fn id(&self) -> &AttributeId {
        &self.id
    }

    /// Get the name
    pub fn get_name(&self) -> &str {
        &self.id.name
    }

    /// Get the display name
    pub fn get_display_name(&self) -> String {
        match &self.display_name {
            Some(name) => name.clone(),
            None => self.id.name.clone(),
        }
    }

    /// Check if hidden
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Set hidden status
    pub fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    /// Parse a value from string
    pub fn parse(&self, value: &str) -> Result<T, String> {
        T::parse_from_string(value)
    }

    /// Convert value to display string
    pub fn to_display_string(&self, value: &T) -> String {
        value.to_display_string()
    }
}

impl<T: AttributeValue> PartialEq for Attribute<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: AttributeValue> Eq for Attribute<T> {}

impl<T: AttributeValue> Hash for Attribute<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Stores attribute values with type safety
#[derive(Debug)]
pub struct AttributeSet {
    values: HashMap<AttributeId, Box<dyn Any + Send + Sync>>,
    read_only: HashMap<AttributeId, bool>,
}

impl AttributeSet {
    /// Create a new attribute set
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            read_only: HashMap::new(),
        }
    }

    /// Check if an attribute is contained
    pub fn contains_attribute<T: AttributeValue>(&self, attr: &Attribute<T>) -> bool {
        self.values.contains_key(attr.id())
    }

    /// Get an attribute value
    pub fn get_value<T: AttributeValue>(&self, attr: &Attribute<T>) -> Option<&T> {
        self.values
            .get(attr.id())
            .and_then(|v| v.downcast_ref::<T>())
    }

    /// Set an attribute value
    pub fn set_value<T: AttributeValue>(
        &mut self,
        attr: &Attribute<T>,
        value: T,
    ) -> Result<(), String> {
        if self.is_read_only(attr) {
            return Err(format!("Attribute '{}' is read-only", attr.get_name()));
        }

        self.values.insert(attr.id().clone(), Box::new(value));
        Ok(())
    }

    /// Check if an attribute is read-only
    pub fn is_read_only<T: AttributeValue>(&self, attr: &Attribute<T>) -> bool {
        self.read_only.get(attr.id()).copied().unwrap_or(false)
    }

    /// Set read-only status
    pub fn set_read_only<T: AttributeValue>(&mut self, attr: &Attribute<T>, read_only: bool) {
        self.read_only.insert(attr.id().clone(), read_only);
    }

    /// Get all attribute IDs
    pub fn get_attribute_ids(&self) -> Vec<&AttributeId> {
        self.values.keys().collect()
    }
}

impl Default for AttributeSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Event fired when an attribute changes  
#[derive(Debug)]
pub struct AttributeEvent {
    attribute_id: AttributeId,
}

impl AttributeEvent {
    fn new(attribute_id: AttributeId) -> Self {
        Self { attribute_id }
    }

    pub fn get_attribute_id(&self) -> &AttributeId {
        &self.attribute_id
    }
}

/// Trait for listening to attribute changes
pub trait AttributeListener: Send + Sync {
    fn attribute_changed(&mut self, event: &AttributeEvent);
}

/// An attribute option for dropdowns and selections
#[derive(Clone)]
pub struct AttributeOption<T: AttributeValue> {
    value: T,
    name: String,
    display_name: Option<String>,
}

impl<T: AttributeValue> AttributeOption<T> {
    /// Create a new attribute option
    pub fn new(value: T, name: String) -> Self {
        Self {
            value,
            name,
            display_name: None,
        }
    }

    /// Create a new attribute option with display getter
    pub fn new_with_display(value: T, name: String, display_name: String) -> Self {
        Self {
            value,
            name,
            display_name: Some(display_name),
        }
    }

    /// Get the value
    pub fn get_value(&self) -> &T {
        &self.value
    }

    /// Get the name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the display string
    pub fn to_display_string(&self) -> String {
        match &self.display_name {
            Some(name) => name.clone(),
            None => self.name.clone(),
        }
    }
}

// Standard attribute value implementations

impl AttributeValue for String {
    fn to_display_string(&self) -> String {
        self.clone()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        Ok(s.to_string())
    }
}

impl AttributeValue for i32 {
    fn to_display_string(&self) -> String {
        self.to_string()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        s.parse().map_err(|e| format!("Invalid integer: {}", e))
    }
}

impl AttributeValue for u32 {
    fn to_display_string(&self) -> String {
        self.to_string()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        s.parse()
            .map_err(|e| format!("Invalid unsigned integer: {}", e))
    }
}

impl AttributeValue for bool {
    fn to_display_string(&self) -> String {
        if *self {
            "true".to_string()
        } else {
            "false".to_string()
        }
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => Ok(true),
            "false" | "0" | "no" | "off" => Ok(false),
            _ => Err(format!("Invalid boolean: {}", s)),
        }
    }
}

impl AttributeValue for super::Direction {
    fn to_display_string(&self) -> String {
        match self {
            super::Direction::East => "East".to_string(),
            super::Direction::North => "North".to_string(),
            super::Direction::West => "West".to_string(),
            super::Direction::South => "South".to_string(),
        }
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        super::Direction::parse(s)
    }
}

impl AttributeValue for super::BitWidth {
    fn to_display_string(&self) -> String {
        self.to_string()
    }

    fn parse_from_string(s: &str) -> Result<Self, String> {
        super::BitWidth::parse(s)
    }
}

/// Common standard attributes
pub struct StdAttr;

impl StdAttr {
    /// Direction attribute
    pub fn facing() -> Attribute<super::Direction> {
        Attribute::new("facing".to_string())
    }

    /// Width attribute  
    pub fn width() -> Attribute<super::BitWidth> {
        Attribute::new("width".to_string())
    }

    /// Label attribute
    pub fn label() -> Attribute<String> {
        Attribute::new("label".to_string())
    }

    /// Number of inputs attribute
    pub fn input_count() -> Attribute<u32> {
        Attribute::new("inputs".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{BitWidth, Direction};

    #[test]
    fn test_attribute_creation() {
        let attr: Attribute<String> = Attribute::new("test".to_string());
        assert_eq!(attr.get_name(), "test");
        assert!(!attr.is_hidden());
    }

    #[test]
    fn test_attribute_set_basic() {
        let mut set = AttributeSet::new();
        let attr = Attribute::new("test".to_string());

        assert!(!set.contains_attribute(&attr));

        set.set_value(&attr, "hello".to_string()).unwrap();
        assert!(set.contains_attribute(&attr));
        assert_eq!(set.get_value(&attr), Some(&"hello".to_string()));
    }

    #[test]
    fn test_attribute_set_read_only() {
        let mut set = AttributeSet::new();
        let attr = Attribute::new("test".to_string());

        set.set_value(&attr, "initial".to_string()).unwrap();
        set.set_read_only(&attr, true);

        assert!(set.is_read_only(&attr));
        let result = set.set_value(&attr, "changed".to_string());
        assert!(result.is_err());
        assert_eq!(set.get_value(&attr), Some(&"initial".to_string()));
    }

    #[test]
    fn test_attribute_value_string() {
        assert_eq!("hello".to_string().to_display_string(), "hello");
        assert_eq!(String::parse_from_string("world").unwrap(), "world");
    }

    #[test]
    fn test_attribute_value_integer() {
        assert_eq!(42i32.to_display_string(), "42");
        assert_eq!(i32::parse_from_string("123").unwrap(), 123);
        assert!(i32::parse_from_string("invalid").is_err());
    }

    #[test]
    fn test_attribute_value_boolean() {
        assert_eq!(true.to_display_string(), "true");
        assert_eq!(false.to_display_string(), "false");

        assert_eq!(bool::parse_from_string("true").unwrap(), true);
        assert_eq!(bool::parse_from_string("false").unwrap(), false);
        assert_eq!(bool::parse_from_string("1").unwrap(), true);
        assert_eq!(bool::parse_from_string("0").unwrap(), false);
        assert!(bool::parse_from_string("maybe").is_err());
    }

    #[test]
    fn test_attribute_value_direction() {
        let dir = Direction::East;
        assert_eq!(dir.to_display_string(), "East");
        assert_eq!(
            Direction::parse_from_string("north").unwrap(),
            Direction::North
        );
    }

    #[test]
    fn test_attribute_value_bit_width() {
        let width = BitWidth::new(8);
        assert_eq!(width.to_display_string(), "8");
        assert_eq!(BitWidth::parse_from_string("16").unwrap().get_width(), 16);
    }

    #[test]
    fn test_attribute_option() {
        let option = AttributeOption::new(Direction::East, "east".to_string());
        assert_eq!(option.get_value(), &Direction::East);
        assert_eq!(option.get_name(), "east");
        assert_eq!(option.to_display_string(), "east");
    }

    #[test]
    fn test_std_attributes() {
        let facing = StdAttr::facing();
        let width = StdAttr::width();
        let label = StdAttr::label();

        assert_eq!(facing.get_name(), "facing");
        assert_eq!(width.get_name(), "width");
        assert_eq!(label.get_name(), "label");
    }

    #[test]
    fn test_attribute_ids() {
        let attr1: Attribute<String> = Attribute::new("test".to_string());
        let attr2: Attribute<String> = Attribute::new("test".to_string());
        let attr3: Attribute<i32> = Attribute::new("test".to_string());

        assert_eq!(attr1.id(), attr2.id());
        assert_ne!(attr1.id(), attr3.id()); // Different types
    }

    #[test]
    fn test_attribute_set_multiple_types() {
        let mut set = AttributeSet::new();
        let str_attr = Attribute::new("string".to_string());
        let int_attr = Attribute::new("integer".to_string());
        let bool_attr = Attribute::new("boolean".to_string());

        set.set_value(&str_attr, "hello".to_string()).unwrap();
        set.set_value(&int_attr, 42i32).unwrap();
        set.set_value(&bool_attr, true).unwrap();

        assert_eq!(set.get_value(&str_attr), Some(&"hello".to_string()));
        assert_eq!(set.get_value(&int_attr), Some(&42i32));
        assert_eq!(set.get_value(&bool_attr), Some(&true));

        let ids = set.get_attribute_ids();
        assert_eq!(ids.len(), 3);
    }
}
