//! Tests for UI components

#[cfg(test)]
#[allow(clippy::module_inception)] // Tests module is conventional
mod tests {
    use super::super::{
        app::LogisimApp, edit_handler::EditHandler, frame::MainFrame, selection::Selection,
    };
    use logisim_core::{ComponentId, NetId, Simulation};
    use std::path::PathBuf;

    #[test]
    fn test_app_creation() {
        let app = LogisimApp::new();
        assert_eq!(app.title(), "Logisim-RUST");
        assert!(!app.has_unsaved_changes());
    }

    #[test]
    fn test_main_frame_creation() {
        let frame = MainFrame::new();
        assert!(frame.simulation().is_none());
    }

    #[test]
    fn test_selection_operations() {
        let mut selection = Selection::new();
        assert!(selection.is_empty());

        let comp_id = ComponentId(1);
        let net_id = NetId(1);

        selection.add_component(comp_id);
        selection.add_net(net_id);

        assert!(!selection.is_empty());
        assert_eq!(selection.count(), 2);
        assert!(selection.is_component_selected(&comp_id));
        assert!(selection.is_net_selected(&net_id));

        selection.clear();
        assert!(selection.is_empty());
    }

    #[test]
    fn test_edit_handler_creation() {
        let handler = EditHandler::new();
        assert!(handler.selection().is_empty());
    }

    #[test]
    fn test_edit_handler_operations() {
        let mut handler = EditHandler::new();

        // Test basic operations don't panic
        assert!(handler.copy().is_ok());
        assert!(handler.paste().is_ok());
        assert!(handler.delete().is_ok());
        assert!(handler.undo().is_ok());
        assert!(handler.redo().is_ok());
        assert!(handler.select_all().is_ok());
    }

    #[test]
    fn test_app_with_simulation() {
        let _app = LogisimApp::new();
        let mut sim = Simulation::new();

        // Add a simple component to test
        let _net_id = sim
            .netlist_mut()
            .create_named_node(logisim_core::BusWidth(1), "test_node".to_string());

        let mut frame = MainFrame::new();
        frame.set_simulation(sim);

        assert!(frame.simulation().is_some());
    }

    #[cfg(not(feature = "gui"))]
    #[test]
    fn test_headless_mode() {
        use super::super::app::{run_app, run_app_with_file};

        // Test that headless mode runs without error
        assert!(run_app().is_ok());

        // Test with a circuit file (using the test resource)
        let test_file = PathBuf::from("logisim_core/test_resources/MAINBOARD.circ");
        if test_file.exists() {
            let result = run_app_with_file(test_file);
            // For now, just check it doesn't panic - detailed validation comes later
            match result {
                Ok(_) => (),
                Err(e) => println!("Expected error in headless mode: {}", e),
            }
        }
    }

    #[test]
    fn test_circuit_file_loading() {
        let mut app = LogisimApp::new();
        let test_file = PathBuf::from("logisim_core/test_resources/MAINBOARD.circ");

        if test_file.exists() {
            let result = app.load_circuit_file(test_file);
            // For now, just check it doesn't panic - detailed validation comes later
            match result {
                Ok(_) => assert!(app.title().contains("MAINBOARD.circ")),
                Err(e) => println!("Expected error loading file: {}", e),
            }
        }
    }
}
