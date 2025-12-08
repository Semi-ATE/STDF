use stdf::{StdfRecordFPIterator, stdf_parse_record, extract_part_records, V4};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <stdf-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    
    // Print table header
    println!("{:<20} | {:>10} | {:>10} | {:>15} | {:>10} | {:>10} | {}", 
             "PART_ID", "HEAD_NUM", "SITE_NUM", "RECORD_COUNT", "NUM_TEST", "HARD_BIN", "FLAG");
    println!("{}", "=".repeat(91));

    // Use the FP iterator to find PIR records and their file pointers
    let iter = StdfRecordFPIterator::new(file_path).expect("Failed to open file");
    let endian = iter.endian();
    
    let mut parts_processed = 0;
    
    for result in iter {
        let (record_bytes, fp) = result.expect("Failed to read record");
        
        // Parse the record to check if it's a PIR
        if let Ok(V4::PIR(pir)) = stdf_parse_record(&record_bytes, endian) {
            // Extract all records for this part
            match extract_part_records(file_path, &record_bytes, fp, endian) {
                Ok(part_bytes) => {
                    let record_count = part_bytes.len();
                    
                    // Parse the last record (should be PRR)
                    if let Some(prr_bytes) = part_bytes.last() {
                        if let Ok(V4::PRR(prr)) = stdf_parse_record(prr_bytes, endian) {
                            let part_id = String::from_utf8_lossy(prr.part_id.0).to_string();
                            let part_id_display = if part_id.is_empty() { 
                                String::from("-") 
                            } else { 
                                part_id 
                            };
                            
                            // Check if record_count - 2 (PIR + PRR) equals num_test
                            let flag = if (record_count - 2) == prr.num_test.0 as usize {
                                "✓"
                            } else {
                                "✗"
                            };
                            
                            println!("{:<20} | {:>10} | {:>10} | {:>15} | {:>10} | {:>10} | {}", 
                                     part_id_display,
                                     prr.head_num.0,
                                     prr.site_num.0,
                                     record_count,
                                     prr.num_test.0,
                                     prr.hard_bin.0,
                                     flag);
                            
                            parts_processed += 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error extracting part records: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
    
    println!("{}", "=".repeat(91));
    println!("Total parts: {}", parts_processed);
}
