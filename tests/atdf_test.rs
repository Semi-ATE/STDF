use stdf::{StdfRecordIterator, stdf_parse_record, V4};
use std::path::Path;

#[test]
fn test_stdf_to_atdf_conversion() {
    // Use a known working STDF file for testing
    let stdf_file = r".\data\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std";
    
    // Skip test if file doesn't exist
    if !Path::new(stdf_file).exists() {
        eprintln!("Test file not found, skipping test: {}", stdf_file);
        return;
    }
    
    // Convert STDF to ATDF - just verify it doesn't crash
    let iter = StdfRecordIterator::new(stdf_file)
        .expect("Failed to open STDF file");
    let endian = iter.endian();
    
    let mut generated_atdf = String::new();
    let mut record_count = 0;
    
    for (i, raw_record) in iter.enumerate().take(100) {  // Test first 100 records
        let raw_record = raw_record.expect(&format!("Failed to read record {}", i));
        let record = stdf_parse_record(&raw_record, endian)
            .expect(&format!("Failed to parse record {}", i));
        
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
            V4::Unknown(_) | V4::Invalid(_) => {
                panic!("Unexpected Unknown or Invalid record at index {}", i);
            }
        };
        
        generated_atdf.push_str(&ascii_line);
        generated_atdf.push('\n');
        record_count += 1;
    }
    
    // Basic sanity checks
    assert!(record_count > 0, "No records were converted");
    assert!(generated_atdf.contains("FAR:"), "Generated ATDF should contain FAR record");
    assert!(generated_atdf.contains("MIR:"), "Generated ATDF should contain MIR record");
    
    // Verify each line starts with a valid record type
    for (i, line) in generated_atdf.lines().enumerate() {
        assert!(line.contains(':'), "Line {} should contain a colon separator: {}", i + 1, line);
        let record_type = line.split(':').next().unwrap();
        assert!(!record_type.is_empty(), "Line {} should have a record type", i + 1);
    }
    
    println!("Successfully converted {} records", record_count);
    println!("\nAll {} records in ATDF format:\n", record_count);
    for (i, line) in generated_atdf.lines().enumerate() {
        println!("{:3}: {}", i + 1, line);
    }
}
