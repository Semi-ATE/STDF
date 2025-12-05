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
        "info" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} info <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match info_stdf(filename) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "dump" => {
            // Parse arguments: stdf dump [record_types...] <file>
            // The last argument is always the file
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} dump [record_types...] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let filename = &args[args.len() - 1];
            let record_types: Vec<String> = args[2..args.len() - 1]
                .iter()
                .map(|s| s.to_uppercase())
                .collect();
            
            // Remove duplicates
            let mut unique_types: Vec<String> = Vec::new();
            for rt in record_types {
                if !unique_types.contains(&rt) {
                    unique_types.push(rt);
                }
            }
            
            match dump_stdf(filename, &unique_types) {
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
        "is_ft" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} is_ft <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match check_is_ft(filename) {
                Ok(true) => process::exit(0),  // Is FT
                Ok(false) => process::exit(1), // Not FT (is WS)
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(2);
                }
            }
        }
        "is_ws" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} is_ws <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match check_is_ws(filename) {
                Ok(true) => process::exit(0),  // Is WS
                Ok(false) => process::exit(1), // Not WS (is FT)
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(2);
                }
            }
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
    println!("  info <file>                      - Display MIR, WRR, and MRR records");
    println!("  dump <file>                      - Dump all records");
    println!("  dump <record_types...> <file>    - Dump specific record types");
    println!("                                     Example: dump MIR PRR file.std");
    println!("  validate <file>  - Validate STDF file structure");
    println!("  is_ft <file>     - Check if file is Final Test (exit 0=FT, 1=WS, 2=error)");
    println!("  is_ws <file>     - Check if file is Wafer Sort (exit 0=WS, 1=FT, 2=error)");
    println!("  convert <file>   - Convert STDF file to another format (not yet implemented)");
    println!("  help             - Show this help message");
}

fn info_stdf(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    parser.info_file(filename)
}

fn dump_stdf(filename: &str, record_types: &[String]) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    parser.dump_file(filename, record_types)
}

fn validate_stdf(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    let count = parser.count_records(filename)?;
    println!("Successfully parsed {} records", count);
    Ok(())
}

fn check_is_ft(filename: &str) -> Result<bool, std::io::Error> {
    let parser = StdfParser::new();
    let has_wir = parser.has_wir_record(filename)?;
    Ok(!has_wir) // FT if no WIR found
}

fn check_is_ws(filename: &str) -> Result<bool, std::io::Error> {
    let parser = StdfParser::new();
    let has_wir = parser.has_wir_record(filename)?;
    Ok(has_wir) // WS if WIR found
}
