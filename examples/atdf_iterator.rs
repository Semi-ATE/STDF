use stdf::parsers::AtdfRecordIterator;

fn main() {
    let atdf_file = r".\data\3xjxnh02.atd";
    
    println!("Reading ATDF file: {}\n", atdf_file);
    
    let iter = AtdfRecordIterator::new(atdf_file)
        .expect("Failed to open ATDF file");
    
    let mut record_count = 0;
    let mut stdf_record_type_counts = std::collections::HashMap::new();
    
    println!("Processing first 20 records:\n");
    
    for (i, line_result) in iter.enumerate().take(20) {
        let line = line_result.expect(&format!("Failed to read line {}", i + 1));
        
        // Extract record type (before the colon)
        let stdf_record_type = line.split(':').next().unwrap_or("UNKNOWN");
        
        // Count occurrences of each record type
        *stdf_record_type_counts.entry(stdf_record_type.to_string()).or_insert(0) += 1;
        
        record_count += 1;
        
        // Print all 20 records (truncate long lines for display)
        let display_line = if line.len() > 120 {
            format!("{}...", &line[..120])
        } else {
            line.clone()
        };
        println!("{:3}: {}", i + 1, display_line);
    }
    
    println!("\n--- Summary (first 20 records) ---");
    println!("Total records processed: {}", record_count);
    println!("\nRecord type counts:");
    let mut types: Vec<_> = stdf_record_type_counts.iter().collect();
    types.sort_by_key(|(name, _)| *name);
    for (stdf_record_type, count) in types {
        println!("  {}: {}", stdf_record_type, count);
    }
}
