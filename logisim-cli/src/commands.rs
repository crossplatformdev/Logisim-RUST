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
    format_json: bool,
}

fn parse_opts(args: &[String]) -> Result<Opts, String> {
    let mut opts = Opts {
        file: None,
        circuit: None,
        steps: 10,
        terse: false,
        format_json: false,
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--circuit" => {
                i += 1;
                opts.circuit = Some(args.get(i).ok_or("--circuit requires an argument")?.clone());
            }
            "--steps" => {
                i += 1;
                let steps = args
                    .get(i)
                    .ok_or("--steps requires an argument")?
                    .parse::<usize>()
                    .map_err(|_| "--steps must be a positive integer".to_string())?;
                if steps == 0 {
                    return Err("--steps must be a positive integer".to_string());
                }
                opts.steps = steps;
            }
            "--terse" => {
                opts.terse = true;
            }
            "--format" => {
                i += 1;
                let fmt = args.get(i).ok_or("--format requires an argument")?;
                match fmt.as_str() {
                    "json" => opts.format_json = true,
                    "text" => opts.format_json = false,
                    other => {
                        return Err(format!("Unknown format '{}'; use 'text' or 'json'", other))
                    }
                }
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

    // Build pin label maps.
    let circuit = &project.circuits[&circuit_name];
    let mut input_pins: Vec<_> = circuit.input_pins().iter().map(|c| c.id).collect();
    let mut output_pins: Vec<_> = circuit.output_pins().iter().map(|c| c.id).collect();
    input_pins.sort();
    output_pins.sort();

    let get_label = |id: logisim_core::component::ComponentId| {
        circuit
            .get_component(id)
            .map(|c| {
                if c.label.is_empty() {
                    format!("{}", id)
                } else {
                    c.label.clone()
                }
            })
            .unwrap_or_else(|| format!("{}", id))
    };
    let in_labels: Vec<String> = input_pins.iter().map(|&id| get_label(id)).collect();
    let out_labels: Vec<String> = output_pins.iter().map(|&id| get_label(id)).collect();

    if !opts.terse && !opts.format_json {
        println!(
            "Simulating circuit '{}' for {} steps",
            circuit_name, opts.steps
        );
        println!("{}", "-".repeat(50));
        println!("Inputs:  {}", in_labels.join(", "));
        println!("Outputs: {}", out_labels.join(", "));
        println!("{}", "-".repeat(50));
        println!(
            "Step | {} | {}",
            in_labels.join(" | "),
            out_labels.join(" | ")
        );
        println!("{}", "-".repeat(50));
    }

    // Run simulation steps.
    let mut json_steps: Vec<serde_json::Value> = Vec::new();
    for step in 0..opts.steps {
        sim.tick(&circuit_name)
            .map_err(|e| format!("Simulation error at step {}: {}", step, e))?;

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

        if opts.format_json {
            let mut inputs = serde_json::Map::new();
            for (k, v) in in_labels.iter().zip(in_vals.iter()) {
                inputs.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
            let mut outputs = serde_json::Map::new();
            for (k, v) in out_labels.iter().zip(out_vals.iter()) {
                outputs.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
            let mut obj = serde_json::Map::new();
            obj.insert("step".to_string(), serde_json::Value::Number(step.into()));
            obj.insert("inputs".to_string(), serde_json::Value::Object(inputs));
            obj.insert("outputs".to_string(), serde_json::Value::Object(outputs));
            json_steps.push(serde_json::Value::Object(obj));
        } else if !opts.terse {
            println!(
                "{:4} | {} | {}",
                step,
                in_vals.join(" | "),
                out_vals.join(" | ")
            );
        } else {
            println!("{}", out_vals.join(" "));
        }
    }

    if opts.format_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::Value::Array(json_steps))
                .unwrap_or_else(|_| "[]".to_string())
        );
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
    let mut input_pins: Vec<_> = circuit
        .input_pins()
        .iter()
        .map(|c| {
            (c.id, {
                if let ComponentKind::Pin { width, .. } = c.kind {
                    width
                } else {
                    BitWidth::ONE
                }
            })
        })
        .collect();
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

    if !opts.format_json {
        println!("{}", header.join(" | "));
        println!("{}", "-".repeat(header.join(" | ").len() + 4));
    }

    // Enumerate all input combinations.
    let mut json_rows: Vec<serde_json::Value> = Vec::new();
    let mut sim = Simulator::new(project.clone());
    for row in 0..num_rows {
        sim.reset_circuit_state(&circuit_name);

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
                            "null".to_string()
                        }
                    })
                    .unwrap_or_else(|| "null".to_string())
            })
            .collect();

        if opts.format_json {
            let mut inputs = serde_json::Map::new();
            for (k, v) in in_labels.iter().zip(in_vals.iter()) {
                let num: serde_json::Value = v
                    .parse::<u64>()
                    .map(|n| serde_json::Value::Number(n.into()))
                    .unwrap_or(serde_json::Value::Null);
                inputs.insert(k.clone(), num);
            }
            let mut outputs = serde_json::Map::new();
            for (k, v) in out_labels.iter().zip(out_vals.iter()) {
                let num: serde_json::Value = if v == "null" {
                    serde_json::Value::Null
                } else {
                    v.parse::<u64>()
                        .map(|n| serde_json::Value::Number(n.into()))
                        .unwrap_or(serde_json::Value::Null)
                };
                outputs.insert(k.clone(), num);
            }
            let mut obj = serde_json::Map::new();
            obj.insert("step".to_string(), serde_json::Value::Number(row.into()));
            obj.insert("inputs".to_string(), serde_json::Value::Object(inputs));
            obj.insert("outputs".to_string(), serde_json::Value::Object(outputs));
            json_rows.push(serde_json::Value::Object(obj));
        } else {
            let row_vals: Vec<&str> = in_vals
                .iter()
                .map(|s| s.as_str())
                .chain(out_vals.iter().map(|s| s.as_str()))
                .collect();
            println!("{}", row_vals.join(" | "));
        }
    }

    if opts.format_json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::Value::Array(json_rows))
                .unwrap_or_else(|_| "[]".to_string())
        );
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
        if wiring > 0 {
            println!("      Wiring:     {}", wiring);
        }
        if gates > 0 {
            println!("      Gates:      {}", gates);
        }
        if plexers > 0 {
            println!("      Plexers:    {}", plexers);
        }
        if arithmetic > 0 {
            println!("      Arithmetic: {}", arithmetic);
        }
        if memory > 0 {
            println!("      Memory:     {}", memory);
        }
        if io > 0 {
            println!("      I/O:        {}", io);
        }
        if subcircuits > 0 {
            println!("      Subcircuits: {}", subcircuits);
        }
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

    #[test]
    fn test_parse_opts_format_json() {
        let args: Vec<String> = vec![
            "--format".to_string(),
            "json".to_string(),
            "f.circ".to_string(),
        ];
        let opts = parse_opts(&args).unwrap();
        assert!(opts.format_json);
    }

    #[test]
    fn test_run_truth_table_json() {
        let path = write_temp_circ(AND_CIRC);
        let args: Vec<String> = vec!["--format".to_string(), "json".to_string(), path];
        assert!(run_truth_table(&args).is_ok());
    }

    #[test]
    fn test_run_simulate_json() {
        let path = write_temp_circ(AND_CIRC);
        let args: Vec<String> = vec![
            "--steps".to_string(),
            "2".to_string(),
            "--format".to_string(),
            "json".to_string(),
            path,
        ];
        assert!(run_simulate(&args).is_ok());
    }
}
