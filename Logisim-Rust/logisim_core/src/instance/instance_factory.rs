/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Instance Factory System
//!
//! This module provides the `InstanceFactory` trait for creating and managing component
//! instances. This is equivalent to Java's `InstanceFactory` abstract class.

// ComponentFactory will be integrated later - using placeholder for now
use crate::data::{Attribute, AttributeSet, Bounds, Direction, Location};
use crate::instance::{Instance, InstancePainter, InstanceState, Port};
use std::any::Any;
use std::fmt::Debug;

/// Factory trait for creating and managing component instances.
///
/// This trait extends the basic `ComponentFactory` with instance-specific functionality
/// for component creation, configuration, and rendering. It is equivalent to Java's
/// `InstanceFactory` abstract class.
///
/// # Design Pattern
///
/// The instance factory implements the Factory pattern, where:
/// - The factory defines the component type and its properties
/// - Instances are created through `create_component()`
/// - Each instance shares the same factory but has unique state
/// - The factory handles rendering and propagation logic
///
/// # Example
///
/// ```rust
/// use logisim_core::instance::{InstanceFactory, InstanceState, InstancePainter, Port};
/// use logisim_core::{AttributeSet, Location, Bounds, Value};
///
/// struct AndGateFactory {
///     name: String,
///     ports: Vec<Port>,
/// }
///
/// impl AndGateFactory {
///     pub fn new() -> Self {
///         Self {
///             name: "AND Gate".to_string(),
///             ports: vec![
///                 Port::new(-30, -10, PortType::Input, PortWidth::fixed_bits(1)),
///                 Port::new(-30, 10, PortType::Input, PortWidth::fixed_bits(1)),
///                 Port::new(0, 0, PortType::Output, PortWidth::fixed_bits(1)),
///             ],
///         }
///     }
/// }
///
/// impl InstanceFactory for AndGateFactory {
///     fn get_name(&self) -> &str {
///         &self.name
///     }
///
///     fn get_ports(&self) -> &[Port] {
///         &self.ports
///     }
///
///     fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
///         Bounds::new(-30, -20, 30, 40)
///     }
///
///     fn paint_instance(&self, painter: &mut InstancePainter) {
///         // Draw AND gate shape
///         let bounds = painter.get_bounds();
///         painter.draw_rectangle(bounds.x(), bounds.y(), bounds.width(), bounds.height());
///         // ... additional drawing code
///     }
///
///     fn propagate(&self, state: &mut dyn InstanceState) {
///         let input_a = state.get_port_value(0);
///         let input_b = state.get_port_value(1);
///         let output = input_a.and(input_b);
///         state.set_port_value_immediate(2, output);
///     }
/// }
/// ```
pub trait InstanceFactory: Debug + Send + Sync {
    /// Returns the unique name for this component type.
    fn get_name(&self) -> &str;

    /// Creates an attribute set with default values.
    fn create_attribute_set(&self) -> AttributeSet;
    /// Returns the human-readable display name for this component type.
    ///
    /// This name appears in the component library and is used for user-facing
    /// messages and documentation.
    fn get_display_name(&self) -> String {
        self.get_name().to_string()
    }

    /// Returns the default tooltip text for components of this type.
    ///
    /// # Returns
    ///
    /// Optional tooltip string, or None for no default tooltip.
    fn get_default_tooltip(&self) -> Option<String> {
        None
    }

    /// Returns the icon name for this component type.
    ///
    /// The icon is used in the component library palette and toolbar.
    ///
    /// # Returns
    ///
    /// Optional icon name, or None to use default rendering.
    fn get_icon_name(&self) -> Option<&str> {
        None
    }

    /// Returns the port definitions for this component type.
    ///
    /// Ports define the connection points (pins) where signals can be
    /// connected to this component.
    fn get_ports(&self) -> &[Port];

    /// Returns the bounding box for components of this type.
    ///
    /// The bounds are relative to the component's location and define
    /// the area occupied by the component for collision detection and
    /// rendering.
    ///
    /// # Arguments
    ///
    /// * `attrs` - Component attributes that may affect size
    ///
    /// # Returns
    ///
    /// The bounding rectangle relative to component location.
    fn get_offset_bounds(&self, attrs: &AttributeSet) -> Bounds;

