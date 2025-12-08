#[macro_use]
extern crate stdf_record_derive;

pub mod types;
pub mod records;
pub mod parsers;
pub mod writers;

// Visualization modules
pub mod models;
pub mod statistics;
pub mod charts;
pub mod export;

// Re-export commonly used types
pub use records::{Header, V4};
pub use types::*;
pub use parsers::{
    StdfParser, StdfRecordIterator, StdfRecordFPIterator, AtdfRecordIterator,
    stdf_parse_record, stdf_record_type, 
    atdf_parse_record, atdf_record_type,
    valid_file, extract_part_records
};

/// Validates if a filename contains only safe characters that work across all platforms
/// (Windows, macOS, Linux).
///
/// Safe characters: A-Z, a-z, 0-9, -, _, ., (, ), [, ]
/// Does not allow filenames starting with `.` or `-`
///
/// # Arguments
/// * `filename` - The filename to validate (without path)
///
/// # Returns
/// * `true` if the filename is safe, `false` otherwise
///
/// # Example
/// ```
/// use stdf::is_complient_filename;
/// 
/// assert!(is_complient_filename("test_file.std"));
/// assert!(is_complient_filename("data-2024.std"));
/// assert!(!is_complient_filename(".hidden"));
/// assert!(!is_complient_filename("bad<file>.std"));
/// ```
pub fn is_complient_filename(filename: &str) -> bool {
    if filename.is_empty() {
        return false;
    }
    
    // Regex pattern: must start with alphanumeric or ( or [, followed by safe characters
    let re = regex::Regex::new(r"^[A-Za-z0-9()\[\]][-A-Za-z0-9_.()\[\]]*$").unwrap();
    
    // Must not start with . or -
    if filename.starts_with('.') || filename.starts_with('-') {
        return false;
    }
    
    re.is_match(filename)
}

// Python bindings (when feature is enabled)
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
/// Adds two numbers together (example function)
#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

#[cfg(feature = "python")]
/// Multiplies two numbers (example function)
#[pyfunction]
fn multiply(a: i64, b: i64) -> PyResult<i64> {
    Ok(a * b)
}

#[cfg(feature = "python")]
/// Python module for STDF
#[pymodule]
fn stdf(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_has_complient_filename_basic() {
        // Valid filenames
        assert!(is_complient_filename("test.std"));
        assert!(is_complient_filename("test_file.std"));
        assert!(is_complient_filename("data-2024.std"));
        assert!(is_complient_filename("file123.std"));
        assert!(is_complient_filename("Test_File-123.std"));
        assert!(is_complient_filename("file(1).std"));
        assert!(is_complient_filename("data[2024].std"));
        assert!(is_complient_filename("(test).std"));
        assert!(is_complient_filename("[test].std"));
        
        // Invalid filenames
        assert!(!is_complient_filename(""));
        assert!(!is_complient_filename(".hidden"));
        assert!(!is_complient_filename("-test.std"));
        assert!(!is_complient_filename("bad<file>.std"));
        assert!(!is_complient_filename("bad>file.std"));
        assert!(!is_complient_filename("bad:file.std"));
        assert!(!is_complient_filename("bad\"file.std"));
        assert!(!is_complient_filename("bad/file.std"));
        assert!(!is_complient_filename("bad\\file.std"));
        assert!(!is_complient_filename("bad|file.std"));
        assert!(!is_complient_filename("bad?file.std"));
        assert!(!is_complient_filename("bad*file.std"));
        assert!(!is_complient_filename("file with spaces.std"));
    }

    #[test]
    fn test_data_directory_filenames() {
        let data_dir = Path::new("data");
        
        if !data_dir.exists() {
            eprintln!("Warning: data directory does not exist, skipping test");
            return;
        }

        let mut valid_count = 0;
        let mut invalid_count = 0;
        let mut invalid_files = Vec::new();

        // Walk through all files in data directory recursively
        fn check_dir(dir: &Path, valid: &mut usize, invalid: &mut usize, invalid_list: &mut Vec<String>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        check_dir(&path, valid, invalid, invalid_list);
                    } else if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                        if is_complient_filename(filename) {
                            *valid += 1;
                        } else {
                            *invalid += 1;
                            invalid_list.push(path.display().to_string());
                        }
                    }
                }
            }
        }

        check_dir(data_dir, &mut valid_count, &mut invalid_count, &mut invalid_files);

        println!("\nFilename validation results:");
        println!("  Valid filenames: {}", valid_count);
        println!("  Invalid filenames: {}", invalid_count);
        
        if !invalid_files.is_empty() {
            println!("\nInvalid files found:");
            for file in &invalid_files {
                println!("  - {}", file);
            }
        }

        // Fail the test if any invalid filenames are found
        assert_eq!(
            invalid_count, 0,
            "Found {} invalid filename(s) in data directory. All filenames must contain only safe characters (A-Z, a-z, 0-9, -, _, ., (, ), [, ]) and must not start with . or -",
            invalid_count
        );
    }
}
