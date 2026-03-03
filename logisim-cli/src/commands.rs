//! CLI command implementations.

use logisim_core::{
    component::ComponentKind,
    project::Project,
    simulation::Simulator,
    value::{BitWidth, Bus},
};
use logisim_file::parse_circ;
use std::fs::File;
use std::io::BufReader;

// ── Shared option parsing ─────────────────────────────────────────────────────

struct Opts {
    file: Option<String>,
    circuit: Option<String>,
    steps: usize,
    terse: bool,
}

fn parse_opts(args: &[String]) -> Result<Opts, String> {
    let mut opts = Opts {
        file: None,
        circuit: None,
        steps: 10,
        terse: false,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--circuit" => {
                i += 1;
                opts.circuit = Some(
                    args.get(i)
                        .ok_or("--circuit requires an argument")?
                        .clone(),
                );
            }
            "--steps" => {
                i += 1;
                opts.steps = args
                    .get(i)
                    .ok_or("--steps requires an argument")?
                    .parse::<usize>()
                    .map_err(|_| "--steps must be a positive integer".to_string())?;
            }
            "--terse" => {
                opts.terse = true;
            }
            arg if !arg.starts_with('-') => {
                opts.file = Some(arg.to_string());
            }
            arg => {
                return Err(format!("Unknown option: {}", arg));
            }
        }
        i += 1;
    }

    Ok(opts)
}

fn load_project(path: &str) -> Result<Project, String> {
    let f = File::open(path).map_err(|e| format!("Cannot open {}: {}", path, e))?;
    let reader = BufReader::new(f);
    parse_circ(reader).map_err(|e| format!("Parse error: {}", e))
}

// ── simulate ──────────────────────────────────────────────────────────────────

/// Run `simulate` command.
pub fn run_simulate(raw_args: &[String]) -> Result<(), String> {
    let opts = parse_opts(raw_args)?;
    let path = opts.file.as_deref().ok_or("No input file specified")?;
    let project = load_project(path)?;

    let circuit_name = opts
        .circuit
        .clone()
        .or_else(|| project.main_circuit_name().map(|s| s.to_string()))
        .ok_or("No circuits in project")?;

    if !project.circuits.contains_key(&circuit_name) {
        return Err(format!("Circuit '{}' not found", circuit_name));
    }

    let mut sim = Simulator::new(project.clone());

    if !opts.terse {
        println!("Simulating circuit '{}' for {} steps", circuit_name, opts.steps);
        println!("{}", "-".repeat(50));
    }

    // Print header.
    let circuit = &project.circuits[&circuit_name];
    let mut input_pins: Vec<_> = circuit.input_pins().iter().map(|c| c.id).collect();
    let mut output_pins: Vec<_> = circuit.output_pins().iter().map(|c| c.id).collect();
    input_pins.sort();
    output_pins.sort();

    if !opts.terse {
        let in_labels: Vec<String> = input_pins
            .iter()
            .map(|&id| {
                circuit
                    .get_component(id)
                    .map(|c| if c.label.is_empty() { format!("{}", id) } else { c.label.clone() })
                    .unwrap_or_else(|| format!("{}", id))
            })
            .collect();
        let out_labels: Vec<String> = output_pins
            .iter()
            .map(|&id| {
                circuit
                    .get_component(id)
                    .map(|c| if c.label.is_empty() { format!("{}", id) } else { c.label.clone() })
                    .unwrap_or_else(|| format!("{}", id))
            })
            .collect();
        println!("Inputs:  {}", in_labels.join(", "));
        println!("Outputs: {}", out_labels.join(", "));
        println!("{}", "-".repeat(50));
        println!("Step | {} | {}", in_labels.join(" | "), out_labels.join(" | "));
        println!("{}", "-".repeat(50));
    }

    // Run simulation steps.
    for step in 0..opts.steps {
        sim.tick(&circuit_name)
            .map_err(|e| format!("Simulation error at step {}: {}", step, e))?;

        if !opts.terse {
            let in_vals: Vec<String> = input_pins
                .iter()
                .map(|&id| {
                    sim.read_pin(&circuit_name, id)
                        .map(|b| b.to_hex_string())
                        .unwrap_or_else(|| "?".to_string())
                })
                .collect();
            let out_vals: Vec<String> = output_pins
                .iter()
                .map(|&id| {
                    sim.read_pin(&circuit_name, id)
                        .map(|b| b.to_hex_string())
                        .unwrap_or_else(|| "?".to_string())
                })
                .collect();
            println!(
                "{:4} | {} | {}",
                step,
                in_vals.join(" | "),
                out_vals.join(" | ")
            );
        } else {
            let out_vals: Vec<String> = output_pins
                .iter()
                .map(|&id| {
                    sim.read_pin(&circuit_name, id)
                        .map(|b| b.to_hex_string())
                        .unwrap_or_else(|| "?".to_string())
                })
                .collect();
            println!("{}", out_vals.join(" "));
        }
    }

    Ok(())
}

