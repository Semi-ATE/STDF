// Data models for STDF test data and visualization

/// Test limits defining pass/fail boundaries
#[derive(Debug, Clone)]
pub struct TestLimits {
    pub lsl: f64, // Lower Spec Limit (hard floor)
    pub ltl: f64, // Lower Test Limit (fail below)
    pub htl: f64, // High Test Limit (fail above)
    pub hsl: f64, // High Spec Limit (hard ceiling)
}

/// A single parametric measurement
#[derive(Debug, Clone)]
pub struct ParametricMeasurement {
    pub die_index: u32,
    pub x_coord: i16, // -32768 = no spatial data
    pub y_coord: i16, // -32768 = no spatial data
    pub value: f64,
    pub site: u8,
}

/// A single functional test result
#[derive(Debug, Clone)]
pub struct FunctionalMeasurement {
    pub die_index: u32,
    pub x_coord: i16,
    pub y_coord: i16,
    pub passed: bool,
    pub site: u8,
}

/// Parametric test data with all measurements
#[derive(Debug)]
pub struct ParametricTest {
    pub name: String,
    pub unit: String,
    pub limits: TestLimits,
    pub measurements: Vec<ParametricMeasurement>,
    pub num_sites: u8,
}

/// Functional test data with all results
#[derive(Debug)]
pub struct FunctionalTest {
    pub name: String,
    pub measurements: Vec<FunctionalMeasurement>,
    pub num_sites: u8,
}

/// Complete test report for a wafer/lot
#[derive(Debug)]
pub struct TestReport {
    pub lot_id: String,
    pub wafer: Option<u8>,
    pub parametric_tests: Vec<ParametricTest>,
    pub functional_tests: Vec<FunctionalTest>,
}

/// Coordinate constant for invalid/missing spatial data
pub const INVALID_COORD: i16 = -32768;

/// Check if measurements have valid spatial data
pub fn has_spatial_data(x: i16, y: i16) -> bool {
    x != INVALID_COORD && y != INVALID_COORD
}
