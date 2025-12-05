use std::env;
use stdf::StdfRecordIterator;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <stdf_file> [stdf_record_type] [record_subtype]", args[0]);
        eprintln!("\nExample: {} file.std 1 10  (shows only MIR records)", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let filter_type = args.get(2).and_then(|s| s.parse::<u8>().ok());
    let filter_subtype = args.get(3).and_then(|s| s.parse::<u8>().ok());
    
    // Create the iterator
    match StdfRecordIterator::new(filename) {
        Ok(iter) => {
            let mut count = 0;
            let mut filtered_count = 0;
            
            for result in iter {
                match result {
                    Ok(record_bytes) => {
                        count += 1;
                        
                        // Parse the header to check record type
                        if record_bytes.len() >= 4 {
                            let rec_type = record_bytes[2];
                            let rec_subtype = record_bytes[3];
                            
                            // Apply filter if specified
                            let matches = match (filter_type, filter_subtype) {
                                (Some(ft), Some(fs)) => rec_type == ft && rec_subtype == fs,
                                (Some(ft), None) => rec_type == ft,
                                _ => true,
                            };
                            
                            if matches {
                                filtered_count += 1;
                                
                                // Show record details
                                let rec_name = get_record_name(rec_type, rec_subtype);
                                println!("Record {} - Type: {}, Subtype: {}, Name: {}, Size: {} bytes",
                                    count, rec_type, rec_subtype, rec_name, record_bytes.len());
                                
                                // Optionally show hex dump of first 64 bytes
                                if args.contains(&"--hex".to_string()) {
                                    print_hex_dump(&record_bytes, 64);
                                }
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading record: {}", e);
                        break;
                    }
                }
            }
            
            if filter_type.is_some() {
                println!("\nMatched {} of {} records", filtered_count, count);
            } else {
                println!("\nTotal records: {}", count);
            }
        },
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_record_name(rec_type: u8, rec_subtype: u8) -> &'static str {
    match (rec_type, rec_subtype) {
        (0, 10) => "FAR",
        (0, 20) => "ATR",
        (1, 10) => "MIR",
        (1, 20) => "MRR",
        (1, 30) => "PCR",
        (1, 40) => "HBR",
        (1, 50) => "SBR",
        (1, 60) => "PMR",
        (1, 62) => "PGR",
        (1, 63) => "PLR",
        (1, 70) => "RDR",
        (1, 80) => "SDR",
        (2, 10) => "WIR",
        (2, 20) => "WRR",
        (2, 30) => "WCR",
        (5, 10) => "PIR",
        (5, 20) => "PRR",
        (10, 30) => "TSR",
        (15, 10) => "PTR",
        (15, 15) => "MPR",
        (15, 20) => "FTR",
        (20, 10) => "BPS",
        (20, 20) => "EPS",
        (50, 10) => "GDR",
        (50, 30) => "DTR",
        _ => "Unknown",
    }
}

fn print_hex_dump(data: &[u8], max_bytes: usize) {
    let bytes_to_show = std::cmp::min(data.len(), max_bytes);
    println!("  Hex dump (first {} bytes):", bytes_to_show);
    
    for (i, chunk) in data[..bytes_to_show].chunks(16).enumerate() {
        print!("    {:04X}: ", i * 16);
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
}
