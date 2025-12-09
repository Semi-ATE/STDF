use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};
use std::path::Path;

extern crate byte;
use byte::BytesExt;

use crate::records::{Header, V4};

pub struct StdfParser {
    // Parser for STDF files
}

impl StdfParser {
    pub fn new() -> Self {
        StdfParser {}
    }

    /// Read file content
    fn read_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Check if file contains WIR records (indicates Wafer Sort)
    pub fn has_wir_record<P: AsRef<Path>>(&self, path: P) -> Result<bool, Error> {
        let iter = StdfRecordIterator::new(path)?;
        
        for result in iter {
            let record_bytes = result?;
            // WIR has REC_TYP = 2 and REC_SUB = 10
            // Bytes [2] and [3] contain REC_TYP and REC_SUB respectively
            if record_bytes.len() >= 4 && record_bytes[2] == 2 && record_bytes[3] == 10 {
                return Ok(true);
            }
        }
        
        Ok(false)
    }

    /// Count records in an STDF file
    pub fn count_records<P: AsRef<Path>>(&self, path: P) -> Result<usize, Error> {
        let iter = StdfRecordIterator::new(path)?;
        let mut count = 0;
        
        for result in iter {
            result?;
            count += 1;
        }
        
        Ok(count)
    }

    /// Display MIR, WRR, and MRR records from an STDF file
    pub fn info_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let iter = StdfRecordIterator::new(path)?;
        let endian = iter.endian();
        
        let mut mir_found = false;
        let mut wrr_found = false;
        let mut mrr_found = false;
        
        for result in iter {
            let record_bytes = result?;
            match stdf_parse_record(&record_bytes, endian)? {
                V4::MIR(mir) => {
                    println!("{}", mir);
                    mir_found = true;
                },
                V4::WRR(wrr) => {
                    if !wrr_found {
                        println!("{}", wrr);
                        wrr_found = true;
                    }
                },
                V4::MRR(mrr) => {
                    println!("{}", mrr);
                    mrr_found = true;
                    // MRR is typically the last record, so we can stop here
                    break;
                },
                _ => {}
            }
        }
        
        if !mir_found {
            eprintln!("Warning: No MIR record found in file");
        }
        if !wrr_found {
            eprintln!("Warning: No WRR record found in file");
        }
        if !mrr_found {
            eprintln!("Warning: No MRR record found in file");
        }
        
        Ok(())
    }

    /// Dump records from an STDF file, optionally filtering by record types
    pub fn dump_file<P: AsRef<Path>>(&self, path: P, record_types: &[String]) -> Result<(), Error> {
        let iter = StdfRecordIterator::new(path)?;
        let endian = iter.endian();
        let dump_all = record_types.is_empty();
        
        for result in iter {
            let record_bytes = result?;
            let record_name = stdf_record_type(&record_bytes)?;
            
            if dump_all || record_types.contains(&record_name.to_string()) {
                let v4 = stdf_parse_record(&record_bytes, endian)?;
                println!("{}", v4);
            }
        }
        
        Ok(())
    }
}

