use stdf::{StdfRecordIterator, parse_record};
use byte::ctx::Endian;

fn main() {
    let test_file = r".\data\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std";
    
    let iter = StdfRecordIterator::new(test_file).expect("Failed to open file");
    let endian = iter.endian();
    
    println!("=== STDF Binary Round-Trip Test ===");
    println!("Endianness: {:?}\n", endian);
    
    let mut tested = 0;
    let mut matched = 0;
    let mut mismatched = 0;
    
    for (idx, result) in iter.enumerate().take(100) {
        let original_bytes = result.expect("Failed to read record");
        
        // Parse the record
        let record = match parse_record(&original_bytes, endian) {
            Ok(r) => r,
            Err(e) => {
                println!("Record {}: Parse error: {}", idx, e);
                continue;
            }
        };
        
        // Convert back to binary
        let regenerated_bytes = match record {
            stdf::V4::FAR(r) => r.binary(endian),
            stdf::V4::ATR(r) => r.binary(endian),
            stdf::V4::MIR(r) => r.binary(endian),
            stdf::V4::MRR(r) => r.binary(endian),
            stdf::V4::PCR(r) => r.binary(endian),
            stdf::V4::HBR(r) => r.binary(endian),
            stdf::V4::SBR(r) => r.binary(endian),
            stdf::V4::PMR(r) => r.binary(endian),
            stdf::V4::PGR(r) => r.binary(endian),
            stdf::V4::PLR(r) => r.binary(endian),
            stdf::V4::RDR(r) => r.binary(endian),
            stdf::V4::SDR(r) => r.binary(endian),
            stdf::V4::WIR(r) => r.binary(endian),
            stdf::V4::WRR(r) => r.binary(endian),
            stdf::V4::WCR(r) => r.binary(endian),
            stdf::V4::PIR(r) => r.binary(endian),
            stdf::V4::PRR(r) => r.binary(endian),
            stdf::V4::TSR(r) => r.binary(endian),
            stdf::V4::PTR(r) => r.binary(endian),
            stdf::V4::MPR(r) => r.binary(endian),
            stdf::V4::FTR(r) => r.binary(endian),
            stdf::V4::BPS(r) => r.binary(endian),
            stdf::V4::EPS(r) => r.binary(endian),
            stdf::V4::GDR(r) => r.binary(endian),
            stdf::V4::DTR(r) => r.binary(endian),
            _ => {
                continue;
            }
        };
        
        tested += 1;
        
        // Compare
        if original_bytes == regenerated_bytes {
            matched += 1;
        } else {
            mismatched += 1;
            if mismatched <= 5 {  // Show first 5 mismatches
                println!("\nRecord {} MISMATCH:", idx);
                println!("  Original  ({:3} bytes): {:02X?}", original_bytes.len(), &original_bytes[..original_bytes.len().min(32)]);
                println!("  Generated ({:3} bytes): {:02X?}", regenerated_bytes.len(), &regenerated_bytes[..regenerated_bytes.len().min(32)]);
            }
        }
    }
    
    println!("\n=== Results ===");
    println!("Tested:     {}", tested);
    println!("Matched:    {} ({:.1}%)", matched, (matched as f64 / tested as f64) * 100.0);
    println!("Mismatched: {} ({:.1}%)", mismatched, (mismatched as f64 / tested as f64) * 100.0);
    
    if mismatched == 0 {
        println!("\n✓ SUCCESS: All records round-trip correctly!");
    } else {
        println!("\n✗ FAILURE: Some records do not round-trip");
    }
}
