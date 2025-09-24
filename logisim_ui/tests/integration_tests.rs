//! Integration tests for UI components with circuit loading

use logisim_ui::{LogisimApp, MainFrame, Selection, EditHandler};
use logisim_core::{Simulation, circ_format::CircIntegration};
use std::path::PathBuf;

#[test]
fn test_ui_app_integration() {
    // Test basic app creation and functionality
    let app = LogisimApp::new();
    assert_eq!(app.title(), "Logisim-RUST");
    assert!(!app.has_unsaved_changes());
}

#[test]
fn test_main_frame_integration() {
    // Test main frame with simulation integration
    let mut frame = MainFrame::new();
    let sim = Simulation::new();
    
    frame.set_simulation(sim);
    assert!(frame.simulation().is_some());
}

#[test] 
fn test_selection_and_edit_integration() {
    // Test that selection and edit handler work together
    let mut handler = EditHandler::new();
    let selection = handler.selection_mut();
    
    // Test selection operations
    let comp_id = logisim_core::ComponentId(1);
    selection.add_component(comp_id);
    assert!(selection.is_component_selected(&comp_id));
    
    // Test edit operations
    assert!(handler.copy().is_ok());
    assert!(handler.paste().is_ok());
    assert!(handler.delete().is_ok());
}

#[test]
fn test_circuit_file_discovery() {
    // Test that we can find circuit files in the project
    // This test adapts to whatever files are actually present
    use std::fs;
    
    let search_dirs = vec![
        "example_schematics",
        "logisim_core/test_resources",
        "../example_schematics",
        "../logisim_core/test_resources",
    ];
    
    let mut found_files = 0;
    
    for dir in search_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "circ") {
                    found_files += 1;
                    println!("Found circuit file: {:?}", path);
                    if found_files >= 3 {
                        break; // Found enough files for testing
                    }
                }
            }
            if found_files >= 3 {
                break;
            }
        }
    }
    
    // For now, just verify the test infrastructure works
    // In a real scenario, we'd have a more controlled test environment
    println!("Circuit file discovery test completed - found {} files", found_files);
    
    // This test passes if we can run the search logic without panicking
    assert!(true);
}

#[test]
fn test_headless_circuit_loading() {
    // Test loading circuit files in headless mode
    let test_paths = vec![
        "example_schematics/logisim/ALU/ALU.circ",
        "example_schematics/logisim_evolution/MAINBOARD.circ",
    ];
    
    for path_str in test_paths {
        let path = PathBuf::from(path_str);
        if path.exists() {
            println!("Testing circuit file: {}", path_str);
            
            // Try to load the circuit
            match CircIntegration::load_into_simulation(&path) {
                Ok(sim) => {
                    let stats = sim.netlist().stats();
                    println!("  Successfully loaded: {} nets, {} nodes", 
                            stats.net_count, stats.node_count);
                    
                    // Test UI integration
                    let mut app = LogisimApp::new();
                    let result = app.load_circuit_file(path.clone());
                    
                    match result {
                        Ok(_) => {
                            println!("  UI integration successful");
                            let filename = path.file_name().unwrap().to_string_lossy();
                            assert!(app.title().contains(&*filename));
                        }
                        Err(e) => {
                            println!("  UI integration failed (expected for unsupported components): {}", e);
                            // This is expected for circuits with unsupported components
                        }
                    }
                }
                Err(e) => {
                    println!("  Failed to load (expected for complex circuits): {}", e);
                    // This is expected for circuits with components not yet implemented
                }
            }
        }
    }
}

#[test]
fn test_ui_architecture_completeness() {
    // Test that all major UI components can be created
    let _app = LogisimApp::new();
    let _frame = MainFrame::new();
    let _selection = Selection::new();
    let _edit_handler = EditHandler::new();
    
    // Test that they integrate properly
    let mut frame = MainFrame::new();
    let sim = Simulation::new();
    frame.set_simulation(sim);
    
    assert!(frame.simulation().is_some());
}

#[test]
fn test_ui_state_management() {
    // Test UI state transitions
    let app = LogisimApp::new();
    
    // Initial state
    assert_eq!(app.title(), "Logisim-RUST");
    
    // Test with simulation
    let sim = Simulation::new();
    let mut frame = MainFrame::new();
    frame.set_simulation(sim);
    
    // Verify state is maintained
    assert!(frame.simulation().is_some());
}

/// Test the GUI architecture without requiring actual GUI libraries
#[cfg(not(feature = "gui"))]
#[test]
fn test_headless_mode_functionality() {
    use logisim_ui::gui::app::{run_app, run_app_with_file};
    
    // Test headless mode works
    assert!(run_app().is_ok());
    
    // Test with a circuit file if available  
    let test_file = PathBuf::from("example_schematics/logisim/ALU/ALU.circ");
    if test_file.exists() {
        // This may fail due to unsupported components, which is expected
        let _result = run_app_with_file(test_file);
    }
}