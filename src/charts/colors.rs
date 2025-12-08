// Color mapping for visualizations

use crate::models::TestLimits;

/// Map a value to a color based on position relative to median
/// Blue (low) → Green (median) → Orange (high)
pub fn value_to_color(value: f64, median: f64, limits: &TestLimits) -> (u8, u8, u8) {
    // Clamp value to spec limits
    let clamped = value.max(limits.lsl).min(limits.hsl);
    
    if clamped < median {
        // Below median: interpolate from blue to green
        let ratio = (clamped - limits.lsl) / (median - limits.lsl);
        let ratio = ratio.max(0.0).min(1.0);
        
        let r = (0.0 * (1.0 - ratio) + 0.0 * ratio) as u8;
        let g = (0.0 * (1.0 - ratio) + 255.0 * ratio) as u8;
        let b = (255.0 * (1.0 - ratio) + 0.0 * ratio) as u8;
        
        (r, g, b)
    } else {
        // Above median: interpolate from green to orange
        let ratio = (clamped - median) / (limits.hsl - median);
        let ratio = ratio.max(0.0).min(1.0);
        
        let r = (0.0 * (1.0 - ratio) + 255.0 * ratio) as u8;
        let g = (255.0 * (1.0 - ratio) + 165.0 * ratio) as u8;
        let b = (0.0 * (1.0 - ratio) + 0.0 * ratio) as u8;
        
        (r, g, b)
    }
}

/// Get background color for Cp/CpK cells based on thresholds
pub fn capability_cell_color(value: f64) -> Option<(u8, u8, u8)> {
    if value < 1.33 {
        Some((255, 0, 0))  // Bright red
    } else if value < 1.67 {
        Some((255, 165, 0))  // Bright orange
    } else {
        None  // No color (white)
    }
}
