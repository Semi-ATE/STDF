# stdf-export Design Document

## Project Overview

A Rust library and CLI tool for generating PDF and Excel reports from semiconductor test data in STDF (Standard Test Data Format) format.

**Key Goals:**
- Visual pattern recognition in wafer test data
- Dual output: PDF (quick analysis) + Excel (deep dive)
- Support both wafer sort and final test data
- Handle multi-site parallel testing (4, 8, 16, 32, 64+ sites)

---

## Data Flow

```
STDF Parser (separate library)
  ↓
Vector of test records per die
  ↓
stdf-export (this library)
  ↓
PDF Report + Excel Spreadsheet
```

### Input Data Structure

**Per Die:**
- PIR (Part Information Record) - site info
- PTR/FTR sequence (n parametric + m functional tests)
- PRR (Part Result Record) - overall pass/fail + X/Y coordinates

**Key Fields:**
- `X_COORD`, `Y_COORD`: i16, -32768 = no spatial data (final test)
- `SITE_NUM`: Which parallel test site (S1, S2, S3, S4, etc.)
- `HARD_BIN`, `SOFT_BIN`: Bin assignments
- Test values and limits (LSL, LTL, HTL, HSL)

---

## Test Types

### Parametric Tests (PTR)
- Measured values with units (mΩ, mA, V, ns, etc.)
- Has test limits and statistical analysis
- Generates full visualization

