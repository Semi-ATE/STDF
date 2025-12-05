use std::env;
use std::collections::HashMap;
use stdf::{StdfRecordIterator, stdf_record_type};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <stdf_file>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    match StdfRecordIterator::new(filename) {
        Ok(iter) => {
            let mut counts: HashMap<String, usize> = HashMap::new();
            let mut total = 0;
            
            for result in iter {
                match result {
                    Ok(record_bytes) => {
                        total += 1;
                        
                        // Get record type without parsing
                        match stdf_record_type(&record_bytes) {
                            Ok(rec_type) => {
                                *counts.entry(rec_type.to_string()).or_insert(0) += 1;
                            },
                            Err(e) => {
                                eprintln!("Error getting record type: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading record: {}", e);
                        break;
                    }
                }
            }
            
            println!("\n=== Record Type Summary ===");
            println!("Total records: {}\n", total);
            
            // Sort by record type name for consistent output
            let mut sorted: Vec<_> = counts.iter().collect();
            sorted.sort_by_key(|a| a.0);
            
            for (rec_type, count) in sorted {
                println!("{:6}: {:>10}", rec_type, count);
            }
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    }
}
