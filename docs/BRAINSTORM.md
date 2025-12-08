# Brainstorming Session Notes - December 7, 2025

## Project Genesis

This document captures the complete brainstorming session that led to the creation of the stdf-export project.

## Initial Concept

Started with: "Rust project that writes an .xlsx file"

Evolved into: **Dual-format (PDF + Excel) semiconductor test report generator**

## Key Discussion Points

### 1. Output Formats
- **Initial**: Just .xlsx
- **Considered**: Adding .ods (OpenDocument) support
- **Decision**: Start with .xlsx, design with abstraction for future .ods
- **Final**: Both PDF and Excel simultaneously
  - PDF: Quick visual analysis with hyperlinks
  - Excel: Deep data analysis with wide matrix format

### 2. Chart Types Needed
- Multiple sheets with colored data
- Red/Orange/Green font colors for status
- **Challenge**: Excel's Python support vs embedded charts
- **Decision**: Generate charts as images (PNG/SVG) in Rust using plotters
  - PDF: Embed SVG charts (vector quality)
  - Excel: NO charts, just data tables (users do their own analysis)

### 3. Color Gradient Evolution
- **Original image**: Blue→Cyan→Green→Yellow→Orange→Red (low to high)
- **Problem**: Doesn't highlight optimal median value
- **Discussion**: Should "good" be green at median?
- **Final Decision**: Diverging color scale centered on median
  - Below median: Blue (cooler/lower)
  - At median: Green (optimal)
  - Above median: Orange (hotter/higher)
  - Purpose: Pattern recognition in wafer maps

### 4. Layout Discovery
- **Initial misunderstanding**: Bottom-left was histogram
- **Correction**: Bottom-left is wafer map!
- **Final 4-panel layout**:
  - Top: Histogram (90° rotated) | Gradient | Trend Chart
  - Bottom: Wafer Map | Statistics Table

### 5. Wafer Map Geometry
- Wafers are round (may have flat edge, ignored)
- Each die has X/Y coordinates (i16, signed)
- X = horizontal, Y = vertical
- Scale die sizes to make overall shape circular
- Only render tested die (others transparent)
- No grid, just colored rectangles

### 6. Multi-Site Testing Insight
- **Key realization**: Testers probe 4-64 die in parallel
- Sites = S1, S2, S3, S4... (parallel probe heads)
- **Critical design decision**: Don't color-code by site in visualizations
  - With 64 sites, human eye can't distinguish 64 colors!
  - Site breakdown only in statistics table
- Make abstraction of site in wafer map and trend

### 7. Test Types
- **PTR (Parametric Test Record)**: Measured values + limits
  - Full 4-panel visualization
  - Statistics: Cp, CpK, mean, median, σ
