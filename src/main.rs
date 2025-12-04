use std::env;
use std::process;

use stdf::parser::StdfParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "info" | "dump" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} info <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match dump_stdf(filename) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "validate" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} validate <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match validate_stdf(filename) {
                Ok(_) => println!("File is valid"),
                Err(e) => {
                    eprintln!("Validation failed: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "convert" => {
            println!("Convert command not yet implemented");
            // TODO: Implement convert command
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("Semi-ATE STDF Tool");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("\nUsage: {} <command> [args...]", program);
    println!("\nCommands:");
    println!("  info <file>      - Display all records in an STDF file");
    println!("  dump <file>      - Alias for info");
    println!("  validate <file>  - Validate STDF file structure");
    println!("  convert <file>   - Convert STDF file to another format (not yet implemented)");
    println!("  help             - Show this help message");
}

fn dump_stdf(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    parser.dump_file(filename)
}

fn validate_stdf(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    let count = parser.count_records(filename)?;
    println!("Successfully parsed {} records", count);
    Ok(())
}
