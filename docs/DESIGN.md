# STDF - Design Document

## Project Overview

**Semi-ATE-STDF** is a comprehensive Rust-based toolkit for semiconductor test data in STDF (Standard Test Data Format). It combines high-performance parsing with advanced visualization and reporting capabilities.

### Naming Convention
- **Rust Library**: `stdf-lib` (crate name: `stdf_lib`)
- **CLI Binary**: `stdf` (outputs as `stdf.exe`)
- **Python Module**: `import stdf` (PyO3 bindings expose as `stdf` for Python users)

The library is named `stdf-lib` to avoid Cargo filename collisions with the `stdf` binary while maintaining a clean interface for both CLI and Python users.

**Key Features:**
- Maximum performance STDF parser (memory-mapped I/O, ~900 MB/s)
- PDF report generation with embedded charts
- Excel export with rich formatting
- Python bindings for data analysis workflows
- CLI tools for STDF file operations
- Multi-file lot-level analysis with Parquet caching

---

# Part 1: STDF Parser

## Architecture

### Core Components

#### 1. **lib.rs** - Library Root
- Main entry point for the library
- Exports public API: `StdfParser`, `StdfRecordIterator`, record types
- Python bindings via PyO3 (feature-gated)
- Module organization:
  - `types`: STDF data types (U1, I2, C*n, etc.)
  - `records`: STDF V4 record definitions
  - `parsers`: File parsing logic
  - `writers`: Multi-format output (STDF, PDF, Excel, Parquet)
  - `models`: Data structures for visualization
  - `statistics`: Statistical calculations (Cp, CpK)
  - `charts`: Chart generation with plotters

#### 2. **records.rs** - Record Definitions
- Defines all STDF V4 record types as Rust structs
- `V4` enum: Tagged union of all possible record types
- Uses procedural macro `#[derive(StdfRecord)]` for automatic parsing
- Each record implements:
  - Binary parsing from byte streams
  - ASCII (ATDF) conversion via `Display` trait
  - Field-level access

#### 3. **parsers.rs** - Parsing Logic

**Iterator Implementations:**
- `StdfRecordIterator`: Memory-mapped iterator (loads entire file into memory)
  - **Performance**: ~906 MB/s on 3GB files
  - **Best for**: Unlimited queries, full-file scans, multiple passes
  - **Memory**: Maps entire file into virtual memory
  
- `StdfRecordFPIterator`: Memory-mapped iterator with file pointers
  - Returns tuple of (bytes, offset) for each record
  - **Performance**: Identical to `StdfRecordIterator` (zero overhead)
  - **Recommendation**: Use for all new code needing offsets
  
- `StdfStreamingIterator`: Progressive file reader (NEW)
  - **Performance**: 25ms for early records (FAR, MIR), 40ms for limited queries
  - **Best for**: Early records, small limits (< 1000), large files
  - **Memory**: 64KB buffer, minimal memory footprint
  - **Strategy**: Reads file progressively, exits early when possible

**Parsing Functions:**
- `stdf_parse_record()`: Parses individual records from byte slices
- `valid_file()`: Validates STDF file structure (starts with FAR, ends with MRR)
- Handles both little-endian and big-endian files
- Uses `byte` crate for efficient binary parsing

**Performance Optimization Strategy:**

The `show_field()` function intelligently routes queries based on record type and limit:

1. **Early records** (FAR, ATR, MIR, RDR, SDR, WIR): Always use streaming
   - These appear at file beginning, streaming finds them in ~25ms
   
2. **Late records** (MRR, PCR, HBR, SBR, WRR, TSR): Always use memory-mapped
   - These appear at file end, memory-mapping is more efficient
   
3. **Middle records** (PMR, PGR, PLR, WCR, PIR, PRR, PTR, etc.): Adaptive
   - Limit < 1000: Use streaming (40ms vs 1300ms for memory-mapped)
   - No limit or limit ≥ 1000: Use memory-mapped (better for full scans)

4. **Singleton records** (FAR, ATR, MIR, MRR, PCR, HBR, SBR): Auto-exit after first match
   - These only occur once per file, no need to continue scanning

**Early Exit Optimizations:**
- Record type filtering: Check bytes[2:3] before parsing to skip unwanted records
- Limit checking: Break immediately when requested count is reached
- Singleton detection: Automatic early exit for records that only occur once

