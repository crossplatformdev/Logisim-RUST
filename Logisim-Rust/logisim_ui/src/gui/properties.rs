/// Complete properties system for component configuration
/// Provides comprehensive property editing with validation and real-time updates

use std::collections::HashMap;
use logisim_core::{ComponentId, BusWidth};
use crate::gui::i18n::tr;

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    Float(f64),
    Enum { value: String, options: Vec<String> },
    BitWidth(BusWidth),
    Color([u8; 3]),
}

impl PropertyValue {
    pub fn as_string(&self) -> String {
        match self {
            PropertyValue::String(s) => s.clone(),
            PropertyValue::Integer(i) => i.to_string(),
            PropertyValue::Boolean(b) => b.to_string(),
            PropertyValue::Float(f) => f.to_string(),
            PropertyValue::Enum { value, .. } => value.clone(),
            PropertyValue::BitWidth(w) => w.0.to_string(),
            PropertyValue::Color([r, g, b]) => format!("#{:02x}{:02x}{:02x}", r, g, b),
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            PropertyValue::Integer(i) => Some(*i),
            PropertyValue::BitWidth(w) => Some(w.0 as i64),
            PropertyValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            PropertyValue::Boolean(b) => Some(*b),
            PropertyValue::String(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Some(true),
                "false" | "no" | "0" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            PropertyValue::Float(f) => Some(*f),
            PropertyValue::Integer(i) => Some(*i as f64),
            PropertyValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PropertyType {
    String,
    Integer { min: Option<i64>, max: Option<i64> },
    Boolean,
    Float { min: Option<f64>, max: Option<f64> },
    Enum { options: Vec<String> },
    BitWidth { min: u32, max: u32 },
    Color,
}

#[derive(Debug, Clone)]
pub struct PropertyDescriptor {
    pub name: String,
    pub key: String,
    pub property_type: PropertyType,
    pub default_value: PropertyValue,
    pub description: String,
    pub category: String,
    pub read_only: bool,
    pub visible: bool,
}

impl PropertyDescriptor {
    pub fn new_string(key: &str, name: &str, default: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            property_type: PropertyType::String,
            default_value: PropertyValue::String(default.to_string()),
            description: String::new(),
            category: "General".to_string(),
            read_only: false,
            visible: true,
        }
    }

    pub fn new_integer(key: &str, name: &str, default: i64) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            property_type: PropertyType::Integer { min: None, max: None },
            default_value: PropertyValue::Integer(default),
            description: String::new(),
            category: "General".to_string(),
            read_only: false,
            visible: true,
        }
    }

    pub fn new_boolean(key: &str, name: &str, default: bool) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            property_type: PropertyType::Boolean,
            default_value: PropertyValue::Boolean(default),
            description: String::new(),
            category: "General".to_string(),
            read_only: false,
            visible: true,
        }
    }

    pub fn new_enum(key: &str, name: &str, options: Vec<String>, default: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            property_type: PropertyType::Enum { options: options.clone() },
            default_value: PropertyValue::Enum { 
                value: default.to_string(), 
                options 
            },
            description: String::new(),
            category: "General".to_string(),
            read_only: false,
            visible: true,
        }
    }

    pub fn new_bit_width(key: &str, name: &str, default: u32) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string(),
            property_type: PropertyType::BitWidth { min: 1, max: 32 },
            default_value: PropertyValue::BitWidth(BusWidth(default)),
            description: String::new(),
            category: "General".to_string(),
            read_only: false,
            visible: true,
        }
    }

    pub fn with_range(mut self, min: i64, max: i64) -> Self {
        if let PropertyType::Integer { .. } = self.property_type {
            self.property_type = PropertyType::Integer { min: Some(min), max: Some(max) };
        }
        self
    }

    pub fn with_float_range(mut self, min: f64, max: f64) -> Self {
        if let PropertyType::Float { .. } = self.property_type {
            self.property_type = PropertyType::Float { min: Some(min), max: Some(max) };
        }
        self
    }

    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.to_string();
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }
}