// ── truth-table ───────────────────────────────────────────────────────────────

/// Run `truth-table` command for combinational circuits.
pub fn run_truth_table(raw_args: &[String]) -> Result<(), String> {
    let opts = parse_opts(raw_args)?;
    let path = opts.file.as_deref().ok_or("No input file specified")?;
    let project = load_project(path)?;

    let circuit_name = opts
        .circuit
        .clone()
        .or_else(|| project.main_circuit_name().map(|s| s.to_string()))
        .ok_or("No circuits in project")?;

    if !project.circuits.contains_key(&circuit_name) {
        return Err(format!("Circuit '{}' not found", circuit_name));
    }

    let circuit = &project.circuits[&circuit_name];

    // Gather input/output pins.
    let mut input_pins: Vec<_> = circuit.input_pins().iter().map(|c| (c.id, {
        if let ComponentKind::Pin { width, .. } = c.kind {
            width
        } else {
            BitWidth::ONE
        }
    })).collect();
    let mut output_pins: Vec<_> = circuit.output_pins().iter().map(|c| c.id).collect();
    input_pins.sort_by_key(|(id, _)| *id);
    output_pins.sort();

    if input_pins.is_empty() {
        return Err("Circuit has no input pins".to_string());
    }

    let total_in_bits: u32 = input_pins.iter().map(|(_, w)| w.get()).sum();
    if total_in_bits > 20 {
        return Err(format!(
            "Too many input bits ({}) for truth table (max 20)",
            total_in_bits
        ));
    }

    let num_rows = 1u64 << total_in_bits;

    // Header
    let in_labels: Vec<String> = input_pins
        .iter()
        .map(|(id, _width)| {
            let label = circuit
                .get_component(*id)
                .map(|c| c.label.clone())
                .unwrap_or_default();
            if label.is_empty() {
                format!("in{}", id.0)
            } else {
                label
            }
        })
        .collect();
    let out_labels: Vec<String> = output_pins
        .iter()
        .map(|id| {
            let label = circuit
                .get_component(*id)
                .map(|c| c.label.clone())
                .unwrap_or_default();
            if label.is_empty() {
                format!("out{}", id.0)
            } else {
                label
            }
        })
        .collect();

    let header: Vec<&str> = in_labels
        .iter()
        .map(|s| s.as_str())
        .chain(out_labels.iter().map(|s| s.as_str()))
        .collect();
    println!("{}", header.join(" | "));
    println!("{}", "-".repeat(header.join(" | ").len() + 4));

    // Enumerate all input combinations.
    for row in 0..num_rows {
        let mut sim = Simulator::new(project.clone());

        // Set input values.
        let mut bit_offset = 0u32;
        for (id, width) in &input_pins {
            let w = width.get();
            let mask = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };
            let val = (row >> bit_offset) & mask;
            sim.set_pin_value(&circuit_name, *id, Bus::from_u64(val, w as usize));
            bit_offset += w;
        }

        // Propagate.
        sim.propagate(&circuit_name)
            .map_err(|e| format!("Simulation error: {}", e))?;

        // Read and print.
        let mut print_offset = 0u32;
        let in_vals: Vec<String> = input_pins
            .iter()
            .map(|(_id, width)| {
                let w = width.get();
                let mask = if w == 64 { u64::MAX } else { (1u64 << w) - 1 };
                let val = (row >> print_offset) & mask;
                print_offset += w;
                format!("{}", val)
            })
            .collect();
        let out_vals: Vec<String> = output_pins
            .iter()
            .map(|id| {
                sim.read_pin(&circuit_name, *id)
                    .map(|b| {
                        if let Some(v) = b.to_u64() {
                            v.to_string()
                        } else {
                            "?".to_string()
                        }
                    })
                    .unwrap_or_else(|| "?".to_string())
            })
            .collect();

        let row_vals: Vec<&str> = in_vals
            .iter()
            .map(|s| s.as_str())
            .chain(out_vals.iter().map(|s| s.as_str()))
            .collect();
        println!("{}", row_vals.join(" | "));
    }

    Ok(())
}

// ── info ──────────────────────────────────────────────────────────────────────