    /// Returns the facing direction attribute for this component.
    ///
    /// If this component supports rotation/orientation, this method
    /// returns the attribute that controls its facing direction.
    ///
    /// # Returns
    ///
    /// The direction attribute, or None if component doesn't rotate.
    fn get_facing_attribute(&self) -> Option<&Attribute<Direction>> {
        None
    }

    /// Checks if this component should snap to grid when placed.
    ///
    /// # Returns
    ///
    /// True if the component should snap to grid, false for free placement.
    fn should_snap(&self) -> bool {
        true
    }

    /// Creates a new component instance at the specified location.
    ///
    /// This is the main factory method that creates instances of this
    /// component type. The default implementation creates an InstanceComponent
    /// wrapper and calls `configure_new_instance()`.
    ///
    /// # Arguments
    ///
    /// * `location` - Where to place the component
    /// * `attrs` - Initial attribute values
    ///
    /// # Returns
    ///
    /// A new component instance.
    fn create_component(&self, location: Location, attrs: AttributeSet) -> Box<dyn Any>;

    /// Configures a newly created instance.
    ///
    /// This method is called after instance creation to perform any
    /// necessary initialization. The default implementation does nothing.
    ///
    /// # Arguments
    ///
    /// * `instance` - The newly created instance to configure
    fn configure_new_instance(&self, _instance: &Instance) {
        // Default: no configuration needed
    }

    /// Handles attribute changes on component instances.
    ///
    /// This method is called when an attribute value changes on a component
    /// instance. Components can use this to update their internal state
    /// or trigger recalculation.
    ///
    /// # Arguments
    ///
    /// * `instance` - The instance whose attribute changed
    /// * `attr` - The attribute that changed (erased type)
    fn instance_attribute_changed(&self, _instance: &Instance, _attr: &dyn Any) {
        // Default: no special handling needed
    }

    /// Paints the component instance.
    ///
    /// This method is responsible for drawing the component's visual
    /// representation. It receives a painter that provides drawing
    /// utilities and context.
    ///
    /// # Arguments
    ///
    /// * `painter` - Drawing context and utilities
    fn paint_instance(&self, painter: &mut InstancePainter);

    /// Paints the component icon.
    ///
    /// This method draws a small icon representation of the component
    /// for use in toolbars and component palettes.
    ///
    /// # Arguments
    ///
    /// * `painter` - Drawing context for the icon
    fn paint_icon(&self, _painter: &mut InstancePainter) {
        // Default: use normal paint_instance for icon
        // Subclasses can override for specialized icon rendering
    }

    /// Paints a ghost/preview image of the component.
    ///
    /// This method draws a translucent or outline version of the component
    /// used when dragging or placing components.
    ///
    /// # Arguments
    ///
    /// * `painter` - Drawing context for the ghost image
    fn paint_ghost(&self, painter: &mut InstancePainter) {
        // Default: use normal paint_instance for ghost
        self.paint_instance(painter);
    }

    /// Propagates signal changes through this component.
    ///
    /// This is the core simulation method that implements the component's
    /// logic. It reads input port values, performs the component's function,
    /// and sets output port values.
    ///
    /// # Arguments
    ///
    /// * `state` - Runtime state providing access to ports and data
    fn propagate(&self, state: &mut dyn InstanceState);

    /// Gets a feature object for this instance.
    ///
    /// Features provide additional capabilities like interactive poking,
    /// logging, or other specialized behaviors.
    ///
    /// # Arguments
    ///
    /// * `instance` - The instance requesting the feature
    /// * `key` - Type identifier for the requested feature
    ///
    /// # Returns
    ///
    /// Feature object if supported, None otherwise.
    fn get_instance_feature(&self, _instance: &Instance, _key: &dyn Any) -> Option<Box<dyn Any>> {
        None
    }

    /// Checks if this factory supports subcircuit menu.
    ///
    /// # Returns
    ///
    /// True if the component provides a subcircuit menu, false otherwise.
    fn provides_subcircuit_menu(&self) -> bool {
        false
    }
}