#[derive(Debug, Clone)]
pub struct ComponentProperties {
    pub component_id: ComponentId,
    pub component_name: String,
    pub properties: HashMap<String, PropertyValue>,
    pub descriptors: HashMap<String, PropertyDescriptor>,
    pub categories: Vec<String>,
}

impl ComponentProperties {
    pub fn new(component_id: ComponentId, component_name: &str) -> Self {
        Self {
            component_id,
            component_name: component_name.to_string(),
            properties: HashMap::new(),
            descriptors: HashMap::new(),
            categories: vec!["General".to_string()],
        }
    }

    pub fn add_property(&mut self, descriptor: PropertyDescriptor) {
        let key = descriptor.key.clone();
        let default_value = descriptor.default_value.clone();
        
        if !self.categories.contains(&descriptor.category) {
            self.categories.push(descriptor.category.clone());
        }
        
        self.descriptors.insert(key.clone(), descriptor);
        self.properties.insert(key, default_value);
    }

    pub fn set_property(&mut self, key: &str, value: PropertyValue) -> Result<(), String> {
        if let Some(descriptor) = self.descriptors.get(key) {
            if descriptor.read_only {
                return Err("Property is read-only".to_string());
            }

            // Validate value
            self.validate_property_value(key, &value)?;
            self.properties.insert(key.to_string(), value);
            Ok(())
        } else {
            Err(format!("Property '{}' not found", key))
        }
    }

    pub fn get_property(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }

    pub fn validate_property_value(&self, key: &str, value: &PropertyValue) -> Result<(), String> {
        if let Some(descriptor) = self.descriptors.get(key) {
            match (&descriptor.property_type, value) {
                (PropertyType::Integer { min, max }, PropertyValue::Integer(i)) => {
                    if let Some(min_val) = min {
                        if *i < *min_val {
                            return Err(format!("Value {} is below minimum {}", i, min_val));
                        }
                    }
                    if let Some(max_val) = max {
                        if *i > *max_val {
                            return Err(format!("Value {} is above maximum {}", i, max_val));
                        }
                    }
                },
                (PropertyType::Float { min, max }, PropertyValue::Float(f)) => {
                    if let Some(min_val) = min {
                        if *f < *min_val {
                            return Err(format!("Value {} is below minimum {}", f, min_val));
                        }
                    }
                    if let Some(max_val) = max {
                        if *f > *max_val {
                            return Err(format!("Value {} is above maximum {}", f, max_val));
                        }
                    }
                },
                (PropertyType::Enum { options }, PropertyValue::Enum { value, .. }) => {
                    if !options.contains(value) {
                        return Err(format!("Value '{}' is not a valid option", value));
                    }
                },
                (PropertyType::BitWidth { min, max }, PropertyValue::BitWidth(w)) => {
                    if w.0 < *min {
                        return Err(format!("Bit width {} is below minimum {}", w.0, min));
                    }
                    if w.0 > *max {
                        return Err(format!("Bit width {} is above maximum {}", w.0, max));
                    }
                },
                _ => {
                    // Type mismatch - could try to convert
                    return Err(format!("Type mismatch for property '{}'", key));
                }
            }
        }
        Ok(())
    }

    pub fn get_properties_by_category(&self) -> HashMap<String, Vec<String>> {
        let mut categorized = HashMap::new();
        
        for (key, descriptor) in &self.descriptors {
            if descriptor.visible {
                let category_props = categorized.entry(descriptor.category.clone()).or_insert_with(Vec::new);
                category_props.push(key.clone());
            }
        }
        
        categorized
    }

    pub fn reset_to_defaults(&mut self) {
        for (key, descriptor) in &self.descriptors {
            self.properties.insert(key.clone(), descriptor.default_value.clone());
        }
    }