**Benchmark Results (Native Windows, 35MB file):**
- FAR STDF_VER (streaming, singleton): 287ms
- MIR LOT_ID (streaming, singleton): 25ms (3rd record in file)
- PRR SITE_NUM -10 (streaming, limited): 40ms
- PRR SITE_NUM unlimited (memory-mapped): 1,285ms

**Note on WSL Performance:**
Testing showed ~4 second overhead when accessing Windows NTFS files from WSL due to cross-system file I/O. Native Windows or Linux shows true performance benefits.

#### 4. **types.rs** - STDF Data Types
- Implements STDF primitive types:
  - `U1`, `U2`, `U4`, `U8`: Unsigned integers
  - `I1`, `I2`, `I4`, `I8`: Signed integers
  - `R4`, `R8`: Floating point
  - `Cn`: Fixed-length strings
  - `Bn`: Binary data
  - Arrays and variable-length types
- Each type handles endianness conversion
- ATDF conversion (escaping special characters)

#### 5. **stdf-record-derive/** - Procedural Macro
- Generates parsing code for all record types
- Implements `Display` trait for ATDF conversion
- Generates `ascii()` method returning pipe-delimited fields

### Data Flow (Parsing)

```
STDF File (binary)
    ↓
Iterator Selection (based on query type)
    ├─→ StdfStreamingIterator (early records, small limits)
    │   └─→ 64KB buffer, progressive reading, early exit
    │
    └─→ StdfRecordIterator (late records, unlimited queries)
        └─→ Memory-mapped, full file access
    ↓
Record Filtering (check bytes[2:3] for type match)
    ↓
stdf_parse_record() (parse matched records only)
    ↓
V4 enum (typed records)
    ↓
Field Extraction
    ↓
Early Exit (limit reached or singleton found)
    ↓
Output (stdout, files, exit codes)
```

### Design Decisions

**Why Two Iterator Types?**

The choice between streaming and memory-mapped iteration depends on the access pattern:

1. **Memory-Mapped (StdfRecordIterator)**
   - **Pros**: Fast random access, no buffer management, OS-optimized caching
   - **Cons**: Must map entire file, slower for queries that can exit early
   - **Use when**: Need multiple passes, full file scan, or late records

2. **Streaming (StdfStreamingIterator)**
   - **Pros**: Low memory footprint, can exit early, fast for beginning of file
   - **Cons**: Sequential only, buffer management overhead
   - **Use when**: Early records, limited queries, very large files

**Routing Logic Rationale:**

The intelligent routing in `show_field()` was designed based on STDF file structure:
- **File Structure**: FAR/ATR/MIR at beginning → Test records in middle → MRR/PCR/HBR at end
- **Access Patterns**: Most queries are for either file-level info (MIR) or test data (PRR, PTR)
- **Performance Trade-off**: 4-40ms (streaming) vs 1300ms (memory-mapped) for limited queries
- **Memory Efficiency**: Streaming uses 64KB vs ~file size for memory-mapping

## CLI Commands

**Design Principle: Consistent Argument Order**

All CLI commands follow a consistent pattern where the file/path argument appears **at the end**:
- `stdf <command> [options] [arguments] <file|path>`
- Examples:
  - `stdf dump [RECORD_TYPES]... <FILE>`
  - `stdf count rectypes [RECORD_TYPES]... <PATH>`
  - `stdf show <RECORD> <FIELD> <FILE>`
  - `stdf to <FORMAT> <FILE>`

This follows natural language patterns ("dump MIR from file", "count PTR in file") and is consistent with common Unix tools like `grep`, `cat`, etc.

### Implementation

Built with **clap 4.5** using derive macros for:
- Auto-generated help text (`--help`)
- Auto-generated version flags (`--version`)
- Type-safe argument parsing
- Subcommand hierarchies
- Better error messages

### Implemented Commands

#### **count records** ✅
```bash
stdf count records <file>
```
- Counts total records in file
- Performance: ~3.6s for 3GB file (86M records)

#### **tally records** ✅
```bash
stdf tally records <file>
```
- Counts each record type individually
- Output sorted by count descending

#### **dump** ✅
```bash
stdf dump <file> [--record-type <type>]
```
- Dumps records in human-readable format
- Optional filtering by record type

#### **is** Commands ✅
Boolean validation commands returning exit codes:
- `is ft <file>`: Check if Final Test (TEST_COD='F')
- `is ws <file>`: Check if Wafer Sort (TEST_COD='W')
- `is hot/cold/room <file>`: Check temperature conditions
- `is truncated/complete <file>`: Check file integrity