/// Extract part records (PIR through PRR) for a specific part
/// 
/// Given a PIR record and its file position, reads all test records (PTR, FTR, MPR)
/// and the PRR that belong to the same part (matching HEAD_NUM and SITE_NUM).
/// 
/// Returns the raw bytes of all matching records.
/// 
/// # Arguments
/// * `filename` - Path to the STDF file
/// * `pir_bytes` - Raw bytes of the PIR record
/// * `file_pointer` - File offset immediately after the PIR record
/// * `endian` - Endianness of the file (from iterator or FAR record)
/// 
/// # Returns
/// * `Ok(Vec<Vec<u8>>)` - Vector of raw record bytes (PIR, test records, PRR)
/// * `Err(Error)` - If file operations fail or parsing fails
/// 
/// # Example
/// ```no_run
/// use stdf::{extract_part_records, StdfRecordFPIterator, stdf_parse_record};
/// 
/// let mut iter = StdfRecordFPIterator::new("test.std").unwrap();
/// let endian = iter.endian();
/// 
/// for result in iter {
///     let (record_bytes, fp) = result.unwrap();
///     if let Ok(rec) = stdf_parse_record(&record_bytes, endian) {
///         if let stdf::V4::PIR(_) = rec {
///             let part_bytes = extract_part_records("test.std", &record_bytes, fp, endian).unwrap();
///             println!("Part has {} records", part_bytes.len());
///             // Parse each record as needed
///             for bytes in &part_bytes {
///                 let record = stdf_parse_record(bytes, endian).unwrap();
///                 println!("{}", record);
///             }
///             break;
///         }
///     }
/// }
/// ```
pub fn extract_part_records<P: AsRef<Path>>(
    filename: P,
    pir_bytes: &[u8],
    file_pointer: usize,
    endian: byte::ctx::Endian,
) -> Result<Vec<Vec<u8>>, Error> {
    
    // Parse the PIR to get HEAD_NUM and SITE_NUM
    let pir_record = stdf_parse_record(pir_bytes, endian)?;
    
    let (head_num, site_num) = match &pir_record {
        V4::PIR(pir) => (pir.head_num.0, pir.site_num.0),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Expected PIR record"
            ));
        }
    };
    
    // Initialize result vector with the PIR bytes
    let mut record_bytes_vec = vec![pir_bytes.to_vec()];
    
    // Open file and seek to the position after PIR
    let mut file = File::open(filename)?;
    file.seek(SeekFrom::Start(file_pointer as u64))?;
    
    // Read remaining file content from this position
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Create iterator from the buffer with known endianness
    let iter = StdfRecordIterator::from_buffer_with_endian(buffer, endian);
    
    // Process records until we find matching PRR
    for result in iter {
        let record_bytes = result?;
        let record = stdf_parse_record(&record_bytes, endian)?;
        
        let (matches, is_prr) = match &record {
            V4::PTR(ptr) => (ptr.head_num.0 == head_num && ptr.site_num.0 == site_num, false),
            V4::FTR(ftr) => (ftr.head_num.0 == head_num && ftr.site_num.0 == site_num, false),
            V4::MPR(mpr) => (mpr.head_num.0 == head_num && mpr.site_num.0 == site_num, false),
            V4::PRR(prr) => (prr.head_num.0 == head_num && prr.site_num.0 == site_num, true),
            _ => (false, false),
        };
        
        if matches {
            record_bytes_vec.push(record_bytes);
            
            // Check if this was the PRR - if so, we're done
            if is_prr {
                break;
            }
        }
    }
    
    Ok(record_bytes_vec)
}

/// Iterator that yields raw STDF records (including header) as byte slices
pub struct StdfRecordIterator {
    buffer: Vec<u8>,
    offset: usize,
    endian: byte::ctx::Endian,
}

impl StdfRecordIterator {
    /// Create a new iterator from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buffer = StdfParser::read_file_content(path)?;
        let endian = Header::detect_endian(&buffer)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;
        
        Ok(StdfRecordIterator {
            buffer,
            offset: 0,
            endian,
        })
    }

    /// Create a new iterator from a buffer
    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        let endian = Header::detect_endian(&buffer)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;
        
        Ok(StdfRecordIterator {
            buffer,
            offset: 0,
            endian,
        })
    }
    
    /// Create a new iterator from a buffer with known endianness
    pub fn from_buffer_with_endian(buffer: Vec<u8>, endian: byte::ctx::Endian) -> Self {
        StdfRecordIterator {
            buffer,
            offset: 0,
            endian,
        }
    }
    
    /// Get the endianness of the STDF file
    pub fn endian(&self) -> byte::ctx::Endian {
        self.endian
    }
}

impl<'a> Iterator for StdfRecordIterator {
    type Item = Result<Vec<u8>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.buffer.len() {
            return None;
        }

        let start_offset = self.offset;
        
        // Read the header to get the record length
        let header_result = self.buffer[..].read_with::<Header>(&mut self.offset, self.endian);
        
        match header_result {
            Ok(header) => {
                let record_length = header.rec_len.0 as usize;
                let header_size = self.offset - start_offset;
                let total_size = header_size + record_length;
                
                // Check if we have enough data
                if start_offset + total_size > self.buffer.len() {
                    return Some(Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        format!("Incomplete record at offset {}", start_offset)
                    )));
                }
                
                // Extract the complete record (header + data)
                let record_bytes = self.buffer[start_offset..start_offset + total_size].to_vec();
                
                // Move offset past the data
                self.offset = start_offset + total_size;
                
                Some(Ok(record_bytes))
            },
            Err(byte::Error::BadOffset(x)) => {
                if x == self.buffer.len() {
                    None  // End of file
                } else {
                    Some(Err(Error::new(
                        ErrorKind::Other,
                        format!("Bad offset {} before EOF", x)
                    )))
                }
            },
            Err(e) => Some(Err(Error::new(
                ErrorKind::Other,
                format!("Error reading header: {:?}", e)
            ))),
        }
    }
}