    pub fn export_properties(&self) -> HashMap<String, String> {
        self.properties
            .iter()
            .map(|(k, v)| (k.clone(), v.as_string()))
            .collect()
    }

    pub fn import_properties(&mut self, props: HashMap<String, String>) -> Vec<String> {
        let mut errors = Vec::new();
        
        for (key, value_str) in props {
            if let Some(descriptor) = self.descriptors.get(&key) {
                let value = match &descriptor.property_type {
                    PropertyType::String => PropertyValue::String(value_str),
                    PropertyType::Integer { .. } => {
                        match value_str.parse::<i64>() {
                            Ok(i) => PropertyValue::Integer(i),
                            Err(_) => {
                                errors.push(format!("Invalid integer value for '{}': {}", key, value_str));
                                continue;
                            }
                        }
                    },
                    PropertyType::Boolean => {
                        match value_str.to_lowercase().as_str() {
                            "true" | "yes" | "1" => PropertyValue::Boolean(true),
                            "false" | "no" | "0" => PropertyValue::Boolean(false),
                            _ => {
                                errors.push(format!("Invalid boolean value for '{}': {}", key, value_str));
                                continue;
                            }
                        }
                    },
                    PropertyType::Float { .. } => {
                        match value_str.parse::<f64>() {
                            Ok(f) => PropertyValue::Float(f),
                            Err(_) => {
                                errors.push(format!("Invalid float value for '{}': {}", key, value_str));
                                continue;
                            }
                        }
                    },
                    PropertyType::Enum { options } => {
                        if options.contains(&value_str) {
                            PropertyValue::Enum { value: value_str, options: options.clone() }
                        } else {
                            errors.push(format!("Invalid enum value for '{}': {}", key, value_str));
                            continue;
                        }
                    },
                    PropertyType::BitWidth { .. } => {
                        match value_str.parse::<u32>() {
                            Ok(w) => PropertyValue::BitWidth(BusWidth(w)),
                            Err(_) => {
                                errors.push(format!("Invalid bit width value for '{}': {}", key, value_str));
                                continue;
                            }
                        }
                    },
                    PropertyType::Color => {
                        // Parse hex color #RRGGBB
                        if value_str.starts_with('#') && value_str.len() == 7 {
                            if let (Ok(r), Ok(g), Ok(b)) = (
                                u8::from_str_radix(&value_str[1..3], 16),
                                u8::from_str_radix(&value_str[3..5], 16),
                                u8::from_str_radix(&value_str[5..7], 16),
                            ) {
                                PropertyValue::Color([r, g, b])
                            } else {
                                errors.push(format!("Invalid color value for '{}': {}", key, value_str));
                                continue;
                            }
                        } else {
                            errors.push(format!("Invalid color format for '{}': {}", key, value_str));
                            continue;
                        }
                    }
                };

                if let Err(err) = self.set_property(&key, value) {
                    errors.push(format!("Error setting '{}': {}", key, err));
                }
            } else {
                errors.push(format!("Unknown property: {}", key));
            }
        }
        
        errors
    }
}

