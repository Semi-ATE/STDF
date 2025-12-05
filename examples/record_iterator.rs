use std::env;
use stdf::StdfRecordIterator;

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
            let mut count = 0;
            
            for (idx, result) in iter.enumerate() {
                match result {
                    Ok(record_bytes) => {
                        count += 1;
                        
                        // Show the first few records
                        if idx < 5 {
                            println!("Record {}: {} bytes", idx + 1, record_bytes.len());
                            
                            // Show the header (first 4 bytes)
                            if record_bytes.len() >= 4 {
                                println!("  Header bytes: {:02X} {:02X} {:02X} {:02X}",
                                    record_bytes[0], record_bytes[1],
                                    record_bytes[2], record_bytes[3]);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading record {}: {}", idx + 1, e);
                        break;
                    }
                }
            }
            
            println!("\nTotal records processed: {}", count);
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    }
}