**Exit codes:** `0`=true, `1`=false, `2`=error

#### **endian** ✅
```bash
stdf endian <file>
```
- Returns "LE" or "BE" based on FAR record

#### **lot** ✅
```bash
stdf lot <file>
```
- Extracts LOT_ID from MIR

#### **tester** ✅
```bash
stdf tester <file>
stdf tester type <file>
```
- Returns tester name and type from MIR

#### **temperature** ✅
```bash
stdf temperature <file> [int]
```
- Returns temperature with intelligent parsing

#### **to atdf** ✅
```bash
stdf to atdf <file>
```
- Converts STDF to ASCII Test Data Format

### Future Commands

#### **export** 🔄 (In Development)
```bash
stdf export pdf <input.stdf> <output.pdf>
stdf export xlsx <input.stdf> <output.xlsx>
stdf export parquet <input.stdf> <output.parquet>
stdf export all <input.stdf> <output-prefix>
```

## Performance Optimizations

1. **Memory-Mapped I/O**: Uses `memmap2` for zero-copy file access
   - 3GB files parsed in ~3.6 seconds (~900 MB/s throughput)
   
2. **Lazy Parsing**: Iterator pattern avoids loading entire file
   - Memory usage independent of file size
   
3. **Zero-Copy Parsing**: `byte` crate enables in-place deserialization

## Compression Support

Supported formats with streaming decompression:
- **gzip/zlib** (`.gz`, `.z`) - Most compatible
- **bzip2** (`.bz2`) - Better compression
- **xz/LZMA** (`.xz`) - Excellent ratio
- **zstd** (`.zst`) - **RECOMMENDED** - Best speed/ratio balance
- **lz4** (`.lz4`) - Fastest decompression
- **zip** (`.zip`) - Archive format
- **tar** (`.tar.*`) - Combined archives

Format detection via `infer` crate (magic bytes, not extension).

## Columnar Data Export

### Polars & Parquet

**Strategy**: Use **polars** DataFrames for columnar representation and **Parquet** export.

**Benefits:**
- High-performance native Rust DataFrame library
- Efficient compression (10-20x reduction)
- Fast columnar queries
- Wide ecosystem support
- Natural fit for test data (tests as columns, parts as rows)

### Excel (XLSX) Export

**Two-tier strategy:**
1. **Simple exports**: polars' built-in Excel writer
2. **Advanced exports**: **rust_xlsxwriter** for rich formatting

---

# Part 2: Visualization & Reporting

## Overview

Generate PDF reports with embedded charts and Excel spreadsheets for deep-dive analysis of semiconductor test data.

**Key Goals:**
- Visual pattern recognition in wafer test data
- Dual output: PDF (quick analysis) + Excel (deep dive)
- Support both wafer sort and final test
- Handle multi-site parallel testing (4-64+ sites)

## Data Flow (Visualization)

```
STDF Parser
    ↓
Polars DataFrame (millions of measurements)
    ↓
Test Report Structures (models)
    ↓
Statistics Calculations (Cp, CpK)
    ↓
Chart Generation (plotters)
    ↓
PDF Report + Excel Spreadsheet
```

### Input Data Structure

**Per Die:**
- PIR (Part Information Record) - site info
- PTR/FTR sequence (n parametric + m functional tests)
- PRR (Part Result Record) - pass/fail + X/Y coordinates

**Key Fields:**
- `X_COORD`, `Y_COORD`: i16, -32768 = no spatial data (final test)
- `SITE_NUM`: Which parallel test site (1-64+)
- `HARD_BIN`, `SOFT_BIN`: Bin assignments
- Test values and limits (LSL, LTL, HTL, HSL)

## Test Types

### Parametric Tests (PTR)
- Measured values with units (mΩ, mA, V, ns, etc.)
- Has test limits and statistical analysis
- Generates full visualization

### Functional Tests (FTR)
- Binary pass/fail results
- No statistics or gradient visualization
- Simple pass/fail percentages

## Visualization Design

### Color Schemes

#### Diverging Gradient (Parametric Tests)
**Purpose**: Visual pattern recognition across dies

**Algorithm**:
```
Blue (low) ← Green (median) → Orange (high)
      ↑                          ↑
   P10 (10th percentile)    P90 (90th percentile)
```

