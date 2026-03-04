//! Logisim-RUST Command-Line Interface
//!
//! Provides non-interactive circuit simulation, truth-table generation,
//! and file format conversion.
//!
//! # Usage
//!
//! ```text
//! logisim-cli [OPTIONS] <COMMAND>
//!
//! Commands:
//!   simulate   Run the simulator for a given circuit file
//!   truth-table  Generate a truth table for a combinational circuit
//!   convert    Convert between file formats (future: VHDL, Verilog export)
//!   info       Print project structure information
//! ```

mod commands;

use commands::{run_info, run_simulate, run_truth_table};
use std::env;
use std::process;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let result = match args[1].as_str() {
        "simulate" => run_simulate(&args[2..]),
        "truth-table" => run_truth_table(&args[2..]),
        "info" => run_info(&args[2..]),
        "--help" | "-h" | "help" => {
            print_usage(&args[0]);
            Ok(())
        }
        "--version" | "-V" => {
            println!("logisim-cli {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        cmd => {
            eprintln!("Unknown command: {}", cmd);
            print_usage(&args[0]);
            Err(format!("Unknown command: {}", cmd))
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn print_usage(prog: &str) {
    eprintln!(
        "Usage: {} <COMMAND> [OPTIONS] <FILE>

Commands:
  simulate     Simulate a .circ file and print output pin values
  truth-table  Generate a complete truth table for a combinational circuit
  info         Display project/circuit structure information

Options:
  --circuit <name>    Select which circuit to operate on (default: main)
  --steps <n>         Number of simulation steps (default: 10)
  --terse             Terse output (values only)
  --format <fmt>      Output format: 'text' (default) or 'json'
  --help              Show this help message
  --version           Show version

Examples:
  {} simulate --steps 5 my_circuit.circ
  {} truth-table --circuit main my_circuit.circ
  {} truth-table --format json my_circuit.circ
  {} info my_circuit.circ
",
        prog, prog, prog, prog, prog
    );
}
