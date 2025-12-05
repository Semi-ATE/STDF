// Common test utilities

use std::path::Path;

/// Get path to test data directory
pub fn test_data_dir() -> &'static str {
    r".\data\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std\diamond28_1_DMHACF91MV0LJM1878A_243_F2N_H_325165058_00_16062023_234311.std"
}

/// Check if test data file exists
pub fn test_data_exists() -> bool {
    Path::new(test_data_dir()).exists()
}