- **FTR (Functional Test Record)**: Pass/Fail only
  - Simplified: Wafer map + summary table
  - No histogram, gradient, or trend (can't calculate stats on binary)

### 8. Test Limits Scaling
- **Key insight**: Vertical scaling is proportional, not linear!
- LSL to LTL: 1/12 of total height
- LTL to HTL: 10/12 (pass zone with 10 grid divisions)
- HTL to HSL: 1/12
- Grid lines: 10 equal divisions between LTL and HTL
- Values clipped to LSL/HSL for visualization

### 9. Coordinate Handling
- Valid coordinates: any i16 value
- Invalid (no spatial data): X = Y = -32768
- **Significance**: Distinguishes wafer sort from final test
- If invalid coords: omit wafer map entirely

### 10. PDF Report Structure
- **Page 1**: Summary table with test list
  - Shows pass/fail counts per site
  - Color coding (orange/red) for problematic tests
  - **Hyperlinks**: Click test name → jump to detail page
  - Lot ID, wafer number, overall yield
- **Pages 2-n**: One chart per test (PTR or FTR)
  - Full visualization with embedded SVG

### 11. Excel Report Structure
- **Sheet 1**: Summary table (same as PDF page 1, no hyperlinks)
- **Sheet 2**: Wide data matrix - ONE row per die, ALL tests as columns
  - Fixed columns: Die#, X, Y, Head, Site, HardBin, SoftBin
  - Per PTR test: 3 columns (LTL | Value | HTL)
  - Per FTR test: 1 column (P or F)
  - Color: RED text for fails only
  - Freeze panes on headers + fixed columns
  - No charts! Users analyze with Excel tools

### 12. Process Capability Color Rules
- **Cp, CpKL, CpKH** get cell background colors:
  - value ≥ 1.67: No color (white) - good
  - 1.33 ≤ value < 1.67: Bright orange - marginal
  - value < 1.33: Bright red - poor
- Other metrics: no coloring

### 13. Data Interface Decision
- STDF parser is separate library (your other project)
- This library consumes parsed data
- **Not decided yet**: API vs intermediate file format
- Parser supplies measurements for each test
- One chart/sheet generated per test
- Total output: n PTR charts + m FTR charts

## Important Technical Details

### Color Gradients
```rust
// Diverging scale centered on median
Below median: interpolate Blue → Green
At median: Bright Green
Above median: interpolate Green → Orange

// Cell coloring for Cp/CpK
< 1.33: RGB(255, 0, 0) - red
< 1.67: RGB(255, 165, 0) - orange
≥ 1.67: No color
```

### Statistics Formulas
```
Cp = (HTL - LTL) / (6σ)
CpKL = (Mean - LTL) / (3σ)
CpKH = (HTL - Mean) / (3σ)
```

### Wafer Map Algorithm
```rust
x_range = x_max - x_min;
y_range = y_max - y_min;
scale = render_size / max(x_range, y_range);
die_width = scale;
die_height = scale;

for each measurement:
    if has_spatial_data(x, y):
        draw_rect(x * scale, y * scale, die_width, die_height, color_from_gradient(value))
```

## Design Philosophy

1. **Pattern Recognition First**: Visualizations optimized for spotting issues
2. **Scale Agnostic**: Works with any number of parallel sites
3. **Format Flexible**: Handles wafer sort and final test data
4. **Dual Output**: PDF for quick review, Excel for deep analysis
5. **No User Interaction Needed**: Generate complete reports automatically
6. **Professional Quality**: Production-ready for semiconductor industry

## Decisions Timeline

1. ✅ Rust + .xlsx → Start here
2. ✅ Add PDF output → Dual format
3. ✅ Keep door open for .ods → Writer trait abstraction  
4. ✅ Generate charts in Rust → Plotters library
5. ✅ Skip Excel Python integration → Not needed
6. ✅ Skip VBA macros → Not needed
7. ✅ Embed images in PDF, data-only in Excel → Simpler
8. ✅ Diverging color gradient → Better pattern recognition
9. ✅ Site abstraction in visuals → Handles 64+ sites
10. ✅ Summary page with hyperlinks → Better navigation

## Questions Still Open

1. **Data interface format**: How will STDF parser communicate with this library?
   - Library API (function calls)
   - Intermediate file (JSON/other)
   - Embedded (as dependency)

2. **PRR fields**: Which PRR fields to include in Excel fixed columns?
   - Currently planned: Die#, X, Y, Head, Site, HardBin, SoftBin
   - May need: TEST_T, PART_FLG, others?

3. **Chart dimensions**: Exact pixel sizes for different outputs?

4. **Hyperlink implementation**: PDF library support for internal links?

## Next Steps (For Future Sessions)

**Suggested Implementation Order:**

1. **Phase 1: Core Infrastructure**
   - Refine data models based on actual STDF fields
   - Implement statistics calculations
   - Test with sample data

2. **Phase 2: Chart Generation**
   - Color mapping functions
   - Wafer map renderer (plotters)
   - Trend chart with reference lines
   - Histogram (rotated)
   - Gradient bar

3. **Phase 3: Excel Writer**
   - Summary sheet
   - Wide data matrix with proper formatting
   - Cell coloring (RED text for fails, colored backgrounds for Cp/CpK)
   - Freeze panes

4. **Phase 4: PDF Writer**
   - Summary page with test list
   - Embed SVG charts
   - Internal hyperlinks
   - Page layout

5. **Phase 5: Integration & Testing**
   - Connect with STDF parser
   - Test with real data
   - Performance optimization
   - Error handling

## Files Added to Project

- `doc/STDF-v4-spec.pdf` - Official STDF specification
- `doc/STDF-v4-2007-spec.pdf` - Updated spec
- `doc/ATDF-spec.pdf` - ASCII Test Data Format spec
- `doc/TestPerWafer.png` - Example wafer test chart (the image we analyzed)
- `doc/SummaryPage.pdf` - Example summary page
- `doc/tables.xlsx` - Example table formats
- `doc/autoreports.pptx` - Additional examples
- `doc/Capture.PNG`, `home.png`, `ProbingTop.xcf` - Reference images
- `doc/DESIGN.md` - Complete design specification

## Key Insights Learned

1. **Semiconductor testing is complex**: Multi-site, wafer maps, statistical process control
2. **Color is critical**: Not just aesthetic, essential for pattern recognition
3. **Dual outputs serve different needs**: PDF for sharing/review, Excel for analysis
4. **Abstraction matters**: Can't hard-code for 4 sites when testers have 64+
5. **Visualization > Data**: Charts catch problems that tables miss
6. **Industry standards**: STDF format is universal in semiconductor testing

## Quotes from Discussion

> "If we test for example 64 dies in parallel, how is the human eye going to distinguish 64 different colors ?!?"

> "The goal is to recognize patterns in the wafermap ;-)"

> "For further investigation we would need the xlsx file ... let's start with a bright red, and bright orange, we can still adjust later ;-)"

> "Yes, you can check everything in to git also afterwards"

---

**Session Duration**: ~3 hours of brainstorming and design
**Date**: December 7, 2025
**Outcome**: Complete project structure, comprehensive design document, clear implementation roadmap

This was an excellent brainstorming session that evolved from a simple "write xlsx file" to a sophisticated semiconductor test report generator!
