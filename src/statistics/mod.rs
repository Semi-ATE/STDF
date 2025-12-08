// Statistical calculations for test data

use crate::models::{ParametricMeasurement, TestLimits};

/// Statistics for a set of measurements
#[derive(Debug, Clone)]
pub struct Statistics {
    pub count: usize,
    pub pass_count: usize,
    pub fail_count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub range: f64,
    pub cp: f64,
    pub cpk_lower: f64,
    pub cpk_upper: f64,
}

/// Calculate statistics for a set of parametric measurements
pub fn calculate_statistics(
    measurements: &[ParametricMeasurement],
    limits: &TestLimits,
) -> Statistics {
    if measurements.is_empty() {
        return Statistics::default();
    }

    let values: Vec<f64> = measurements.iter().map(|m| m.value).collect();

    let count = values.len();
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = values.iter().sum::<f64>() / count as f64;

    let mut sorted = values.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if count % 2 == 0 {
        (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
    } else {
        sorted[count / 2]
    };

    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
    let std_dev = variance.sqrt();

    let range = max - min;

    // Pass/Fail counts
    let pass_count = measurements
        .iter()
        .filter(|m| m.value >= limits.ltl && m.value <= limits.htl)
        .count();
    let fail_count = count - pass_count;

    // Process capability
    let cp = (limits.htl - limits.ltl) / (6.0 * std_dev);
    let cpk_lower = (mean - limits.ltl) / (3.0 * std_dev);
    let cpk_upper = (limits.htl - mean) / (3.0 * std_dev);

    Statistics {
        count,
        pass_count,
        fail_count,
        min,
        max,
        mean,
        median,
        std_dev,
        range,
        cp,
        cpk_lower,
        cpk_upper,
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Statistics {
            count: 0,
            pass_count: 0,
            fail_count: 0,
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            range: 0.0,
            cp: 0.0,
            cpk_lower: 0.0,
            cpk_upper: 0.0,
        }
    }
}
