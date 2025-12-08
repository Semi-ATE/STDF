#[macro_use]
extern crate stdf_record_derive;

pub mod types;
pub mod records;
pub mod parsers;
pub mod writer;

// Re-export commonly used types
pub use records::{Header, V4};
pub use types::*;
pub use parsers::{
    StdfParser, StdfRecordIterator, StdfRecordFPIterator, AtdfRecordIterator,
    stdf_parse_record, stdf_record_type, 
    atdf_parse_record, atdf_record_type,
    valid_file, extract_part_records
};

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