**Color mapping**:
```rust
fn value_to_color(value: f64, p10: f64, median: f64, p90: f64) -> RGB {
    if value < median {
        // Blue → Green interpolation
        let ratio = (value - p10) / (median - p10);
        interpolate(BLUE, GREEN, ratio)
    } else {
        // Green → Orange interpolation
        let ratio = (value - median) / (p90 - median);
        interpolate(GREEN, ORANGE, ratio)
    }
}
```

**Colors**:
- Blue: RGB(0, 128, 255) - Low values
- Green: RGB(0, 255, 0) - Median/center
- Orange: RGB(255, 128, 0) - High values

**Rationale**: Diverging gradient centered on median makes patterns visible regardless of absolute scale.

#### Capability Colors (Statistics Table)
**Purpose**: Highlight poor process capability

**Rules**:
```rust
fn capability_cell_color(cp: f64, cpk: f64) -> Option<Color> {
    if cp < 1.0 || cpk < 1.0 {
        return Some(RED_BACKGROUND);      // Failing capability
    }
    if cp < 1.33 || cpk < 1.33 {
        return Some(ORANGE_BACKGROUND);   // Marginal capability
    }
    None  // Good capability (no background)
}
```

**Colors**:
- Red: RGB(255, 200, 200) - Cp/CpK < 1.0
- Orange: RGB(255, 230, 200) - Cp/CpK < 1.33
- White: No highlight - Cp/CpK ≥ 1.33

### Test Limits Visualization

#### Vertical Scaling (1/12 - 10/12 - 1/12)
**Purpose**: Make test data readable regardless of limit range

**Layout**:
```
┌─────────────────┐  ← HSL (1/12 height)
│   Spec Limit    │
├─────────────────┤  ← HTL
│                 │
│                 │
│   Test Range    │
│   (10/12)       │
│                 │
│                 │
├─────────────────┤  ← LTL
│   Spec Limit    │
└─────────────────┘  ← LSL (1/12 height)
```

**Formula**:
```rust
let total_height = 1200;
let limit_zone = 100;  // 1/12 of total
let test_zone = 1000;  // 10/12 of total

let y_pos = if value > HTL {
    interpolate(HTL_y, HSL_y, (value - HTL) / (HSL - HTL))
} else if value < LTL {
    interpolate(LSL_y, LTL_y, (value - LSL) / (LTL - LSL))
} else {
    interpolate(LTL_y, HTL_y, (value - LTL) / (HTL - LTL))
}
```

#### Gradient Bar
**Visual element**: Colored bar showing where measurement falls in test range

```
LSL    LTL                    HTL    HSL
 ├──────┼──────────────────────┼──────┤
 │ Red  │     Green Zone       │ Red  │
        ↑
        └─ Arrow indicates median position
```

### Charts & Visualizations

#### 1. Wafer Map
**Condition**: Only if `X_COORD ≠ -32768` and `Y_COORD ≠ -32768`

**Layout**:
- Circular wafer boundary
- Scale die X/Y to fit in circle
- Color each die by test value (diverging gradient)
- Show bin failures as dark/black

**Site handling**: Abstract away (average or show S1 only)

#### 2. Trend Chart
**Purpose**: Show test values in die sequence

**Axes**:
- X: Die number (sequence)
- Y: Test value (scaled with 1/12-10/12-1/12)

**Elements**:
- Scatter points colored by gradient
- Median line (green, horizontal)
- Test limits (LTL/HTL as horizontal lines)

#### 3. Histogram (Rotated 90°)
**Purpose**: Show value distribution

**Orientation**: Rotated to fit alongside trend chart

**Elements**:
- Bins colored by gradient
- Normal distribution overlay (if applicable)
- Show skewness visually

#### 4. Gradient Bar
As described above - visual reference for color mapping

#### 5. Statistics Table
**Columns**: Test number, Test name, N, Min, Max, Mean, Median, σ, Cp, CpK, Unit

**Formatting**:
- Red/orange backgrounds for poor Cp/CpK
- Freeze panes (header row)
- Conditional formatting in Excel

### Multi-Site Handling

**Problem**: Parallel testing (S1, S2, S3, S4...) creates visual clutter

**Solution**: Abstract sites in visualizations
- Show **aggregated** wafer maps (average or S1 only)
- Show **all sites** only in statistics table
- Keep raw data with site info in Excel wide matrix

## Output Formats

### PDF Report