/// Iterator that yields raw STDF records with file pointers
/// Returns tuple of (record bytes, file pointer after record)
pub struct StdfRecordFPIterator {
    buffer: Vec<u8>,
    offset: usize,
    endian: byte::ctx::Endian,
}

impl StdfRecordFPIterator {
    /// Create a new iterator from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let buffer = StdfParser::read_file_content(path)?;
        let endian = Header::detect_endian(&buffer)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;
        
        Ok(StdfRecordFPIterator {
            buffer,
            offset: 0,
            endian,
        })
    }

    /// Create a new iterator from a buffer
    pub fn from_buffer(buffer: Vec<u8>) -> Result<Self, Error> {
        let endian = Header::detect_endian(&buffer)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;
        
        Ok(StdfRecordFPIterator {
            buffer,
            offset: 0,
            endian,
        })
    }
    
    /// Get the endianness of the STDF file
    pub fn endian(&self) -> byte::ctx::Endian {
        self.endian
    }
}

impl<'a> Iterator for StdfRecordFPIterator {
    type Item = Result<(Vec<u8>, usize), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.buffer.len() {
            return None;
        }

        let start_offset = self.offset;
        
        // Read the header to get the record length
        let header_result = self.buffer[..].read_with::<Header>(&mut self.offset, self.endian);
        
        match header_result {
            Ok(header) => {
                let record_length = header.rec_len.0 as usize;
                let header_size = self.offset - start_offset;
                let total_size = header_size + record_length;
                
                // Check if we have enough data
                if start_offset + total_size > self.buffer.len() {
                    return Some(Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        format!("Incomplete record at offset {}", start_offset)
                    )));
                }
                
                // Extract the complete record (header + data)
                let record_bytes = self.buffer[start_offset..start_offset + total_size].to_vec();
                
                // Move offset past the data
                self.offset = start_offset + total_size;
                
                // Return record and file pointer after the record
                Some(Ok((record_bytes, self.offset)))
            },
            Err(byte::Error::BadOffset(x)) => {
                if x == self.buffer.len() {
                    None  // End of file
                } else {
                    Some(Err(Error::new(
                        ErrorKind::Other,
                        format!("Bad offset {} before EOF", x)
                    )))
                }
            },
            Err(e) => Some(Err(Error::new(
                ErrorKind::Other,
                format!("Error reading header: {:?}", e)
            ))),
        }
    }
}

/// Iterator that yields ATDF records as parsed strings
pub struct AtdfRecordIterator {
    lines: Vec<String>,
    index: usize,
}

impl AtdfRecordIterator {
    /// Create a new iterator from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = fs::read_to_string(path)?;
        let lines: Vec<String> = content
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        Ok(AtdfRecordIterator {
            lines,
            index: 0,
        })
    }

    /// Create a new iterator from a string
    pub fn from_string(content: String) -> Self {
        let lines: Vec<String> = content
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        AtdfRecordIterator {
            lines,
            index: 0,
        }
    }
}

impl Iterator for AtdfRecordIterator {
    type Item = Result<String, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.lines.len() {
            return None;
        }

        let line = self.lines[self.index].clone();
        self.index += 1;

        // Skip empty lines
        if line.trim().is_empty() {
            return self.next();
        }

        // Validate line has ATDF format (RECORD_TYPE:fields...)
        if !line.contains(':') {
            return Some(Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid ATDF format at line {}: missing colon separator", self.index)
            )));
        }

        Some(Ok(line))
    }
}

