// Integration tests for parse -> binary -> parse roundtrip

mod common;

use stdf::{StdfRecordIterator, stdf_parse_record, V4};
use byte::ctx::Endian;

#[test]
fn roundtrip_all_records_from_real_file() {
    if !common::test_data_exists() {
        eprintln!("Skipping test: test data file not found");
        return;
    }
    
    let iter = StdfRecordIterator::new(common::test_data_dir())
        .expect("Failed to open test file");
    let endian = iter.endian();
    
    let mut tested = 0;
    
    for result in iter.take(100) {
        let original_bytes = result.expect("Failed to read record");
        
        let record = stdf_parse_record(&original_bytes, endian)
            .expect("Failed to parse record");
        
        let regenerated_bytes = get_record_binary(&record, endian);
        
        assert_eq!(
            original_bytes, 
            regenerated_bytes,
            "Record {} failed roundtrip: original != regenerated",
            tested
        );
        
        tested += 1;
    }
    
    assert!(tested > 0, "No records were tested");
    println!("✓ {} records passed roundtrip test", tested);
}

#[test]
fn roundtrip_preserves_header() {
    if !common::test_data_exists() {
        eprintln!("Skipping test: test data file not found");
        return;
    }
    
    let iter = StdfRecordIterator::new(common::test_data_dir())
        .expect("Failed to open test file");
    let endian = iter.endian();
    
    for result in iter.take(10) {
        let original_bytes = result.expect("Failed to read record");
        
        let record = stdf_parse_record(&original_bytes, endian)
            .expect("Failed to parse record");
        
        let regenerated_bytes = get_record_binary(&record, endian);
        
        // Check header specifically (first 4 bytes)
        assert_eq!(
            &original_bytes[0..4],
            &regenerated_bytes[0..4],
            "Header mismatch"
        );
    }
}

#[test]
fn roundtrip_specific_far_record() {
    // FAR with Big Endian: REC_LEN=2, REC_TYP=0, REC_SUB=10, cpu_type=2, stdf_ver=4
    let original = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
    
    let record = stdf_parse_record(&original, Endian::Big)
        .expect("Failed to parse FAR");
    
    if let V4::FAR(far) = record {
        let regenerated = far.binary(Endian::Big);
        assert_eq!(original, regenerated, "FAR roundtrip failed");
        
        // Verify fields
        assert_eq!(far.cpu_type.0, 2);
        assert_eq!(far.stdf_ver.0, 4);
    } else {
        panic!("Expected FAR record");
    }
}

#[test]
fn roundtrip_specific_pir_record() {
    // PIR with Big Endian: REC_LEN=2, REC_TYP=5, REC_SUB=10, head_num=1, site_num=1
    let original = vec![0x00, 0x02, 0x05, 0x0A, 0x01, 0x01];
    
    let record = stdf_parse_record(&original, Endian::Big)
        .expect("Failed to parse PIR");
    
    if let V4::PIR(pir) = record {
        let regenerated = pir.binary(Endian::Big);
        assert_eq!(original, regenerated, "PIR roundtrip failed");
        
        // Verify fields
        assert_eq!(pir.head_num.0, 1);
        assert_eq!(pir.site_num.0, 1);
    } else {
        panic!("Expected PIR record");
    }
}

#[test]
fn roundtrip_respects_endianness() {
    // Test with little endian FAR
    let le_bytes = vec![0x02, 0x00, 0x00, 0x0A, 0x02, 0x04];
    
    let record = stdf_parse_record(&le_bytes, Endian::Little)
        .expect("Failed to parse LE FAR");
    
    if let V4::FAR(far) = record {
        let regenerated = far.binary(Endian::Little);
        assert_eq!(le_bytes, regenerated, "LE FAR roundtrip failed");
    } else {
        panic!("Expected FAR record");
    }
    
    // Test with big endian FAR
    let be_bytes = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
    
    let record = stdf_parse_record(&be_bytes, Endian::Big)
        .expect("Failed to parse BE FAR");
    
    if let V4::FAR(far) = record {
        let regenerated = far.binary(Endian::Big);
        assert_eq!(be_bytes, regenerated, "BE FAR roundtrip failed");
    } else {
        panic!("Expected FAR record");
    }
}

// Helper function to extract binary from any V4 record
fn get_record_binary(record: &V4, endian: Endian) -> Vec<u8> {
    match record {
        V4::FAR(r) => r.binary(endian),
        V4::ATR(r) => r.binary(endian),
        V4::MIR(r) => r.binary(endian),
        V4::MRR(r) => r.binary(endian),
        V4::PCR(r) => r.binary(endian),
        V4::HBR(r) => r.binary(endian),
        V4::SBR(r) => r.binary(endian),
        V4::PMR(r) => r.binary(endian),
        V4::PGR(r) => r.binary(endian),
        V4::PLR(r) => r.binary(endian),
        V4::RDR(r) => r.binary(endian),
        V4::SDR(r) => r.binary(endian),
        V4::WIR(r) => r.binary(endian),
        V4::WRR(r) => r.binary(endian),
        V4::WCR(r) => r.binary(endian),
        V4::PIR(r) => r.binary(endian),
        V4::PRR(r) => r.binary(endian),
        V4::TSR(r) => r.binary(endian),
        V4::PTR(r) => r.binary(endian),
        V4::MPR(r) => r.binary(endian),
        V4::FTR(r) => r.binary(endian),
        V4::BPS(r) => r.binary(endian),
        V4::EPS(r) => r.binary(endian),
        V4::GDR(r) => r.binary(endian),
        V4::DTR(r) => r.binary(endian),
        _ => vec![],
    }
}
