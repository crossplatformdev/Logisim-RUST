/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Layout manager contract

/// Represents dimensions (width, height)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}

impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

/// Represents a UI component for layout purposes
#[derive(Debug, Clone)]
pub struct Component {
    pub id: u32,
    pub name: String,
    pub bounds: (i32, i32, u32, u32), // x, y, width, height
}

/// Represents a container of components
#[derive(Debug, Clone)]
pub struct Container {
    pub id: u32,
    pub components: Vec<Component>,
    pub bounds: (i32, i32, u32, u32), // x, y, width, height
}

/// Base contract for layout managers
pub trait BaseLayoutManagerContract {
    /// Add a component to the layout with a constraint string
    fn add_layout_component(&mut self, _constraint: &str, _component: &Component) {
        // no-op implementation
    }

    /// Remove a component from the layout
    fn remove_layout_component(&mut self, _component: &Component) {
        // no-op implementation
    }

    /// Calculate the preferred size for the container
    fn preferred_layout_size(&self, container: &Container) -> Dimension;

    /// Calculate the minimum size for the container
    fn minimum_layout_size(&self, container: &Container) -> Dimension;

    /// Layout the container's components
    fn layout_container(&mut self, _container: &mut Container) {
        // no-op implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLayoutManager;

    impl BaseLayoutManagerContract for TestLayoutManager {
        fn preferred_layout_size(&self, _container: &Container) -> Dimension {
            Dimension::new(100, 100)
        }

        fn minimum_layout_size(&self, _container: &Container) -> Dimension {
            Dimension::new(50, 50)
        }
    }

    #[test]
    fn test_layout_manager() {
        let mut layout_manager = TestLayoutManager;
        let component = Component {
            id: 1,
            name: "test".to_string(),
            bounds: (0, 0, 10, 10),
        };
        let container = Container {
            id: 1,
            components: vec![component.clone()],
            bounds: (0, 0, 100, 100),
        };

        // Test default implementations
        layout_manager.add_layout_component("center", &component);
        layout_manager.remove_layout_component(&component);

        // Test required methods
        let preferred = layout_manager.preferred_layout_size(&container);
        assert_eq!(preferred, Dimension::new(100, 100));

        let minimum = layout_manager.minimum_layout_size(&container);
        assert_eq!(minimum, Dimension::new(50, 50));
    }
}