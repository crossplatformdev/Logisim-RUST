//! Application startup and command line parsing
//!
//! This module is the Rust equivalent of Java's Startup.java class.
//! It handles command line argument parsing and application initialization.

use crate::UiResult;
use logisim_core::{build_info::BuildInfo, prefs::AppPreferences};
use std::path::PathBuf;

/// Application startup configuration
/// Equivalent to Java's Startup class
pub struct Startup {
    /// Files to open on startup
    files_to_open: Vec<PathBuf>,

    /// Whether to quit immediately (e.g., after showing help)
    quit_flag: bool,

    /// Whether to run in headless mode
    headless: bool,

    /// Template file to use for new circuits
    template_file: Option<PathBuf>,

    /// Test bench mode
    test_bench: bool,

    /// Print mode
    print_mode: bool,

    /// Output file for non-interactive operations
    output_file: Option<PathBuf>,

    /// Substitution values for template variables
    substitutions: std::collections::HashMap<String, String>,
}

impl Startup {
    /// Parse command line arguments - equivalent to Java's Startup.parseArgs()
    pub fn parse_args(args: &[String]) -> Option<Self> {
        let mut startup = Self {
            files_to_open: Vec::new(),
            quit_flag: false,
            headless: false,
            template_file: None,
            test_bench: false,
            print_mode: false,
            output_file: None,
            substitutions: std::collections::HashMap::new(),
        };

        let mut i = 1; // Skip program name
        while i < args.len() {
            let arg = &args[i];

            match arg.as_str() {
                "--help" | "-h" => {
                    show_help(&args[0]);
                    startup.quit_flag = true;
                    return Some(startup);
                }

                "--version" | "-v" => {
                    show_version();
                    startup.quit_flag = true;
                    return Some(startup);
                }

                "--headless" => {
                    startup.headless = true;
                }

                "--template" => {
                    if i + 1 < args.len() {
                        startup.template_file = Some(PathBuf::from(&args[i + 1]));
                        i += 1; // Skip next argument
                    } else {
                        eprintln!("Error: --template requires a file path");
                        return None;
                    }
                }

                "--test-bench" => {
                    startup.test_bench = true;
                }

                "--print" => {
                    startup.print_mode = true;
                }

                "--output" => {
                    if i + 1 < args.len() {
                        startup.output_file = Some(PathBuf::from(&args[i + 1]));
                        i += 1; // Skip next argument
                    } else {
                        eprintln!("Error: --output requires a file path");
                        return None;
                    }
                }

                "--sub" => {
                    if i + 2 < args.len() {
                        let key = args[i + 1].clone();
                        let value = args[i + 2].clone();
                        startup.substitutions.insert(key, value);
                        i += 2; // Skip next two arguments
                    } else {
                        eprintln!("Error: --sub requires key and value");
                        return None;
                    }
                }

                arg if arg.starts_with('-') => {
                    eprintln!("Error: Unknown option: {}", arg);
                    return None;
                }

                _ => {
                    // Treat as file to open
                    let path = PathBuf::from(arg);
                    if path.exists() || arg.ends_with(".circ") {
                        startup.files_to_open.push(path);
                    } else {
                        eprintln!("Warning: File does not exist: {}", arg);
                    }
                }
            }

            i += 1;
        }

        Some(startup)
    }

    /// Check if the application should quit immediately
    pub fn should_quit(&self) -> bool {
        self.quit_flag
    }

    /// Run the application - equivalent to Java's Startup.run()
    pub fn run(self) -> UiResult<()> {
        // Set headless mode if requested
        if self.headless {
            // Set global headless flag via main module function
            crate::main::set_headless(true);
        }

        // Handle different startup modes
        if self.test_bench {
            return self.run_test_bench();
        }

        if self.print_mode {
            return self.run_print_mode();
        }

        // Normal GUI or headless mode
        if self.files_to_open.is_empty() {
            // No files specified - start with empty project or template
            if let Some(template) = self.template_file {
                crate::gui::app::run_app_with_template(template)
            } else {
                crate::gui::app::run_app()
            }
        } else if self.files_to_open.len() == 1 {
            // Single file - open it directly
            crate::gui::app::run_app_with_file(self.files_to_open[0].clone())
        } else {
            // Multiple files - open them all
            crate::gui::app::run_app_with_files(self.files_to_open)
        }
    }

