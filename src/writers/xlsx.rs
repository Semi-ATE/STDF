// Excel file generation

use anyhow::Result;

pub struct XlsxWriter {
    output_path: String,
}

impl XlsxWriter {
    pub fn new(output_path: String) -> Self {
        XlsxWriter { output_path }
    }
    
    pub fn write(&mut self) -> Result<()> {
        // TODO: Implement Excel generation
        // Sheet 1: Summary table
        // Sheet 2: Wide data matrix (all tests as columns)
        Ok(())
    }
}