**Structure**:
1. **Summary Page**
   - Lot information (LOT_ID, wafer count, test date)
   - Test list with hyperlinks
   - Statistics summary table
   
2. **Individual Test Pages** (one per test)
   - Test name, number, units
   - Wafer map (if spatial data exists)
   - Trend chart
   - Histogram (rotated)
   - Gradient bar
   - Statistics summary

**Technical**:
- Use `printpdf` for PDF generation
- Embed SVG charts generated by `plotters`
- Internal hyperlinks from summary to test pages
- Page size: A4 landscape

### Excel Spreadsheet

**Structure**:
1. **Summary Sheet**
   - Same as PDF summary page
   - Statistics table with formatting
   
2. **Data Matrix Sheet** (Wide format)
   - **Rows**: One per die
   - **Columns**: 
     - PART_ID
     - X_COORD, Y_COORD
     - SITE_NUM
     - HARD_BIN, SOFT_BIN
     - Test_1_value, Test_1_status
     - Test_2_value, Test_2_status
     - ... (all tests as columns)
   
   **Example**:
   ```
   PART_ID | X | Y | SITE | HBIN | SBIN | T1_val | T1_stat | T2_val | T2_stat | ...
   --------|---|---|------|------|------|--------|---------|--------|---------|----
   1       | 0 | 0 | 1    | 1    | 1    | 1.234  | PASS    | 5.678  | PASS    | ...
   2       | 0 | 1 | 1    | 1    | 1    | 1.235  | PASS    | 5.680  | PASS    | ...
   ```

**Formatting**:
- Freeze panes (header row + first 3 columns)
- Conditional formatting for failures
- Cell colors matching PDF gradient (optional)
- Filter buttons enabled

**Technical**:
- Use `rust_xlsxwriter` for rich formatting
- Support up to Excel's 1M row limit
- For larger datasets, split into multiple sheets or use Parquet

### Parquet Cache (Lot-Level Analysis)

**Purpose**: Multi-file aggregation for lot analysis (50-75 STDF files per lot)

**Schema** (columnar):
```
wafer_id: String
part_id: i32
x_coord: i16
y_coord: i16
site_num: u8
hard_bin: u16
soft_bin: u16
test_num: u32
test_name: String
test_value: f64
test_status: String
units: String
lo_limit: f64
hi_limit: f64
```

**Usage Pattern**:
1. Parse each STDF file → DataFrame
2. Write to Parquet with wafer_id
3. For lot analysis: Read all Parquet files → Concatenate → Analyze

**Benefits**:
- 10-20x compression vs raw STDF
- Fast filtering (e.g., "all results for test 1234 across 75 wafers")
- Cross-wafer correlation analysis
- Temperature correlation (hot/room/cold)
- WS→FT correlation

## Statistics Calculations

### Formulas

**Process Capability**:
```
Cp = (HTL - LTL) / (6 × σ)
```

**Process Capability Index (Lower)**:
```
CpKL = (Mean - LTL) / (3 × σ)
```

**Process Capability Index (Upper)**:
```
CpKH = (HTL - Mean) / (3 × σ)
```

**Overall CpK**:
```
CpK = min(CpKL, CpKH)
```

**Interpretation**:
- `Cp ≥ 1.33`: Process capable
- `1.0 ≤ Cp < 1.33`: Marginal capability
- `Cp < 1.0`: Process not capable
- `CpK` accounts for process centering

### Implementation

Using polars for efficient grouped calculations:
```rust
use polars::prelude::*;

let stats = df
    .groupby(&["test_num"])?
    .agg(&[
        col("test_value").count().alias("n"),
        col("test_value").min().alias("min"),
        col("test_value").max().alias("max"),
        col("test_value").mean().alias("mean"),
        col("test_value").median().alias("median"),
        col("test_value").std(1).alias("std"),
    ])?;

// Calculate Cp, CpK from aggregated stats
```

## Lot-Level Analysis Features

### Multi-File Aggregation
1. Parse all STDF files in lot
2. Cache each as Parquet with wafer identifier
3. Load all Parquet files into single DataFrame
4. Generate lot-level reports

### Cross-Wafer Analysis
- Wafer-to-wafer variation
- Identify outlier wafers
- Track trends across wafer sequence

### Temperature Correlation
- Compare hot/room/cold test results
- Identify temperature-sensitive parameters
- Visualize Δ(hot-cold) distributions