/// Builder for creating instance factories with common properties.
pub struct InstanceFactoryBuilder<T> {
    inner: T,
    display_name: Option<String>,
    tooltip: Option<String>,
    icon_name: Option<String>,
    facing_attribute: Option<Attribute<Direction>>,
    should_snap: bool,
}

impl<T> InstanceFactoryBuilder<T>
where
    T: InstanceFactory,
{
    /// Creates a new builder wrapping an existing factory.
    pub fn new(factory: T) -> Self {
        Self {
            inner: factory,
            display_name: None,
            tooltip: None,
            icon_name: None,
            facing_attribute: None,
            should_snap: true,
        }
    }

    /// Sets the display name.
    pub fn display_name(mut self, name: String) -> Self {
        self.display_name = Some(name);
        self
    }

    /// Sets the display name from a string.
    pub fn display_name_str(mut self, name: &str) -> Self {
        self.display_name = Some(name.to_string());
        self
    }

    /// Sets the default tooltip.
    pub fn tooltip(mut self, tooltip: String) -> Self {
        self.tooltip = Some(tooltip);
        self
    }

    /// Sets the default tooltip from a string.
    pub fn tooltip_str(mut self, tooltip: &str) -> Self {
        self.tooltip = Some(tooltip.to_string());
        self
    }

    /// Sets the icon name.
    pub fn icon_name(mut self, name: &str) -> Self {
        self.icon_name = Some(name.to_string());
        self
    }

    /// Sets the facing attribute.
    pub fn facing_attribute(mut self, attr: Attribute<Direction>) -> Self {
        self.facing_attribute = Some(attr);
        self
    }

    /// Sets whether the component should snap to grid.
    pub fn should_snap(mut self, snap: bool) -> Self {
        self.should_snap = snap;
        self
    }

    /// Builds the configured factory.
    pub fn build(self) -> BuiltInstanceFactory<T> {
        BuiltInstanceFactory {
            inner: self.inner,
            display_name: self.display_name,
            tooltip: self.tooltip,
            icon_name: self.icon_name,
            facing_attribute: self.facing_attribute,
            should_snap: self.should_snap,
        }
    }
}

/// Wrapper that adds configuration to an existing instance factory.
pub struct BuiltInstanceFactory<T> {
    inner: T,
    display_name: Option<String>,
    tooltip: Option<String>,
    icon_name: Option<String>,
    facing_attribute: Option<Attribute<Direction>>,
    should_snap: bool,
}

impl<T> InstanceFactory for BuiltInstanceFactory<T>
where
    T: InstanceFactory,
{
    fn get_name(&self) -> &str {
        self.inner.get_name()
    }

    fn create_attribute_set(&self) -> AttributeSet {
        self.inner.create_attribute_set()
    }
    fn get_display_name(&self) -> String {
        self.display_name
            .clone()
            .unwrap_or_else(|| self.inner.get_display_name())
    }

    fn get_default_tooltip(&self) -> Option<String> {
        self.tooltip
            .clone()
            .or_else(|| self.inner.get_default_tooltip())
    }

    fn get_icon_name(&self) -> Option<&str> {
        self.icon_name
            .as_deref()
            .or_else(|| self.inner.get_icon_name())
    }

    fn get_ports(&self) -> &[Port] {
        self.inner.get_ports()
    }

    fn get_offset_bounds(&self, attrs: &AttributeSet) -> Bounds {
        self.inner.get_offset_bounds(attrs)
    }

    fn get_facing_attribute(&self) -> Option<&Attribute<Direction>> {
        self.facing_attribute
            .as_ref()
            .or_else(|| self.inner.get_facing_attribute())
    }

    fn should_snap(&self) -> bool {
        self.should_snap
    }

    fn create_component(&self, location: Location, attrs: AttributeSet) -> Box<dyn Any> {
        self.inner.create_component(location, attrs)
    }

    fn configure_new_instance(&self, instance: &Instance) {
        self.inner.configure_new_instance(instance)
    }

    fn instance_attribute_changed(&self, instance: &Instance, attr: &dyn Any) {
        self.inner.instance_attribute_changed(instance, attr)
    }

    fn paint_instance(&self, painter: &mut InstancePainter) {
        self.inner.paint_instance(painter)
    }

    fn paint_icon(&self, painter: &mut InstancePainter) {
        self.inner.paint_icon(painter)
    }

    fn paint_ghost(&self, painter: &mut InstancePainter) {
        self.inner.paint_ghost(painter)
    }

    fn propagate(&self, state: &mut dyn InstanceState) {
        self.inner.propagate(state)
    }

    fn get_instance_feature(&self, instance: &Instance, key: &dyn Any) -> Option<Box<dyn Any>> {
        self.inner.get_instance_feature(instance, key)
    }

    fn provides_subcircuit_menu(&self) -> bool {
        self.inner.provides_subcircuit_menu()
    }
}