/// Get the record type name from raw record bytes
/// 
/// # Arguments
/// * `record_bytes` - Complete record including header (must be at least 4 bytes)
/// 
/// # Returns
/// * `Ok(&str)` - The record type name (e.g., "FAR", "MIR", "PRR")
/// * `Err(Error)` - If the record is too short
/// 
/// # Example
/// ```no_run
/// use stdf::{StdfRecordIterator, stdf_record_type};
/// 
/// let iter = StdfRecordIterator::new("test.std").unwrap();
/// 
/// for result in iter {
///     let record_bytes = result.unwrap();
///     let rec_type = stdf_record_type(&record_bytes).unwrap();
///     println!("Record type: {}", rec_type);
/// }
/// ```
pub fn stdf_record_type(record_bytes: &[u8]) -> Result<&'static str, Error> {
    if record_bytes.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Record too short: {} bytes (minimum 4)", record_bytes.len())
        ));
    }
    
    let rec_typ = record_bytes[2];
    let rec_sub = record_bytes[3];
    
    Ok(match (rec_typ, rec_sub) {
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
        _ => "UNKNOWN",
    })
}

/// Factory function to parse raw record bytes into a V4 record type
/// 
/// # Arguments
/// * `record_bytes` - Complete record including header (must be at least 4 bytes)
/// * `endian` - The endianness to use for parsing
/// 
/// # Returns
/// * `Ok(V4)` - Successfully parsed record
/// * `Err(Error)` - If the record cannot be parsed
/// 
/// # Example
/// ```no_run
/// use stdf::{StdfRecordIterator, stdf_parse_record};
/// 
/// let mut iter = StdfRecordIterator::new("test.std").unwrap();
/// let endian = iter.endian();
/// 
/// for result in iter {
///     let record_bytes = result.unwrap();
///     let record = stdf_parse_record(&record_bytes, endian).unwrap();
///     println!("{:?}", record);
/// }
/// ```
pub fn stdf_parse_record(record_bytes: &[u8], endian: byte::ctx::Endian) -> Result<V4<'_>, Error> {
    if record_bytes.len() < 4 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("Record too short: {} bytes (minimum 4)", record_bytes.len())
        ));
    }
    
    // Parse the record
    let offset = &mut 0;
    record_bytes.read_with::<V4>(offset, endian)
        .map_err(|e| Error::new(ErrorKind::Other, format!("Parse error: {:?}", e)))
}

