use std::env;
use std::process;
use std::io::{self, Write};

use stdf::{StdfRecordIterator, stdf_parse_record, V4};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "convert" | "to-atdf" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} convert <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match convert_to_atdf(filename) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "help" | "--help" | "-h" => {
            print_usage(&args[0]);
        }
        _ => {
            eprintln!("Error: Unknown command '{}'", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program_name: &str) {
    eprintln!("ATDF Converter - Convert STDF binary files to ATDF (ASCII) format");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  {} convert <stdf_file>     Convert STDF file to ATDF and write to stdout", program_name);
    eprintln!("  {} help                    Show this help message", program_name);
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  {} convert test.std                    # Print ATDF to console", program_name);
    eprintln!("  {} convert test.std > output.atd       # Save ATDF to file", program_name);
}

fn convert_to_atdf(filename: &str) -> io::Result<()> {
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for (i, raw_record) in iter.enumerate() {
        let raw_record = raw_record?;
        let record = stdf_parse_record(&raw_record, endian)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse record {}: {:?}", i, e)))?;
        
        let ascii_line = match record {
            V4::FAR(r) => r.ascii(),
            V4::MIR(r) => r.ascii(),
            V4::MRR(r) => r.ascii(),
            V4::PCR(r) => r.ascii(),
            V4::HBR(r) => r.ascii(),
            V4::SBR(r) => r.ascii(),
            V4::PMR(r) => r.ascii(),
            V4::PGR(r) => r.ascii(),
            V4::PLR(r) => r.ascii(),
            V4::RDR(r) => r.ascii(),
            V4::SDR(r) => r.ascii(),
            V4::WIR(r) => r.ascii(),
            V4::WRR(r) => r.ascii(),
            V4::WCR(r) => r.ascii(),
            V4::PIR(r) => r.ascii(),
            V4::PRR(r) => r.ascii(),
            V4::TSR(r) => r.ascii(),
            V4::PTR(r) => r.ascii(),
            V4::MPR(r) => r.ascii(),
            V4::FTR(r) => r.ascii(),
            V4::ATR(r) => r.ascii(),
            V4::BPS(r) => r.ascii(),
            V4::EPS(r) => r.ascii(),
            V4::GDR(r) => r.ascii(),
            V4::DTR(r) => r.ascii(),
            V4::Unknown(_) => {
                eprintln!("Warning: Skipping unknown record at index {}", i);
                continue;
            }
            V4::Invalid(_) => {
                eprintln!("Warning: Skipping invalid record at index {}", i);
                continue;
            }
        };
        
        writeln!(handle, "{}", ascii_line)?;
    }
    
    Ok(())
}