### WS→FT Correlation
- Match wafer sort to final test by coordinates
- Calculate FT yield by WS bin
- Identify WS bin splits at FT

## Dependencies (Visualization)

```toml
[dependencies]
# Existing
memmap2 = "0.9"
byte = "0.2"
clap = "4.5"
anyhow = "1.0"
pyo3 = { version = "0.22", optional = true }

# Visualization (NEW)
polars = { version = "0.44", features = ["parquet", "lazy", "dtype-datetime"] }
plotters = "0.3"
rust_xlsxwriter = "0.79"
printpdf = "0.7"
```

## Module Structure

```
src/
├── lib.rs              # Library root, module exports
├── types.rs            # STDF data types
├── records.rs          # STDF V4 record definitions
├── parsers.rs          # STDF parser, iterators
├── tally.rs            # Record tallying
├── conversions.rs      # Data conversions
├── export.rs           # Integration layer (NEW)
│
├── models/             # Data structures (NEW)
│   └── mod.rs          # TestReport, TestLimits, ParametricTest, etc.
│
├── statistics/         # Statistical calculations (NEW)
│   └── mod.rs          # Cp, CpK, distributions
│
├── charts/             # Chart generation (NEW)
│   ├── mod.rs          # Chart trait, factory
│   └── colors.rs       # Color mapping functions
│
└── writers/            # Output writers (RESTRUCTURED)
    ├── mod.rs          # ReportWriter trait
    ├── stdf.rs         # STDF binary writer (existing)
    ├── pdf.rs          # PDF report writer (NEW)
    ├── xlsx.rs         # Excel writer (NEW)
    └── parquet.rs      # Parquet cache writer (NEW)
```

## Implementation Roadmap

### Phase 1: Core Data Conversion ✅ (Partially)
- [x] Define data models (`TestReport`, `ParametricTest`, etc.)
- [x] Implement statistics calculations (Cp, CpK)
- [x] Implement color mapping functions
- [ ] STDF → DataFrame conversion
- [ ] DataFrame → TestReport structures

### Phase 2: Chart Generation 🔄 (In Progress)
- [x] Color gradient algorithm
- [ ] Wafer map renderer
- [ ] Trend chart
- [ ] Histogram (rotated)
- [ ] Gradient bar

### Phase 3: Excel Writer 🔄 (In Progress)
- [ ] Summary sheet
- [ ] Wide data matrix
- [ ] Cell formatting (colors, freeze panes)

### Phase 4: PDF Writer 📋 (Planned)
- [ ] Summary page with test list
- [ ] Embed SVG charts
- [ ] Internal hyperlinks
- [ ] Page layout

### Phase 5: Lot-Level Analysis 📋 (Planned)
- [ ] Parquet caching system
- [ ] Multi-file aggregation
- [ ] Cross-wafer analysis
- [ ] Temperature correlation
- [ ] Lot summary reports

---

# Development & Testing

## Build System

Cargo workspace with multiple crates:
- Main crate: `semi-ate-stdf`
- Procedural macro: `stdf-record-derive`

## Testing

### Test Files
Located in `data/` directory (production STDF files)

### Test Organization
- **Unit tests**: Embedded in source files or separate `tests/` directory
  - `tests/types_test.rs`: Comprehensive type system tests (65 tests)
  - `tests/display_defaults_test.rs`: Display trait tests
  - `tests/roundtrip_tests.rs`: Serialization roundtrip tests
  - `tests/atdf_test.rs`: ATDF conversion tests
  - `src/parsers.rs`: Parser unit tests (embedded)
  
### Test Strategy
1. **Unit tests** for parsers, types, and statistics
2. **Integration tests** with real STDF files
3. **Roundtrip tests** to ensure serialization fidelity
4. **Coverage-driven testing** using cargo-llvm-cov
5. Visual regression tests for charts (future)
6. Performance benchmarks

### Coverage Analysis
- Tool: `cargo-llvm-cov` for line coverage analysis
- Current coverage: **79.9%** on `types.rs` (255/319 lines)
- Workflow:
  1. Write tests
  2. Run `cargo llvm-cov --lcov --output-path lcov.info` to generate report
  3. Run `cargo llvm-cov --html` to generate interactive HTML report
  4. Review uncovered lines in `target/llvm-cov/html/`
  
**Important**: Coverage is **NOT** automatically updated on file save. You must explicitly run coverage tools after changes.