impl<T> std::fmt::Debug for BuiltInstanceFactory<T>
where
    T: InstanceFactory,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltInstanceFactory")
            .field("inner", &self.inner)
            .field("display_name", &self.display_name)
            .field("has_tooltip", &self.tooltip.is_some())
            .field("icon_name", &self.icon_name)
            .field("should_snap", &self.should_snap)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instance::{PortType, PortWidth};

    #[derive(Debug)]
    struct TestFactory {
        name: String,
        ports: Vec<Port>,
    }

    impl TestFactory {
        fn new() -> Self {
            Self {
                name: "Test Component".to_string(),
                ports: vec![
                    Port::new(0, -10, PortType::Input, PortWidth::fixed_bits(1)),
                    Port::new(0, 10, PortType::Output, PortWidth::fixed_bits(1)),
                ],
            }
        }
    }

    impl InstanceFactory for TestFactory {
        fn get_name(&self) -> &str {
            &self.name
        }

        fn create_attribute_set(&self) -> AttributeSet {
            AttributeSet::new()
        }
        fn get_ports(&self) -> &[Port] {
            &self.ports
        }

        fn get_offset_bounds(&self, _attrs: &AttributeSet) -> Bounds {
            Bounds::new(-10, -20, 20, 40)
        }

        fn create_component(&self, _location: Location, _attrs: AttributeSet) -> Box<dyn Any> {
            // Mock implementation
            Box::new(())
        }

        fn paint_instance(&self, _painter: &mut InstancePainter) {
            // Mock implementation
        }

        fn propagate(&self, _state: &mut dyn InstanceState) {
            // Mock implementation
        }
    }

    #[test]
    fn test_instance_factory_basic_properties() {
        let factory = TestFactory::new();

        assert_eq!(factory.get_name(), "Test Component");
        assert_eq!(factory.get_ports().len(), 2);
        assert!(factory.should_snap());
        assert!(!factory.provides_subcircuit_menu());
        assert!(factory.get_default_tooltip().is_none());
        assert!(factory.get_icon_name().is_none());
    }

    #[test]
    fn test_instance_factory_builder() {
        let factory = TestFactory::new();
        let built = InstanceFactoryBuilder::new(factory)
            .display_name_str("Custom Name")
            .tooltip_str("Custom tooltip")
            .icon_name("custom_icon")
            .should_snap(false)
            .build();

        assert_eq!(built.get_display_name().as_str(), "Custom Name");
        assert_eq!(
            built.get_default_tooltip().unwrap().as_str(),
            "Custom tooltip"
        );
        assert_eq!(built.get_icon_name(), Some("custom_icon"));
        assert!(!built.should_snap());
    }

    #[test]
    fn test_port_access() {
        let factory = TestFactory::new();
        let ports = factory.get_ports();

        assert_eq!(ports.len(), 2);
        assert_eq!(ports[0].port_type(), PortType::Input);
        assert_eq!(ports[1].port_type(), PortType::Output);
    }

    #[test]
    fn test_bounds() {
        let factory = TestFactory::new();
        let attrs = AttributeSet::new();
        let bounds = factory.get_offset_bounds(&attrs);

        assert_eq!(bounds.x(), -10);
        assert_eq!(bounds.y(), -20);
        assert_eq!(bounds.width(), 20);
        assert_eq!(bounds.height(), 40);
    }
}
