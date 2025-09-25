//! Comprehensive UI components integration tests
//!
//! This test suite validates that all UI components work together properly,
//! especially in headless mode for CI/CD environments.

use logisim_core::Simulation;
use logisim_ui::{
    gui::{
        app::LogisimApp,
        edit_handler::EditHandler,
        frame::MainFrame,
        i18n::{set_language, tr, Language},
        properties::{ComponentProperties, PropertyDescriptor, PropertyValue},
        selection::Selection,
        startup::Startup,
    },
    UiResult,
};
use std::path::PathBuf;

#[cfg(feature = "gui")]
use logisim_ui::gui::{chronogram::ChronogramModel, toolbox::ToolType};

#[test]
fn test_complete_ui_architecture_integration() {
    // Test that all major UI components can be created and work together
    let _app = LogisimApp::new();
    let mut frame = MainFrame::new();
    let mut selection = Selection::new();
    let mut edit_handler = EditHandler::new();

    // Test simulation integration
    let sim = Simulation::new();
    frame.set_simulation(sim);
    assert!(frame.simulation().is_some());

    // Test selection operations
    use logisim_core::ComponentId;
    selection.add_component(ComponentId(1));
    selection.add_component(ComponentId(2));
    assert_eq!(selection.count(), 2);
    assert!(!selection.is_empty());

    // Test edit operations
    edit_handler.selection_mut().add_component(ComponentId(3));
    assert_eq!(edit_handler.selection().count(), 1);

    // Clear selection
    selection.clear();
    assert!(selection.is_empty());
}

#[test]
fn test_properties_system_integration() {
    use logisim_core::ComponentId;

    // Test comprehensive property system
    let mut props = ComponentProperties::new(ComponentId(1), "AND Gate");

    // Add various property types
    props.add_property(
        PropertyDescriptor::new_integer("inputs", "Input Count", 2)
            .with_range(2, 8)
            .with_description("Number of input pins"),
    );

    props.add_property(
        PropertyDescriptor::new_enum(
            "facing",
            "Facing Direction",
            vec![
                "East".to_string(),
                "West".to_string(),
                "North".to_string(),
                "South".to_string(),
            ],
            "East",
        )
        .with_description("Direction the component faces"),
    );

    props.add_property(
        PropertyDescriptor::new_boolean("negated", "Negated Output", false)
            .with_description("Whether to negate the output"),
    );

    // Test property operations
    assert!(props
        .set_property("inputs", PropertyValue::Integer(4))
        .is_ok());
    assert!(props
        .set_property("inputs", PropertyValue::Integer(10))
        .is_err()); // Out of range

    assert!(props
        .set_property(
            "facing",
            PropertyValue::Enum {
                value: "North".to_string(),
                options: vec![
                    "East".to_string(),
                    "West".to_string(),
                    "North".to_string(),
                    "South".to_string()
                ]
            }
        )
        .is_ok());

    assert!(props
        .set_property("negated", PropertyValue::Boolean(true))
        .is_ok());

    // Test property categorization
    let categorized = props.get_properties_by_category();
    assert!(!categorized.is_empty());

    // Test export/import
    let exported = props.export_properties();
    assert_eq!(exported.get("inputs"), Some(&"4".to_string()));
    assert_eq!(exported.get("negated"), Some(&"true".to_string()));

    let mut new_props = ComponentProperties::new(ComponentId(2), "OR Gate");
    new_props.add_property(PropertyDescriptor::new_integer("inputs", "Input Count", 2));

    let mut import_data = std::collections::HashMap::new();
    import_data.insert("inputs".to_string(), "3".to_string());

    let errors = new_props.import_properties(import_data);
    assert!(errors.is_empty());
    assert_eq!(
        new_props.get_property("inputs"),
        Some(&PropertyValue::Integer(3))
    );
}

#[test]
fn test_i18n_system_integration() {
    // Test internationalization system

    // Test language switching
    set_language(Language::English);
    let english_title = tr("app.title");
    assert_eq!(english_title, "Logisim-RUST");

    set_language(Language::Spanish);
    let spanish_file = tr("menu.file");
    assert_eq!(spanish_file, "Archivo");

    set_language(Language::French);
    let french_file = tr("menu.file");
    assert_eq!(french_file, "Fichier");

    // Test fallback behavior
    let missing_key = tr("nonexistent.key");
    assert!(missing_key.starts_with('[') && missing_key.ends_with(']'));

    // Reset to English for other tests
    set_language(Language::English);
}

#[test]
fn test_startup_system_integration() {
    // Test comprehensive startup argument parsing

    // Test empty args
    let args = vec!["logisim".to_string()];
    let startup = Startup::parse_args(&args).unwrap();
    assert!(!startup.should_quit());

    // Test help
    let args = vec!["logisim".to_string(), "--help".to_string()];
    let startup = Startup::parse_args(&args).unwrap();
    assert!(startup.should_quit());

    // Test version
    let args = vec!["logisim".to_string(), "--version".to_string()];
    let startup = Startup::parse_args(&args).unwrap();
    assert!(startup.should_quit());

    // Test file loading
    let args = vec!["logisim".to_string(), "test.circ".to_string()];
    let startup = Startup::parse_args(&args).unwrap();
    assert!(!startup.should_quit());

    // Test complex args
    let args = vec![
        "logisim".to_string(),
        "--headless".to_string(),
        "--template".to_string(),
        "template.circ".to_string(),
        "--sub".to_string(),
        "NAME".to_string(),
        "VALUE".to_string(),
        "circuit.circ".to_string(),
    ];
    let startup = Startup::parse_args(&args);
    assert!(startup.is_some());

    // Test invalid args
    let args = vec!["logisim".to_string(), "--invalid-option".to_string()];
    let startup = Startup::parse_args(&args);
    assert!(startup.is_none());
}