### Functional Tests (FTR)
- Binary pass/fail results
- No statistics (can't calculate mean/σ on pass/fail)
- Simplified visualization

---

## Test Limits Structure

```
HSL ─── High Spec Limit (hard ceiling, clip values above)
 ↕  1/12 of total range
HTL ─── High Test Limit (fail if above)
 ↕  10/12 of total range (PASS ZONE with 10 grid divisions)
LTL ─── Lower Test Limit (fail if below)
 ↕  1/12 of total range
LSL ─── Lower Spec Limit (hard floor, clip values below)
```

**Vertical Scaling:**
- Total height = 12 parts
- Bottom margin: LSL to LTL = 1/12
- Pass zone: LTL to HTL = 10/12 (divided into 10 equal grid lines)
- Top margin: HTL to HSL = 1/12

**Pass/Fail Logic:**
- Pass: `LTL ≤ value ≤ HTL`
- Fail: `value < LTL OR value > HTL`

---

## Multi-Site Testing

**Concept:**
- Tester has multiple probe heads (sites) testing die in parallel
- Each test cycle produces N measurements simultaneously (one per site)
- Example: 4-site tester → die 0,1,2,3 tested together, then 4,5,6,7, etc.

**Key Principles:**
1. **Site info only in statistics table** - don't color-code by site in charts
2. **Abstraction in visualizations** - wafer map and trend don't show site
3. **Scalability** - support 4 to 64+ sites without cluttering visuals

**Rationale:** With 64 sites, the human eye can't distinguish 64 colors!

---

## Color Schemes

### Gradient for Parametric Tests

**Philosophy:** Diverging color scale centered on median
- **Median = optimal** (green)
- **Deviations from median** = less desirable
- **Blue = below median** (cooler/lower)
- **Orange = above median** (hotter/higher)

**Color Mapping:**
```
HSL/High values:  [Orange/Red]
                  [Yellow]
                  [Light Green]
Median:           [Bright Green] ← CENTER
                  [Cyan/Turquoise]
                  [Blue]
LSL/Low values:   [Dark Blue]
```

**Benefits for Pattern Recognition:**
- Symmetric around actual process median
- Intuitive hot/cold metaphor
- Spatial patterns become obvious
- Works regardless of where median falls in pass zone

### Cell Colors for Process Capability

**Cp, CpKL, CpKH thresholds:**
- **value ≥ 1.67** → No color (white background) - good
- **1.33 ≤ value < 1.67** → Bright orange background - marginal
- **value < 1.33** → Bright red background - poor

**Other metrics:** No coloring (Pass, Fail, Min, Max, Median, Range, σ)

### Functional Test Colors

Simple binary:
- **Pass** → Green
- **Fail** → Red

---

## PDF Report Structure

### Page 1: Summary/Index Table

**Layout:**
```
Lot: 123456.000  Wafer: 08
Yield: 26 / 3724 = 99.30%

┌────┬─────────────────────┬────────────────┬─────────────────┐
│ #  │ Test Name           │ CoK (S1-S4)    │ Fails (S1-S4+TTL)│
├────┼─────────────────────┼────────────────┼─────────────────┤
│ 12 │ BVDSS_3_Vx         │ 1.89 4.91 ...  │ 6               │
│  4 │ DIVID_1_t_Vx       │ 8.22 3.23 ...  │ 4 (orange cell) │
│  2 │ CONTACT_TEST...    │ 1.95 1.96 ...  │ 1.93            │
└────┴─────────────────────┴────────────────┴─────────────────┘
```

**Features:**
- Test names are **hyperlinks** → jump to detailed chart page
- Color coding on problematic cells (orange/red)
- Shows pass counts per site (CoK columns)
- Shows fail counts per site + total (TTL)

### Pages 2 to n+m+1: Individual Test Charts

**One page per test** with full visualization

---

## Parametric Test Visualization

**Layout (4 panels):**

```
┌─────────────────────────────────────────────────────────────┐
│ Title: Lot 123456.000 Wafer 08 Test: CONTACT_TEST_...      │
│                                                              │
│  ┌────┬──┬─────────────────────────────────────────────┐   │
│  │Hist│GR│  Trend Chart (die index vs value)          │   │
│  │90° │AD│  - Gray/green line                          │   │
│  │    │  │  - Reference lines: HSL, HTL, ±3σ, LTL, LSL│   │
│  └────┴──┴─────────────────────────────────────────────┘   │
│  ┌─────────────────────┬──────────────────────────────┐   │
│  │ Wafer Map           │ Statistics Table             │   │
│  │ (colored by value)  │ Site  S1   S2   S3   S4  TTL │   │
│  │                     │ Pass  931  930  929  928 3718│   │
│  │ Die at X,Y position │ Fail  0    1    2    3   6   │   │
│  │ Color from gradient │ Min   9201 ...               │   │
│  │ Circular pattern    │ Cp    8.15 8.16 ...          │   │
│  │                     │ CpKL  1.95 1.25 (red)  ...   │   │
│  └─────────────────────┴──────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

**Top Section - Left to Right:**
1. **Histogram (rotated 90° CCW)**
   - Shows value distribution
   - Colored bars matching gradient
   - Vertical orientation

2. **Gradient Bar**
   - Color scale legend
   - LSL to HSL range
   - Shows unit at top (e.g., "mΩ")

3. **Trend Chart**
   - X-axis: Die index (0 to total count)
   - Y-axis: Value (LSL to HSL with 1-10-1 scaling)
   - Single line (all sites combined)
   - Reference lines:
     - Red dashed: HSL, HTL, LTL, LSL
     - Green dashed: Median, ±3σ
   - 10 horizontal grid lines in pass zone (LTL to HTL)

**Bottom Section:**
1. **Wafer Map (Left)**
   - Only if spatial data exists (X,Y ≠ -32768)
   - Each tested die = colored rectangle at (X, Y)
   - Color from gradient based on value
   - Untested positions = transparent
   - Natural circular pattern from die coordinates
   - No grid, just colored die
   - Die size auto-scaled to make overall shape circular

2. **Statistics Table (Right)**
   - Columns: S1, S2, S3, S4, ..., SN, TTL
   - Rows: Pass, Fail, TTL, Min, Median, Max, Range, σ, Cp, CpKL, CpKH
   - Color coding on Cp, CpKL, CpKH cells (orange/red for poor values)

---

## Functional Test Visualization

**Simplified layout** (no histogram, gradient, or trend needed):

```
┌─────────────────────────────────────────────────────────────┐
│ Title: Lot 123456.000 Wafer 08 Test: CONTINUITY_CHECK      │
│                                                              │
│  ┌─────────────────────┬──────────────────────────────┐    │
│  │ Wafer Map           │ Summary Table                │    │
│  │ (Green=Pass         │ Site    Pass   Fail   Yield  │    │
│  │  Red=Fail)          │ S1      931    0      100%   │    │
│  │                     │ S2      930    1      99.89% │    │
│  │                     │ S3      929    2      99.78% │    │
│  │                     │ S4      928    3      99.68% │    │
│  │                     │ TTL     3718   6      99.84% │    │
│  └─────────────────────┴──────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

**If no spatial data (final test):**
- Only show summary table (centered or full width)
- No wafer map section

---

## Excel Spreadsheet Structure

### Sheet 1: Summary Table
Same layout as PDF page 1 (without hyperlinks)

### Sheet 2: Wide Data Matrix

**All test data in one sheet, wide format:**

```
┌──────┬────┬────┬──────┬──────┬────────┬────────┬──────────────────┬──────────────────┬─────┐
│ Die# │ X  │ Y  │ Head │ Site │HardBin │SoftBin │  PTR Test 1      │  PTR Test 2      │ FTR │
│      │    │    │      │      │        │        │ LTL│Value│HTL    │ LTL│Value│HTL    │Test │
├──────┼────┼────┼──────┼──────┼────────┼────────┼────┼─────┼───────┼────┼─────┼───────┼─────┤
│  0   │ 10 │ 15 │  1   │  1   │   1    │   1    │  0 │11955│18840  │  5 │ 9.2 │  15   │  P  │
│  1   │ 11 │ 15 │  1   │  2   │   1    │   1    │  0 │11960│18840  │  5 │ 9.3 │  15   │  P  │
│  2   │ 12 │ 15 │  1   │  3   │   1    │   1    │  0 │19000│18840  │  5 │14.2 │  15   │  F  │
│      │    │    │      │      │        │        │    │ RED │       │    │     │       │ RED │
└──────┴────┴────┴──────┴──────┴────────┴────────┴────┴─────┴───────┴────┴─────┴───────┴─────┘
```

**Column Groups:**

1. **Fixed Columns (from PRR):**
   - Die Index (sequential)
   - X, Y coordinates
   - Test Head number
   - Test Site number
   - Hard Bin
   - Soft Bin
   - (other PRR fields as needed)

2. **Per Parametric Test (3 columns each):**
   - LTL (black text, constant value)
   - Measurement Value (black if pass, **RED if fail**)
   - HTL (black text, constant value)

3. **Per Functional Test (1 column each):**
   - "P" (black) or "F" (**RED**)

**Formatting:**
- Freeze panes on header row + fixed columns
- Color only on fail values (RED text)
- No charts in Excel - users do their own analysis

**Benefits:**
- One row per die, all tests visible horizontally
- Easy to filter, sort, pivot
- Standard semiconductor data format
- Smaller files (no embedded images)

---

## Wafer Map Details

**Geometry:**
- Input: (X, Y) coordinates per die (i16, signed)
- X = horizontal, Y = vertical (standard Cartesian)
- Find X range and Y range across all measurements
- Scale die dimensions so overall shape is circular
- Only render tested die (others transparent)

**Algorithm:**
```rust
let x_range = x_max - x_min;
let y_range = y_max - y_min;
let scale = render_size / max(x_range, y_range);
let die_width = scale;
let die_height = scale;

for each measurement:
    draw rectangle at (x * scale, y * scale)
    with color from gradient(value, median, limits)
```

**Result:** Natural circular pattern from wafer edge

---

## Statistical Calculations

### Basic Statistics
- Count, Pass count, Fail count
- Min, Max, Range
- Mean (average)
- Median (middle value)
- Standard Deviation (σ)

### Process Capability Metrics

**Cp (Process Capability):**
```
Cp = (HTL - LTL) / (6σ)
```
Measures if process spread fits within spec limits

**CpKL (Lower Capability Index):**
```
CpKL = (Mean - LTL) / (3σ)
```
Measures distance to lower limit

**CpKH (Upper Capability Index):**
```
CpKH = (HTL - Mean) / (3σ)
```
Measures distance to upper limit

**Interpretation:**
- Cp/CpK ≥ 1.67: Excellent capability
- Cp/CpK ≥ 1.33: Adequate capability  
- Cp/CpK < 1.33: Poor capability (red flag)

---

## Chart Generation

**Library:** plotters

**Output Formats:**
- **SVG** for PDF embedding (vector, scalable)
- **PNG** for preview/fallback (raster)

**Chart Dimensions:**
- Full page for single chart
- Maintain aspect ratio
- High DPI for print quality

---

## Data Interface

**From STDF Parser to this library:**

Options discussed:
1. **Library API** - parser calls functions in this crate
2. **Intermediate file** - JSON/other format between projects
3. **Embedded** - this code as dependency in parser

**Data flow per test:**
- Parser supplies all measurements for one test
- This library accumulates across all die
- Generates one chart/sheet per test
- Final output: complete report with n+m tests

---

## Future Enhancements

### ODS Support
Currently targets .xlsx, but designed with abstraction for future .ods (OpenDocument Spreadsheet) support:
- Writer trait for format abstraction
- Format detection from file extension or CLI flag
- Same data model, different writer implementation

### Additional Features (Potential)
- Bin pareto charts
- Correlation analysis between tests
- Trend analysis across wafers/lots
- Interactive HTML reports
- Real-time streaming mode

---

## Design Principles

1. **Pattern Recognition First** - Visual design optimized for spotting issues
2. **Scale Agnostic** - Works with 4 or 64 parallel sites
3. **Format Flexible** - Support wafer sort and final test data
4. **Dual Output** - PDF for quick review, Excel for deep analysis
5. **Clean Abstractions** - Writer trait, color mapping, statistics module
6. **Professional Quality** - Production-ready reports for semiconductor industry

---

## References

- STDF v4 Specification: `doc/STDF-v4-spec.pdf`
- Example visualization: See wafer test chart image in `doc/`
- Summary table example: See summary page image in `doc/`