/// Run `info` command.
pub fn run_info(raw_args: &[String]) -> Result<(), String> {
    let opts = parse_opts(raw_args)?;
    let path = opts.file.as_deref().ok_or("No input file specified")?;
    let project = load_project(path)?;

    println!("Project: {}", project.name);
    println!("Circuits ({}):", project.circuits.len());

    for circuit in project.ordered_circuits() {
        println!("  Circuit: {}", circuit.name);
        println!("    Components: {}", circuit.components.len());
        println!("    Wires:      {}", circuit.wires.len());
        println!("    Input pins: {}", circuit.input_pins().len());
        println!("    Output pins: {}", circuit.output_pins().len());

        // Count by library
        let mut gates = 0usize;
        let mut memory = 0usize;
        let mut arithmetic = 0usize;
        let mut plexers = 0usize;
        let mut io = 0usize;
        let mut wiring = 0usize;
        let mut subcircuits = 0usize;
        for comp in circuit.components.values() {
            match comp.kind.library_name() {
                "gates" => gates += 1,
                "memory" => memory += 1,
                "arithmetic" => arithmetic += 1,
                "plexers" => plexers += 1,
                "io" => io += 1,
                "wiring" => wiring += 1,
                "user" => subcircuits += 1,
                _ => {}
            }
        }
        if wiring > 0 { println!("      Wiring:     {}", wiring); }
        if gates > 0 { println!("      Gates:      {}", gates); }
        if plexers > 0 { println!("      Plexers:    {}", plexers); }
        if arithmetic > 0 { println!("      Arithmetic: {}", arithmetic); }
        if memory > 0 { println!("      Memory:     {}", memory); }
        if io > 0 { println!("      I/O:        {}", io); }
        if subcircuits > 0 { println!("      Subcircuits: {}", subcircuits); }
    }

    if !project.options.is_empty() {
        println!("\nOptions:");
        for (k, v) in &project.options {
            println!("  {} = {}", k, v);
        }
    }

    Ok(())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write_temp_circ(content: &str) -> String {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "test_circuit_{}.circ",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos()
        ));
        std::fs::write(&path, content).unwrap();
        path.to_string_lossy().into_owned()
    }

    const AND_CIRC: &str = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<project version="1.0">
  <lib desc="#Wiring" name="0"/>
  <lib desc="#Gates" name="1"/>
  <circuit name="main">
    <comp lib="0" loc="(10,10)" name="Pin">
      <a name="label" val="A"/>
    </comp>
    <comp lib="0" loc="(10,20)" name="Pin">
      <a name="label" val="B"/>
    </comp>
    <comp lib="1" loc="(30,10)" name="AND Gate">
      <a name="inputs" val="2"/>
    </comp>
    <comp lib="0" loc="(50,10)" name="Pin">
      <a name="output" val="true"/>
      <a name="label" val="OUT"/>
    </comp>
    <wire from="(10,10)" to="(30,10)"/>
    <wire from="(10,20)" to="(30,11)"/>
    <wire from="(30,12)" to="(50,10)"/>
  </circuit>
</project>"##;

    #[test]
    fn test_run_info() {
        let path = write_temp_circ(AND_CIRC);
        let args: Vec<String> = vec![path];
        assert!(run_info(&args).is_ok());
    }

    #[test]
    fn test_run_simulate() {
        let path = write_temp_circ(AND_CIRC);
        let args: Vec<String> = vec!["--steps".to_string(), "3".to_string(), path];
        assert!(run_simulate(&args).is_ok());
    }

    #[test]
    fn test_run_truth_table() {
        let path = write_temp_circ(AND_CIRC);
        let args: Vec<String> = vec![path];
        assert!(run_truth_table(&args).is_ok());
    }

    #[test]
    fn test_parse_opts_defaults() {
        let args: Vec<String> = vec!["myfile.circ".to_string()];
        let opts = parse_opts(&args).unwrap();
        assert_eq!(opts.file, Some("myfile.circ".to_string()));
        assert_eq!(opts.steps, 10);
        assert!(!opts.terse);
    }

    #[test]
    fn test_parse_opts_full() {
        let args: Vec<String> = vec![
            "--circuit".to_string(),
            "sub".to_string(),
            "--steps".to_string(),
            "5".to_string(),
            "--terse".to_string(),
            "file.circ".to_string(),
        ];
        let opts = parse_opts(&args).unwrap();
        assert_eq!(opts.circuit, Some("sub".to_string()));
        assert_eq!(opts.steps, 5);
        assert!(opts.terse);
        assert_eq!(opts.file, Some("file.circ".to_string()));
    }
}