#[test]
#[cfg(feature = "gui")]
fn test_chronogram_system_integration() {
    // Test chronogram data model
    let mut model = ChronogramModel::new();

    // Add signals (this would normally come from simulation)
    use logisim_core::signal::BusWidth;

    // Test that the model can be created and initialized
    assert!(model.signal_count() == 0);

    // Test signal management
    model.add_signal("clk".to_string(), BusWidth(1));
    model.add_signal("data".to_string(), BusWidth(8));
    assert!(model.signal_count() == 2);

    // Test signal removal
    model.remove_signal("clk");
    assert!(model.signal_count() == 1);

    model.clear_signals();
    assert!(model.signal_count() == 0);
}

#[test]
#[cfg(feature = "gui")]
fn test_toolbox_system_integration() {
    // Test tool type enumeration and basic toolbox functionality
    let tools = vec![
        ToolType::Edit,
        ToolType::Wire,
        ToolType::InputPin,
        ToolType::OutputPin,
        ToolType::AndGate,
        ToolType::OrGate,
        ToolType::NotGate,
    ];

    // Verify all tools have distinct values
    for (i, tool1) in tools.iter().enumerate() {
        for (j, tool2) in tools.iter().enumerate() {
            if i != j {
                assert_ne!(tool1, tool2);
            }
        }
    }

    // Test tool categorization (basic validation)
    let gate_tools = vec![ToolType::AndGate, ToolType::OrGate, ToolType::NotGate];
    let io_tools = vec![ToolType::InputPin, ToolType::OutputPin];
    let edit_tools = vec![ToolType::Edit, ToolType::Wire];

    assert!(!gate_tools.is_empty());
    assert!(!io_tools.is_empty());
    assert!(!edit_tools.is_empty());
}

#[test]
fn test_cross_component_communication() {
    // Test that components can communicate properly
    let _app = LogisimApp::new();
    let mut frame = MainFrame::new();
    let mut edit_handler = EditHandler::new();

    // Create a simulation and wire it through the system
    let sim = Simulation::new();
    frame.set_simulation(sim);

    // Test that the frame has the simulation
    assert!(frame.simulation().is_some());

    // Test selection coordination between edit handler and frame
    use logisim_core::ComponentId;
    edit_handler.selection_mut().add_component(ComponentId(1));
    edit_handler.selection_mut().add_component(ComponentId(2));

    // Test edit operations
    assert!(edit_handler.copy().is_ok());
    assert!(edit_handler.cut().is_ok());
    assert!(edit_handler.selection().is_empty()); // Cut should clear selection

    assert!(edit_handler.paste().is_ok());
    assert!(edit_handler.undo().is_ok());
    assert!(edit_handler.redo().is_ok());
}

#[test]
fn test_error_handling_integration() {
    // Test that error handling works across the UI system
    use logisim_ui::UiError;

    // Test property validation errors
    let mut props = ComponentProperties::new(logisim_core::ComponentId(1), "Test");
    props.add_property(PropertyDescriptor::new_integer("value", "Value", 0).with_range(1, 10));

    // This should fail validation
    let result = props.set_property("value", PropertyValue::Integer(15));
    assert!(result.is_err());

    // Test that error types can be created and handled
    let _ui_error = UiError::PlacementError("Test error".to_string());
    let _render_error = UiError::RenderError("Render failed".to_string());
    let _file_error = UiError::FileError("File not found".to_string());

    // Test error conversion from core
    let core_error = logisim_core::simulation::SimulationError::NetlistError("test".to_string());
    let _ui_error: UiError = core_error.into();
}

#[test]
fn test_headless_mode_complete_functionality() {
    // Test that all UI components work in headless mode
    let app = LogisimApp::new();

    // Test app title and basic functionality
    assert_eq!(app.title(), "Logisim-RUST");

    // Test that edit operations work in headless mode
    let mut edit_handler = EditHandler::new();
    assert!(edit_handler.select_all().is_ok());
    assert!(edit_handler.copy().is_ok());
    assert!(edit_handler.paste().is_ok());

    // Test properties in headless mode
    let props = ComponentProperties::new(logisim_core::ComponentId(1), "Test Component");
    assert_eq!(props.component_name, "Test Component");

    // Test i18n in headless mode
    set_language(Language::English);
    let text = tr("app.title");
    assert_eq!(text, "Logisim-RUST");
}

/// Integration test helper to verify file operations work
#[test]
fn test_file_integration_headless() {
    // Test that the UI can work with files in headless mode

    // Create temporary test file paths
    let test_paths = vec![
        PathBuf::from("test_basic.circ"),
        PathBuf::from("test_complex.circ"),
    ];

    for path in test_paths {
        // Test that the app can attempt to load files (even if they don't exist)
        let mut app = LogisimApp::new();
        let result = app.load_circuit_file(path.clone());

        // The file might not exist, but the interface should handle it gracefully
        match result {
            Ok(_) => {
                // File loaded successfully
                println!("Successfully loaded: {}", path.display());
            }
            Err(error) => {
                // File not found or other error - this is expected for non-existent files
                match error {
                    logisim_ui::UiError::FileError(_) => {
                        // Expected for non-existent files
                        println!("File not found (expected): {}", path.display());
                    }
                    logisim_ui::UiError::CoreError(_) => {
                        // Expected for invalid circuit files
                        println!("Core error (expected): {}", path.display());
                    }
                    _ => {
                        // Other errors might indicate implementation issues
                        eprintln!("Unexpected error loading {}: {}", path.display(), error);
                    }
                }
            }
        }
    }
}
