use std::env;
use stdf::{StdfRecordIterator, parse_record, V4};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <stdf_file>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // Create the iterator
    match StdfRecordIterator::new(filename) {
        Ok(iter) => {
            let endian = iter.endian();
            let mut mir_count = 0;
            let mut prr_count = 0;
            let mut ptr_count = 0;
            let mut other_count = 0;
            
            for (idx, result) in iter.enumerate() {
                match result {
                    Ok(record_bytes) => {
                        // Use the factory to parse the raw bytes into a V4 record
                        match parse_record(&record_bytes, endian) {
                            Ok(record) => {
                                match record {
                                    V4::MIR(mir) => {
                                        mir_count += 1;
                                        if mir_count == 1 {
                                            println!("Found MIR record:");
                                            println!("{}", mir);
                                        }
                                    },
                                    V4::PRR(_) => {
                                        prr_count += 1;
                                        if prr_count <= 3 {
                                            println!("\nPRR record #{}:", prr_count);
                                        }
                                    },
                                    V4::PTR(_) => {
                                        ptr_count += 1;
                                    },
                                    _ => {
                                        other_count += 1;
                                    }
                                }
                            },
                            Err(e) => {
                                eprintln!("Error parsing record {}: {}", idx + 1, e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading record {}: {}", idx + 1, e);
                        break;
                    }
                }
            }
            
            println!("\n=== Record Summary ===");
            println!("MIR records: {}", mir_count);
            println!("PRR records: {}", prr_count);
            println!("PTR records: {}", ptr_count);
            println!("Other records: {}", other_count);
            println!("Total: {}", mir_count + prr_count + ptr_count + other_count);
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    }
}
