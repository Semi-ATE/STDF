// Output writers for multiple formats

pub mod stdf;   // Existing STDF binary writer
pub mod pdf;    // PDF report writer
pub mod xlsx;   // Excel spreadsheet writer

use anyhow::Result;
use crate::models::TestReport;

/// Writer trait for abstracting different output formats
pub trait ReportWriter {
    fn write_report(&mut self, report: &TestReport) -> Result<()>;
}
