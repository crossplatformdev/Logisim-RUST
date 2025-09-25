//! Demonstration of the new extensibility features and plugin system
//!
//! This example shows how to use the advanced modeling features, observer pattern,
//! and dynamic component registration capabilities of Logisim-RUST.

use logisim_core::integrations::{
    PluginManager, ComponentRegistry, ComponentCategory,
    register_example_plugin,
};
use logisim_core::event_system::{EventSystem, event_utils};
use logisim_core::{ComponentId, Location, Signal, Value};

fn main() {
    // Initialize logging
    env_logger::init();
    
    println!("🚀 Logisim-RUST Extensibility Demo");
    println!("===================================");
    
    // Demo 1: Event System
    demo_event_system();
    
    // Demo 2: Plugin Manager  
    demo_plugin_manager();
    
    // Demo 3: Dynamic Component Registration
    demo_component_registry();
    
    println!("\n✅ All extensibility features demonstrated successfully!");
    println!("⚠️  Note: These APIs are UNSTABLE and subject to change in future versions.");
}

fn demo_event_system() {
    println!("\n📡 Event System Demo");
    println!("-------------------");
    
    let mut event_system = EventSystem::new();
    println!("✓ Created event system");
    
    // Create some example events
    let component_id = ComponentId::with_value(42);
    let location = Location::new(100, 200);
    
    let _event1 = event_utils::component_added(component_id, location);
    let _event2 = event_utils::signal_changed(component_id, Signal::new_single(Value::High));
    
    // Emit events (would normally have observers registered)
    println!("✓ Created sample circuit and simulation events");
    println!("  - Component added at ({}, {})", location.get_x(), location.get_y());
    println!("  - Signal changed to HIGH");
    
    // Show event system stats
    let stats = event_system.stats();
    println!("✓ Event System Stats:");
    println!("  - Circuit observers: {}", stats.circuit_observers);
    println!("  - Simulation observers: {}", stats.simulation_observers);
    println!("  - Circuit queue length: {}", stats.circuit_queue_length);
    println!("  - Simulation queue length: {}", stats.simulation_queue_length);
}

fn demo_plugin_manager() {
    println!("\n🔌 Plugin Manager Demo");
    println!("---------------------");
    
    let mut plugin_manager = PluginManager::new();
    println!("✓ Created plugin manager");
    
    // Add search paths
    plugin_manager.add_search_path(std::path::PathBuf::from("./plugins"));
    plugin_manager.add_search_path(std::path::PathBuf::from("/usr/local/lib/logisim/plugins"));
    println!("✓ Added plugin search paths");
    
    // Register example plugin components and extensions
    if let Err(e) = register_example_plugin(&mut plugin_manager) {
        println!("⚠️  Plugin registration failed: {}", e);
    } else {
        println!("✓ Registered example plugin extensions");
    }
    
    // Show plugin manager stats
    let stats = plugin_manager.stats();
    println!("✓ Plugin Manager Stats:");
    println!("  - Loaded plugins: {}", stats.loaded_plugins);
    println!("  - Search paths: {}", stats.search_paths);
    println!("  - Registered components: {}", stats.registered_components);
    println!("  - Extension hooks: {}", stats.extension_hooks);
    println!("  - Circuit observers: {}", stats.circuit_observers);
    println!("  - Simulation observers: {}", stats.simulation_observers);
    
    // List all available components
    let components = plugin_manager.get_all_components();
    println!("✓ Available components: {}", components.len());
    for (source, info) in &components {
        println!("  - {} from {}: {}", info.name, source, info.description);
    }
}

fn demo_component_registry() {
    println!("\n⚙️  Component Registry Demo");
    println!("---------------------------");
    
    let registry = ComponentRegistry::new();
    println!("✓ Created component registry");
    
    // Component types would normally be registered by plugins
    let component_types = registry.component_types();
    println!("✓ Registered component types: {}", component_types.len());
    
    // Show available categories
    let categories = [
        ComponentCategory::Gates,
        ComponentCategory::Memory,
        ComponentCategory::IO,
        ComponentCategory::Arithmetic,
        ComponentCategory::Plexers,
        ComponentCategory::Wiring,
        ComponentCategory::Custom("Example".to_string()),
    ];
    
    println!("✓ Available component categories:");
    for category in &categories {
        let components_in_category = registry.components_in_category(category);
        println!("  - {}: {} components", 
                category.display_name(), 
                components_in_category.len());
    }
}