/// Validates an STDF file by checking if the last record has a valid length.
/// Returns true if REC_LEN + 2 equals the actual record byte length, false otherwise.
pub fn valid_file(filename: &str) -> Result<bool, Error> {
    let iter = StdfRecordIterator::new(filename)?;
    let mut last_record: Option<Vec<u8>> = None;
    
    for result in iter {
        last_record = Some(result?);
    }
    
    match last_record {
        Some(record_bytes) => {
            if record_bytes.len() < 2 {
                return Ok(false);
            }
            
            // Get REC_LEN from first two bytes (endianness doesn't matter for validation)
            let rec_len = u16::from_le_bytes([record_bytes[0], record_bytes[1]]);
            
            // Check if REC_LEN + 2 equals the actual record length
            Ok((rec_len as usize + 2) == record_bytes.len())
        },
        None => Ok(false), // Empty file
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byte::ctx::Endian;

    #[test]
    fn parse_record_too_short() {
        let bytes = vec![0x00, 0x01]; // Only 2 bytes, need at least 4
        let result = stdf_parse_record(&bytes, Endian::Big);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert_eq!(err.kind(), ErrorKind::InvalidData);
        assert!(err.to_string().contains("too short"));
    }

    #[test]
    fn parse_record_empty() {
        let bytes = vec![];
        let result = stdf_parse_record(&bytes, Endian::Big);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn parse_record_valid_far() {
        // FAR: REC_LEN=2, REC_TYP=0, REC_SUB=10, cpu_type=2, stdf_ver=4
        let bytes = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
        let result = stdf_parse_record(&bytes, Endian::Big);
        assert!(result.is_ok());
        
        if let Ok(V4::FAR(far)) = result {
            assert_eq!(far.cpu_type.0, 2);
            assert_eq!(far.stdf_ver.0, 4);
        } else {
            panic!("Expected FAR record");
        }
    }

    #[test]
    fn parse_record_valid_pir() {
        // PIR: REC_LEN=2, REC_TYP=5, REC_SUB=10, head_num=1, site_num=2
        let bytes = vec![0x00, 0x02, 0x05, 0x0A, 0x01, 0x02];
        let result = stdf_parse_record(&bytes, Endian::Big);
        assert!(result.is_ok());
        
        if let Ok(V4::PIR(pir)) = result {
            assert_eq!(pir.head_num.0, 1);
            assert_eq!(pir.site_num.0, 2);
        } else {
            panic!("Expected PIR record");
        }
    }

    #[test]
    fn parse_record_respects_endianness() {
        // Little endian FAR
        let le_bytes = vec![0x02, 0x00, 0x00, 0x0A, 0x02, 0x04];
        let le_result = stdf_parse_record(&le_bytes, Endian::Little);
        assert!(le_result.is_ok());
        
        // Big endian FAR
        let be_bytes = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
        let be_result = stdf_parse_record(&be_bytes, Endian::Big);
        assert!(be_result.is_ok());
        
        // Both should parse to the same values
        if let (Ok(V4::FAR(le_far)), Ok(V4::FAR(be_far))) = (le_result, be_result) {
            assert_eq!(le_far.cpu_type.0, be_far.cpu_type.0);
            assert_eq!(le_far.stdf_ver.0, be_far.stdf_ver.0);
        } else {
            panic!("Expected FAR records");
        }
    }

    #[test]
    fn record_type_far() {
        let bytes = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
        let result = stdf_record_type(&bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "FAR");
    }

    #[test]
    fn stdf_record_fp_iterator_basic() {
        // Create a buffer with two records: FAR and PIR
        // FAR: REC_LEN=2, REC_TYP=0, REC_SUB=10, cpu_type=2, stdf_ver=4
        let far_bytes = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
        // PIR: REC_LEN=2, REC_TYP=5, REC_SUB=10, head_num=1, site_num=2
        let pir_bytes = vec![0x00, 0x02, 0x05, 0x0A, 0x01, 0x02];
        
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&far_bytes);
        buffer.extend_from_slice(&pir_bytes);
        
        let mut iter = StdfRecordFPIterator::from_buffer(buffer).expect("Failed to create iterator");
        
        // First record should be FAR at offset 0, ending at 6
        let first = iter.next().expect("Should have first record");
        assert!(first.is_ok());
        let (record1, fp1) = first.unwrap();
        assert_eq!(record1.len(), 6);
        assert_eq!(fp1, 6); // File pointer after FAR record
        assert_eq!(record1[2], 0x00); // REC_TYP = 0 (FAR)
        assert_eq!(record1[3], 0x0A); // REC_SUB = 10
        
        // Second record should be PIR at offset 6, ending at 12
        let second = iter.next().expect("Should have second record");
        assert!(second.is_ok());
        let (record2, fp2) = second.unwrap();
        assert_eq!(record2.len(), 6);
        assert_eq!(fp2, 12); // File pointer after PIR record
        assert_eq!(record2[2], 0x05); // REC_TYP = 5 (PIR)
        assert_eq!(record2[3], 0x0A); // REC_SUB = 10
        
        // Should be no more records
        assert!(iter.next().is_none());
    }

    #[test]
    fn stdf_record_fp_iterator_file_pointers_sequential() {
        // Create buffer with three different-sized records
        // FAR: 6 bytes total
        let far = vec![0x00, 0x02, 0x00, 0x0A, 0x02, 0x04];
        // PIR: 6 bytes total
        let pir = vec![0x00, 0x02, 0x05, 0x0A, 0x01, 0x02];
        // MRR: REC_LEN=8, REC_TYP=1, REC_SUB=20 + 8 data bytes = 12 bytes total
        let mrr = vec![0x00, 0x08, 0x01, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&far);
        buffer.extend_from_slice(&pir);
        buffer.extend_from_slice(&mrr);
        
        let mut iter = StdfRecordFPIterator::from_buffer(buffer).expect("Failed to create iterator");
        
        let mut last_fp = 0;
        let expected_fps = vec![6, 12, 24];
        let mut count = 0;
        
        for (idx, result) in iter.enumerate() {
            assert!(result.is_ok());
            let (_, fp) = result.unwrap();
            assert!(fp > last_fp, "File pointer should increase");
            assert_eq!(fp, expected_fps[idx], "File pointer mismatch at record {}", idx);
            last_fp = fp;
            count += 1;
        }
        
        assert_eq!(count, 3);
    }

    #[test]
    fn stdf_record_fp_iterator_endianness() {
        // Little endian FAR
        let le_far = vec![0x02, 0x00, 0x00, 0x0A, 0x02, 0x04];
        
        let mut iter = StdfRecordFPIterator::from_buffer(le_far.clone()).expect("Failed to create iterator");
        assert_eq!(iter.endian(), Endian::Little);
        
        let result = iter.next().expect("Should have record");
        assert!(result.is_ok());
        let (record, fp) = result.unwrap();
        assert_eq!(record.len(), 6);
        assert_eq!(fp, 6);
    }

    #[test]
    fn record_type_pir() {
        let bytes = vec![0x00, 0x02, 0x05, 0x0A, 0x01, 0x02];
        let result = stdf_record_type(&bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "PIR");
    }

    #[test]
    fn record_type_unknown() {
        let bytes = vec![0x00, 0x02, 0xFF, 0xFF, 0x00, 0x00];
        let result = stdf_record_type(&bytes);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "UNKNOWN");
    }

    #[test]
    fn record_type_too_short() {
        let bytes = vec![0x00, 0x02];
        let result = stdf_record_type(&bytes);
        assert!(result.is_err());
    }
}

