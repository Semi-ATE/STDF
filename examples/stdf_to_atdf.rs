use stdf::{StdfRecordIterator, stdf_parse_record};

fn main() {
    let test_file = r".\data\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std";
    
    let iter = StdfRecordIterator::new(test_file).expect("Failed to open file");
    let endian = iter.endian();
    
    println!("=== STDF to ASCII (ATDF) Conversion ===\n");
    
    for (idx, result) in iter.enumerate().take(20) {
        let record_bytes = result.expect("Failed to read record");
        
        if let Ok(record) = stdf_parse_record(&record_bytes, endian) {
            let ascii_line = match record {
                stdf::V4::FAR(r) => r.ascii(),
                stdf::V4::MIR(r) => r.ascii(),
                stdf::V4::MRR(r) => r.ascii(),
                stdf::V4::PIR(r) => r.ascii(),
                stdf::V4::PRR(r) => r.ascii(),
                stdf::V4::PTR(r) => r.ascii(),
                stdf::V4::WIR(r) => r.ascii(),
                stdf::V4::WRR(r) => r.ascii(),
                stdf::V4::PCR(r) => r.ascii(),
                stdf::V4::TSR(r) => r.ascii(),
                _ => format!("Record {}: Not yet implemented", idx),
            };
            println!("{}", ascii_line);
        }
    }
}
