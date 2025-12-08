use stdf::{StdfRecordFPIterator, stdf_parse_record, extract_part_records, V4};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <stdf-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Extracting part records from: {}", file_path);
    println!("{}", "=".repeat(80));

    // Use the FP iterator to find PIR records and their file pointers
    let iter = StdfRecordFPIterator::new(file_path).expect("Failed to open file");
    let endian = iter.endian();
    
    let mut parts_extracted = 0;
    let max_parts = 5; // Only extract first 5 parts for demo
    
    for result in iter {
        if parts_extracted >= max_parts {
            break;
        }
        
        let (record_bytes, fp) = result.expect("Failed to read record");
        
        // Parse the record to check if it's a PIR
        if let Ok(V4::PIR(pir)) = stdf_parse_record(&record_bytes, endian) {
            println!("\n--- Part {}: HEAD={}, SITE={} ---", 
                     parts_extracted + 1, pir.head_num.0, pir.site_num.0);
            
            // Extract all records for this part
            match extract_part_records(file_path, &record_bytes, fp, endian) {
                Ok(part_bytes) => {
                    // Count record types
                    let mut ptr_count = 0;
                    let mut ftr_count = 0;
                    let mut mpr_count = 0;
                    
                    for bytes in &part_bytes[1..part_bytes.len()-1] { // Skip PIR and PRR
                        if let Ok(record) = stdf_parse_record(bytes, endian) {
                            match &record {
                                V4::PTR(_) => ptr_count += 1,
                                V4::FTR(_) => ftr_count += 1,
                                V4::MPR(_) => mpr_count += 1,
                                _ => {}
                            }
                        }
                    }
                    
                    println!("Extracted {} records:", part_bytes.len());
                    println!("  PIR: 1");
                    if ptr_count > 0 { println!("  PTR: {}", ptr_count); }
                    if ftr_count > 0 { println!("  FTR: {}", ftr_count); }
                    if mpr_count > 0 { println!("  MPR: {}", mpr_count); }
                    println!("  PRR: 1");
                    
                    parts_extracted += 1;
                }
                Err(e) => {
                    eprintln!("Error extracting part records: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("Extracted records for {} parts", parts_extracted);
}