/// Get the record type from an ATDF line (first 3 characters before the colon)
/// 
/// # Arguments
/// * `atdf_line` - A line from an ATDF file (e.g., "FAR:2|4|")
/// 
/// # Returns
/// * `Ok(&str)` - The record type (e.g., "FAR", "MIR", "PRR")
/// * `Err(Error)` - If the line doesn't contain a colon or is too short
/// 
/// # Example
/// ```
/// use stdf::parsers::atdf_record_type;
/// 
/// let line = "FAR:2|4|";
/// let record_type = atdf_record_type(line).unwrap();
/// assert_eq!(record_type, "FAR");
/// ```
pub fn atdf_record_type(atdf_line: &str) -> Result<&str, Error> {
    let colon_pos = atdf_line.find(':')
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "ATDF line missing colon separator"))?;
    
    if colon_pos == 0 {
        return Err(Error::new(ErrorKind::InvalidData, "ATDF line has empty record type"));
    }
    
    let record_type = &atdf_line[..colon_pos];
    
    // Validate it's 3 characters (standard STDF record type length)
    if record_type.len() != 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("ATDF record type '{}' is not 3 characters", record_type)
        ));
    }
    
    Ok(record_type)
}

/// Parse an ATDF line into a V4 STDF record
/// 
/// # Arguments
/// * `atdf_line` - A line from an ATDF file (e.g., "FAR:2|4|")
/// 
/// # Returns
/// * `Ok(V4)` - Successfully parsed STDF record
/// * `Err(Error)` - Parse error
/// 
/// # Example
/// ```no_run
/// use stdf::parsers::atdf_parse_record;
/// 
/// let line = "FAR:2|4|";
/// let record = atdf_parse_record(line).unwrap();
/// ```
pub fn atdf_parse_record(atdf_line: &str) -> Result<V4<'static>, Error> {
    let record_type = atdf_record_type(atdf_line)?;
    
    // Split the line into record type and fields
    let fields_part = &atdf_line[record_type.len() + 1..]; // Skip "TYPE:"
    let _fields: Vec<&str> = fields_part.split('|').collect();
    
    // TODO: Implement parsing logic for each record type
    // For now, return an error indicating it's not implemented
    Err(Error::new(
        ErrorKind::Other,
        format!("ATDF parsing for record type '{}' not yet implemented", record_type)
    ))
}

/// Streaming iterator that reads STDF records from a file without loading entire file into memory
/// This is more efficient for large files when you only need to read a few records from the beginning
pub struct StdfStreamingIterator {
    file: File,
    buffer: Vec<u8>,
    buffer_pos: usize,
    buffer_len: usize,
    endian: byte::ctx::Endian,
    eof_reached: bool,
}

impl StdfStreamingIterator {
    const BUFFER_SIZE: usize = 64 * 1024; // 64KB buffer
    
