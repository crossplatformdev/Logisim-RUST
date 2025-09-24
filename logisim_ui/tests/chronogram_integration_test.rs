//! Integration test for chronogram with real circuit simulation

use logisim_core::{circ_format::CircIntegration, Simulation};
use std::path::PathBuf;

#[cfg(feature = "gui")]
use logisim_ui::gui::chronogram::ChronogramPanel;

#[test]
fn test_chronogram_basic_functionality() {
    // Create a basic simulation with some nodes for testing
    let mut simulation = Simulation::new();
    
    // Add a simple node to the netlist for testing
    use logisim_core::signal::BusWidth;
    let _node_id = simulation.netlist_mut().create_named_node(BusWidth(1), "test_clk".to_string());
    
    // In headless mode, just verify simulation works
    let stats = simulation.stats();
    println!("Simulation stats: events_processed={}, current_time={}", 
             stats.events_processed, stats.current_time.as_u64());
    
    // Try to step the simulation
    let step_result = simulation.step();
    println!("Simulation step result: {:?}", step_result);
    
    // Check we can get node IDs
    let node_ids = simulation.get_all_node_ids();
    println!("Found {} nodes in simulation", node_ids.len());
    
    // Verify basic functionality
    assert!(node_ids.len() > 0, "Should have at least one node");
}

#[test]
fn test_chronogram_with_real_circuit() {
    // Try to find a test circuit file
    let test_files = [
        "example_schematics/logisim/small_counter.circ",
        "example_schematics/logisim_evolution/Clock_C.circ",
        "logisim_core/test_resources/simple_and_gate.circ",
    ];
    
    let mut test_file_path: Option<PathBuf> = None;
    
    for file in &test_files {
        let path = PathBuf::from(file);
        if path.exists() {
            test_file_path = Some(path);
            break;
        }
    }
    
    // If no test file exists, create a minimal simulation for testing
    let mut simulation = if let Some(path) = test_file_path {
        println!("Loading circuit from: {}", path.display());
        match CircIntegration::load_into_simulation(&path) {
            Ok(sim) => {
                println!("Successfully loaded circuit");
                sim
            },
            Err(e) => {
                println!("Failed to load circuit: {}, using empty simulation", e);
                Simulation::new()
            }
        }
    } else {
        println!("No test circuit files found, using empty simulation");
        Simulation::new()
    };
    
    #[cfg(feature = "gui")]
    {
        // Test chronogram integration
        let mut chronogram = ChronogramPanel::new();
        
        // Start recording
        chronogram.start_recording(&simulation);
        assert!(chronogram.is_recording());
        
        // Run a few simulation steps to generate data
        for i in 0..10 {
            // Update chronogram with current simulation state
            chronogram.update_from_simulation(&simulation);
            
            // Step simulation if possible
            if let Ok(has_more) = simulation.step() {
                if !has_more {
                    break;
                }
                println!("Simulation step {}: time={}", i, simulation.current_time().as_u64());
            }
        }
        
        // Check that we have some signal data
        println!("Chronogram has {} signals", chronogram.model().signal_count());
        
        // Test export functionality
        let export_text = chronogram.export_to_text();
        assert!(export_text.contains("Logisim-RUST Chronogram Export"));
        assert!(export_text.contains("Time Range:"));
        
        println!("Chronogram export sample:\n{}", export_text);
        
        // Stop recording
        chronogram.stop_recording();
        assert!(!chronogram.is_recording());
    }
    
    #[cfg(not(feature = "gui"))]
    {
        // In headless mode, just verify simulation works
        let stats = simulation.stats();
        println!("Simulation stats: {:?}", stats);
        
        // Try to step the simulation
        let _ = simulation.step();
        
        // Check we can get node IDs
        let node_ids = simulation.get_all_node_ids();
        println!("Found {} nodes in simulation", node_ids.len());
    }
}

#[cfg(feature = "gui")]
#[test]
fn test_chronogram_signal_types() {
    use logisim_core::{
        netlist::NodeId,
        signal::{BusWidth, Signal, Timestamp, Value},
    };
    use logisim_ui::gui::chronogram::{ChronogramModel, SignalInfo};
    
    let mut model = ChronogramModel::new();
    
    // Test single-bit signal
    let clk_info = SignalInfo::new(NodeId(1), "clk".to_string(), BusWidth(1), 0);
    model.add_signal(clk_info);
    
    // Test multi-bit signal (bus)  
    let data_info = SignalInfo::new(NodeId(2), "data_bus".to_string(), BusWidth(8), 1);
    model.add_signal(data_info);
    
    // Record some signal changes
    let clk_high = Signal::new_single(Value::High);
    let clk_low = Signal::new_single(Value::Low);
    let data_val = Signal::from_u64(0xAB, BusWidth(8));
    
    model.record_signal_change(NodeId(1), Timestamp(0), clk_low.clone());
    model.record_signal_change(NodeId(1), Timestamp(10), clk_high.clone());
    model.record_signal_change(NodeId(1), Timestamp(20), clk_low.clone());
    
    model.record_signal_change(NodeId(2), Timestamp(5), data_val);
    
    // Verify data was recorded
    assert_eq!(model.signal_count(), 2);
    assert!(model.has_data());
    assert_eq!(model.start_time(), Timestamp(0));
    assert_eq!(model.end_time(), Timestamp(20));
    
    // Test signal data retrieval
    let clk_data = model.get_signal_data(NodeId(1)).unwrap();
    assert_eq!(clk_data.get_value_at(Timestamp(0)), Some(&clk_low));
    assert_eq!(clk_data.get_value_at(Timestamp(10)), Some(&clk_high));
    assert_eq!(clk_data.get_value_at(Timestamp(15)), Some(&clk_high)); // Should get last value
    
    println!("Successfully tested chronogram with {} signals", model.signal_count());
}