pub fn create_standard_properties(component_name: &str, component_id: ComponentId) -> ComponentProperties {
    let mut props = ComponentProperties::new(component_id, component_name);
    
    // Standard properties for all components
    props.add_property(
        PropertyDescriptor::new_string("label", &tr("properties.label"), "")
            .with_description("Custom label for this component")
    );
    
    props.add_property(
        PropertyDescriptor::new_enum(
            "facing",
            &tr("properties.facing"),
            vec!["East".to_string(), "West".to_string(), "North".to_string(), "South".to_string()],
            "East"
        ).with_description("Direction the component faces")
    );

    // Component-specific properties
    match component_name {
        "AND Gate" | "OR Gate" | "XOR Gate" | "NAND Gate" | "NOR Gate" | "XNOR Gate" => {
            props.add_property(
                PropertyDescriptor::new_integer("inputs", &tr("properties.inputs"), 2)
                    .with_range(2, 32)
                    .with_description("Number of input pins")
            );
            
            props.add_property(
                PropertyDescriptor::new_bit_width("width", &tr("properties.width"), 1)
                    .with_description("Bit width of inputs and output")
            );
        },
        "D Flip-Flop" | "JK Flip-Flop" => {
            props.add_property(
                PropertyDescriptor::new_enum(
                    "trigger",
                    &tr("properties.trigger"),
                    vec!["Rising Edge".to_string(), "Falling Edge".to_string(), "High Level".to_string(), "Low Level".to_string()],
                    "Rising Edge"
                ).with_description("Clock trigger type")
            );
        },
        "Register" | "Counter" => {
            props.add_property(
                PropertyDescriptor::new_bit_width("width", &tr("properties.width"), 8)
                    .with_description("Bit width of the register/counter")
            );
        },
        "RAM" | "ROM" => {
            props.add_property(
                PropertyDescriptor::new_integer("address_bits", "Address Bits", 8)
                    .with_range(1, 20)
                    .with_description("Number of address bits")
            );
            
            props.add_property(
                PropertyDescriptor::new_bit_width("data_width", &tr("properties.width"), 8)
                    .with_description("Bit width of data")
            );
        },
        "Input Pin" | "Output Pin" => {
            props.add_property(
                PropertyDescriptor::new_bit_width("width", &tr("properties.width"), 1)
                    .with_description("Bit width of the pin")
            );
        },
        _ => {
            // Default properties for unknown components
        }
    }
    
    props
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_value_conversion() {
        let int_val = PropertyValue::Integer(42);
        assert_eq!(int_val.as_string(), "42");
        assert_eq!(int_val.as_integer(), Some(42));

        let bool_val = PropertyValue::Boolean(true);
        assert_eq!(bool_val.as_string(), "true");
        assert_eq!(bool_val.as_boolean(), Some(true));
    }

    #[test]
    fn test_component_properties() {
        let mut props = ComponentProperties::new(ComponentId(1), "Test Component");
        
        let desc = PropertyDescriptor::new_integer("test_prop", "Test Property", 10)
            .with_range(1, 100);
        props.add_property(desc);
        
        assert_eq!(props.get_property("test_prop"), Some(&PropertyValue::Integer(10)));
        
        // Test validation
        assert!(props.set_property("test_prop", PropertyValue::Integer(50)).is_ok());
        assert!(props.set_property("test_prop", PropertyValue::Integer(200)).is_err());
    }

    #[test]
    fn test_property_categories() {
        let mut props = ComponentProperties::new(ComponentId(1), "Test");
        
        props.add_property(
            PropertyDescriptor::new_string("name", "Name", "test")
                .with_category("Basic")
        );
        
        props.add_property(
            PropertyDescriptor::new_integer("value", "Value", 0)
                .with_category("Advanced")
        );
        
        let categorized = props.get_properties_by_category();
        assert_eq!(categorized.len(), 2);
        assert!(categorized.contains_key("Basic"));
        assert!(categorized.contains_key("Advanced"));
    }

    #[test]
    fn test_property_export_import() {
        let mut props = ComponentProperties::new(ComponentId(1), "Test");
        
        props.add_property(PropertyDescriptor::new_string("name", "Name", "default"));
        props.add_property(PropertyDescriptor::new_integer("count", "Count", 5));
        
        let exported = props.export_properties();
        assert_eq!(exported.get("name"), Some(&"default".to_string()));
        assert_eq!(exported.get("count"), Some(&"5".to_string()));
        
        let mut new_values = HashMap::new();
        new_values.insert("name".to_string(), "updated".to_string());
        new_values.insert("count".to_string(), "10".to_string());
        
        let errors = props.import_properties(new_values);
        assert!(errors.is_empty());
        assert_eq!(props.get_property("name"), Some(&PropertyValue::String("updated".to_string())));
        assert_eq!(props.get_property("count"), Some(&PropertyValue::Integer(10)));
    }
}