    /// Run test bench mode
    fn run_test_bench(self) -> UiResult<()> {
        log::info!("Running test bench mode");

        // Test bench mode implementation would go here
        // For now, return not implemented error
        Err(crate::UiError::NotImplemented(
            "Test bench mode not implemented yet".to_string(),
        ))
    }

    /// Run print mode (headless printing of circuits)
    fn run_print_mode(self) -> UiResult<()> {
        log::info!("Running print mode");

        // Print mode implementation would go here
        // For now, return not implemented error
        Err(crate::UiError::NotImplemented(
            "Print mode not implemented yet".to_string(),
        ))
    }
}

/// Show help message
fn show_help(program_name: &str) {
    println!("{}", BuildInfo::full_version());
    println!();
    println!("Usage: {} [OPTIONS] [FILE...]", program_name);
    println!();
    println!("Options:");
    println!("  -h, --help          Show this help message");
    println!("  -v, --version       Show version information");
    println!("      --headless      Run in headless mode (no GUI)");
    println!("      --template FILE Use FILE as template for new circuits");
    println!("      --test-bench    Run in test bench mode");
    println!("      --print         Print circuits (requires --output)");
    println!("      --output FILE   Output file for non-interactive operations");
    println!("      --sub KEY VALUE Substitute VALUE for KEY in templates");
    println!();
    println!("Arguments:");
    println!("  FILE                Circuit files to open (.circ extension)");
    println!();
    println!("Examples:");
    println!(
        "  {}                    Start with empty project",
        program_name
    );
    println!(
        "  {} circuit.circ       Open specific circuit file",
        program_name
    );
    println!(
        "  {} --headless --print --output out.pdf circuit.circ",
        program_name
    );
    println!("                        Print circuit to PDF in headless mode");
    println!();
    println!("Environment Variables:");
    println!("  LOGISIM_RUST_LOG      Set log level (error, warn, info, debug, trace)");
    println!("  DISPLAY               Required for GUI mode on Linux");
}

/// Show version information
fn show_version() {
    println!("{}", BuildInfo::full_version());
    println!("Built for: {}", BuildInfo::TARGET);

    if let Some(git) = BuildInfo::GIT_HASH {
        println!("Git commit: {}", git);
    }

    if BuildInfo::DEBUG {
        println!("Build type: Debug");
    } else {
        println!("Build type: Release");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_args() {
        let args = vec!["program".to_string()];
        let startup = Startup::parse_args(&args).unwrap();
        assert!(startup.files_to_open.is_empty());
        assert!(!startup.should_quit());
        assert!(!startup.headless);
    }

    #[test]
    fn test_parse_help() {
        let args = vec!["program".to_string(), "--help".to_string()];
        let startup = Startup::parse_args(&args).unwrap();
        assert!(startup.should_quit());
    }

    #[test]
    fn test_parse_version() {
        let args = vec!["program".to_string(), "--version".to_string()];
        let startup = Startup::parse_args(&args).unwrap();
        assert!(startup.should_quit());
    }

    #[test]
    fn test_parse_headless() {
        let args = vec!["program".to_string(), "--headless".to_string()];
        let startup = Startup::parse_args(&args).unwrap();
        assert!(startup.headless);
    }

    #[test]
    fn test_parse_file() {
        let args = vec!["program".to_string(), "test.circ".to_string()];
        let startup = Startup::parse_args(&args).unwrap();
        assert_eq!(startup.files_to_open.len(), 1);
        assert_eq!(startup.files_to_open[0], PathBuf::from("test.circ"));
    }

    #[test]
    fn test_parse_template() {
        let args = vec![
            "program".to_string(),
            "--template".to_string(),
            "template.circ".to_string(),
        ];
        let startup = Startup::parse_args(&args).unwrap();
        assert_eq!(startup.template_file, Some(PathBuf::from("template.circ")));
    }

    #[test]
    fn test_parse_substitutions() {
        let args = vec![
            "program".to_string(),
            "--sub".to_string(),
            "NAME".to_string(),
            "VALUE".to_string(),
        ];
        let startup = Startup::parse_args(&args).unwrap();
        assert_eq!(
            startup.substitutions.get("NAME"),
            Some(&"VALUE".to_string())
        );
    }

    #[test]
    fn test_parse_invalid_option() {
        let args = vec!["program".to_string(), "--invalid".to_string()];
        let startup = Startup::parse_args(&args);
        assert!(startup.is_none());
    }
}