### Types Module Testing Insights
The `types.rs` module (603 lines) defines STDF primitive types with comprehensive test coverage:

**Tested areas** (65 tests in `tests/types_test.rs`):
- All single-byte types: C1, U1, I1, B1, N1
- All multi-byte types: U2, U4, U8, I2, I4, I8, R4, R8, U4T
- Variable-length types: Cn, Bn, Dn, Vn (12 enum variants)
- From/Into trait conversions (13 tests)
- Display and Debug trait implementations
- Binary serialization (TryRead/TryWrite)
- Roundtrip consistency tests

**Remaining uncovered code (~20%)**:
- Error handling paths (malformed binary data)
- U4T Display edge cases (invalid timestamp formatting)
- C1 escape character handling (line 150)
- `to_hex_string()` helper function (lines 363-368)
- Dn/Vn serialization error paths

**Key learnings**:
- Wrapper types (B1, N1) use inner type's default Display (decimal not binary/hex)
- U4T timestamps are timezone-aware (use substring assertions, not exact matches)
- Dn Display only shows hex data (not bit length field)
- From<&T> traits not always implemented - test actual trait bounds

## Python Integration

### Current State
- Python bindings scaffolded via PyO3
- Full API exposure pending

### Future Python API
```python
import stdf

# Parsing
parser = stdf.Parser("test.stdf")
for record in parser:
    print(record)

# High-level operations
info = stdf.info("test.stdf")
df = stdf.to_dataframe("test.stdf")  # Polars/Pandas DataFrame

# Visualization
stdf.export_pdf("test.stdf", "report.pdf")
stdf.export_xlsx("test.stdf", "data.xlsx")
```

## Build & Distribution

### Rust
- Cargo workspace
- Target: Windows, Linux, macOS

### Python
- Maturin for building wheels
- PyPI: `Semi-ATE-stdf`
- Conda: `semi-ate-stdf` (conda-forge)

---

# Design Patterns

### Iterator Pattern
Lazy evaluation for all parsing operations

### Type Safety
Binary format mapped to strongly-typed Rust enums

### Procedural Macros
Code generation for repetitive parsing logic

### Writer Trait
Abstraction for multiple output formats:
```rust
pub trait ReportWriter {
    fn write_report(&mut self, report: &TestReport) -> Result<()>;
}
```

---

# Notes & Considerations

## Performance
- Parser: ~900 MB/s throughput
- Visualization: Slower (chart rendering, file I/O)
- Use progress bars for long operations

## Known Issues
1. **PyO3/Polars version conflict**: May need feature flags or separate binaries
2. **Excel row limit**: 1M rows (use Parquet for larger datasets)
3. **Array field handling**: Not fully implemented in ATDF conversion
4. **File corruption**: Some real-world files don't start with FAR

## Future Enhancements
1. Real-time streaming visualization
2. Interactive web-based reports (WASM)
3. Machine learning integration (outlier detection)
4. Multi-threaded parsing for parallel processing

---

# Appendix: Recent Development Sessions

## 2024-12-10: Test Coverage Improvement

**Context**: Analyzed and improved test coverage for `types.rs` module.

**Actions Taken**:
1. Analyzed existing `lcov.info` coverage data (was outdated from previous test run)
2. Separated type tests from `src/types.rs` to `tests/types_test.rs` (41 → 65 tests)
3. Added comprehensive tests for uncovered code paths:
   - From/Into trait conversions for all primitive types
   - Display/Debug trait implementations
   - All 12 Vn enum variants
   - Binary serialization edge cases
4. Fixed test assertions to match actual implementations (not assumptions)
5. Regenerated coverage: **79.9%** (up from 72%)

**CLI Enhancement**: Added `stdf show records` command to list all 25 STDF V4 record types without requiring a file.

**Key Insights**:
- Coverage data requires explicit regeneration (`cargo llvm-cov`)
- Coverage is NOT automatically updated on file save
- Wrapper type Display traits use inner type's default formatting
- Most uncovered code (~20%) is error handling paths and edge cases
- Test actual behavior, not assumed behavior (discovered B1→"170", N1→"15", U4T timezone awareness)

**Test Results**: All 103 tests passing (12 parser + 1 ATDF + 15 display + 5 roundtrip + 65 types + 5 doc)

---

*Last updated: 2024-12-10*
*Combined from parser (DESIGN-old.md) and visualization (DESIGN-viz.md) documentation*