    /// Create a new streaming iterator from a file path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        
        // Read initial buffer to detect endianness
        let mut initial_buffer = vec![0u8; 16]; // Enough for FAR record
        let bytes_read = file.read(&mut initial_buffer)?;
        if bytes_read < 6 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "File too small to be valid STDF"));
        }
        
        let endian = Header::detect_endian(&initial_buffer)
            .map_err(|e| Error::new(ErrorKind::Other, format!("{:?}", e)))?;
        
        // Reset file to beginning
        file.seek(SeekFrom::Start(0))?;
        
        // Create larger buffer for streaming
        let mut buffer = vec![0u8; Self::BUFFER_SIZE];
        let buffer_len = file.read(&mut buffer)?;
        
        Ok(StdfStreamingIterator {
            file,
            buffer,
            buffer_pos: 0,
            buffer_len,
            endian,
            eof_reached: buffer_len == 0,
        })
    }
    
    /// Get the endianness of the STDF file
    pub fn endian(&self) -> byte::ctx::Endian {
        self.endian
    }
    
    /// Refill buffer from file
    fn refill_buffer(&mut self) -> Result<(), Error> {
        if self.eof_reached {
            return Ok(());
        }
        
        // Move remaining bytes to front of buffer
        if self.buffer_pos < self.buffer_len {
            let remaining = self.buffer_len - self.buffer_pos;
            self.buffer.copy_within(self.buffer_pos..self.buffer_len, 0);
            self.buffer_len = remaining;
            self.buffer_pos = 0;
        } else {
            self.buffer_len = 0;
            self.buffer_pos = 0;
        }
        
        // Read more data from file
        let bytes_read = self.file.read(&mut self.buffer[self.buffer_len..])?;
        self.buffer_len += bytes_read;
        
        if bytes_read == 0 {
            self.eof_reached = true;
        }
        
        Ok(())
    }
}

impl Iterator for StdfStreamingIterator {
    type Item = Result<Vec<u8>, Error>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer_pos >= self.buffer_len && self.eof_reached {
            return None;
        }
        
        // Ensure we have enough data for a header (at least 4 bytes)
        if self.buffer_len - self.buffer_pos < 4 {
            if let Err(e) = self.refill_buffer() {
                return Some(Err(e));
            }
            if self.buffer_pos >= self.buffer_len {
                return None;
            }
        }
        
        let start_pos = self.buffer_pos;
        
        // Read the header
        let header_result = self.buffer[..self.buffer_len]
            .read_with::<Header>(&mut self.buffer_pos, self.endian);
        
        match header_result {
            Ok(header) => {
                let record_length = header.rec_len.0 as usize;
                let header_size = self.buffer_pos - start_pos;
                let total_size = header_size + record_length;
                
                // Check if we need to refill buffer to get complete record
                if start_pos + total_size > self.buffer_len && !self.eof_reached {
                    self.buffer_pos = start_pos; // Reset position before refill
                    if let Err(e) = self.refill_buffer() {
                        return Some(Err(e));
                    }
                    
                    // After refill, start_pos is now 0, re-read header
                    self.buffer_pos = 0;
                    let header_result2 = self.buffer[..self.buffer_len]
                        .read_with::<Header>(&mut self.buffer_pos, self.endian);
                    
                    let header2 = match header_result2 {
                        Ok(h) => h,
                        Err(e) => {
                            return Some(Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Failed to re-read header after buffer refill: {:?}", e)
                            )));
                        }
                    };
                    
                    let record_length2 = header2.rec_len.0 as usize;
                    let header_size2 = self.buffer_pos;
                    let total_size2 = header_size2 + record_length2;
                    
                    // Check if we have enough data after refill
                    if total_size2 > self.buffer_len {
                        return Some(Err(Error::new(
                            ErrorKind::UnexpectedEof,
                            format!("Record too large for buffer: {} bytes", total_size2)
                        )));
                    }
                    
                    // Extract the complete record
                    let record_bytes = self.buffer[0..total_size2].to_vec();
                    self.buffer_pos = total_size2;
                    return Some(Ok(record_bytes));
                }
                
                // Check if we have enough data for complete record (no refill needed)
                if start_pos + total_size > self.buffer_len {
                    return Some(Err(Error::new(
                        ErrorKind::UnexpectedEof,
                        format!("Incomplete record at offset {}", start_pos)
                    )));
                }
                
                // Extract the complete record
                let record_bytes = self.buffer[start_pos..start_pos + total_size].to_vec();
                self.buffer_pos = start_pos + total_size;
                
                Some(Ok(record_bytes))
            },
            Err(byte::Error::BadOffset(_)) => {
                None  // End of file
            },
            Err(e) => Some(Err(Error::new(
                ErrorKind::Other,
                format!("Error reading header: {:?}", e)
            ))),
        }
    }
}
