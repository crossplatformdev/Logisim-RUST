use clap::{Arg, Command};
use env_logger;
use log::{error, info};
use std::path::PathBuf;

use logisim_cli::{simulate, validate, CliConfig, CliResult};

fn main() -> CliResult<()> {
    // Initialize logging
    env_logger::init();

    let matches = Command::new("logisim-cli")
        .version("1.0.0")
        .author("Logisim-RUST Contributors")
        .about("Command-line interface for Logisim-RUST digital logic simulator")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("Suppress output")
                .action(clap::ArgAction::SetTrue),
        )
        .subcommand(
            Command::new("simulate")
                .about("Run circuit simulation")
                .arg(
                    Arg::new("file")
                        .help("Circuit file to simulate")
                        .required(true)
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("validate").about("Validate circuit file").arg(
                Arg::new("file")
                    .help("Circuit file to validate")
                    .required(true)
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
        .get_matches();

    let config = CliConfig {
        verbose: matches.get_flag("verbose"),
        quiet: matches.get_flag("quiet"),
    };

    match matches.subcommand() {
        Some(("simulate", sub_matches)) => {
            let file_path = sub_matches.get_one::<PathBuf>("file").unwrap();
            if config.verbose {
                info!("Starting simulation of: {}", file_path.display());
            }
            simulate(file_path)?;
            if !config.quiet {
                println!("Simulation completed successfully");
            }
        },
        Some(("validate", sub_matches)) => {
            let file_path = sub_matches.get_one::<PathBuf>("file").unwrap();
            if config.verbose {
                info!("Starting validation of: {}", file_path.display());
            }
            validate(file_path)?;
            if !config.quiet {
                println!("Validation completed successfully");
            }
        },
        _ => {
            error!("No subcommand provided. Use --help for usage information.");
            std::process::exit(1);
        },
    }

    Ok(())
}
