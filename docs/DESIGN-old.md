# DESIGN.md - STDF Parser Architecture & Implementation

## Project Overview

**semi-ate-stdf** is a high-performance Standard Test Data Format (STDF) parser and manipulation tool written in Rust with Python bindings. STDF is the industry-standard file format used by semiconductor Automatic Test Equipment (ATE) to log test program data.

**Key Design Goals:**
- Maximum performance through Rust and memory-mapped I/O
- Comprehensive CLI tool for STDF file operations
- Python bindings for integration with data analysis workflows
- Support for multiple output formats (ATDF, XLSX, HDF5)

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
  - `writer`: STDF writing capabilities

#### 2. **records.rs** - Record Definitions
- Defines all STDF V4 record types as Rust structs
- `V4` enum: Tagged union of all possible record types
- Uses procedural macro `#[derive(StdfRecord)]` for automatic parsing
- Each record implements:
  - Binary parsing from byte streams
  - ASCII (ATDF) conversion via `Display` trait
  - Field-level access

#### 3. **parsers.rs** - Parsing Logic
- `StdfParser`: Memory-mapped file wrapper using `memmap2` crate
- `StdfRecordIterator`: Iterator over STDF records (returns raw bytes)
- `StdfRecordFPIterator`: Iterator over STDF records with file pointers (returns tuple of bytes + offset)
  - **Performance**: Identical to `StdfRecordIterator` (~906 MB/s on 3GB files)
  - **Zero overhead**: File pointer is already tracked internally for iteration
  - **Recommendation**: Use `StdfRecordFPIterator` for all new code; deprecate `StdfRecordIterator`
- `stdf_parse_record()`: Parses individual records from byte slices
- `valid_file()`: Validates STDF file structure (starts with FAR, ends with MRR)
- Handles both little-endian and big-endian files
- Uses `byte` crate for efficient binary parsing

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
- **Recent fix:** Changed from append-based to join-based field concatenation to eliminate trailing pipes

#### 6. **main.rs (stdf.rs)** - CLI Application
- Command-line interface for STDF operations
- Built with clap for argument parsing
- Supports multiple subcommands (see Commands section)

### Data Flow

```
STDF File (binary)
    ↓
StdfParser (memory-mapped)
    ↓
StdfRecordIterator (lazy parsing)
    ↓
V4 enum (typed records)
    ↓
Operations (count, dump, convert, tally, etc.)
    ↓
Output (stdout, files, exit codes)
```

## CLI Commands

### Implemented Commands

#### **count records** ✅
```bash
stdf count records <file>
```
- Counts total records in file
- Performance: ~3.6s for 3GB file (86M records)
- Returns count as stdout

#### **tally records** ✅
```bash
stdf tally records <file>
```
- Counts each record type individually
- Uses HashMap for aggregation
- Output sorted by count descending
- Example:
  ```
  PTR: 41234567
  FTR: 12345678
  PIR: 8765432
  ...
  ```

#### **dump** ✅
```bash
stdf dump <file> [--record-type <type>]
```
- Dumps records in human-readable format
- Optional filtering by record type (e.g., `--record-type MIR`)
- Shows each record's fields and values

#### **is** Commands ✅
Boolean validation commands returning exit codes:
- `is ft <file>`: Check if Final Test (TEST_COD='F')
- `is ws <file>`: Check if Wafer Sort (TEST_COD='W')
- `is hot <file>`: Check temperature conditions (>100°C or "H"/"HOT")
- `is cold <file>`: Check cold conditions (<0°C or "C"/"COLD")
- `is room <file>`: Check room temp (20-30°C or "R"/"ROOM")
- `is truncated <file>`: Check if file is incomplete (corrupted/truncated)
- `is complete <file>`: Check if file ends with MRR record

**Exit codes:** `0`=success/true, `1`=false/not found, `2`=error

**Cross-shell compatibility:** Exit codes work identically across all platforms and shells:
- Windows: PowerShell (`$LASTEXITCODE`), CMD (`%ERRORLEVEL%`)
- Linux/macOS: bash/zsh/fish/sh (`$?`)

#### **endian** ✅
```bash
stdf endian <file>
```
- Returns "LE" or "BE" based on FAR record's CPU_TYPE field

#### **lot** ✅
```bash
stdf lot <file>
```
- Extracts LOT_ID from MIR (Master Information Record)

#### **tester** ✅
```bash
stdf tester <file>
stdf tester type <file>
```
- `tester`: Returns NODE_NAM (tester name) from MIR
- `tester type`: Returns TSTR_TYP (tester type) from MIR

#### **temperature** ✅
```bash
stdf temperature <file>
stdf temperature <file> int
```
- Default: Returns raw TEMP_TXT string from MIR
- With `int`: Intelligent parsing to integer Celsius
  - Special cases:
    - `R`/`ROOM` → 25°C
    - `C`/`COLD` → -40°C
    - `H`/`HOT` → 150°C
  - Handles suffixes: "25C", "170F" (converts F→C)
  - Parses numeric strings: "+25", "85"

#### **to atdf** ✅
```bash
stdf to atdf <file>
```
- Converts STDF to ASCII Test Data Format
- Pipe-delimited fields
- **Recent fix:** Removed trailing pipe from each record

### Stubbed Commands (Not Yet Implemented)

#### **count** Subcommands
- `count parts <file>`: Total parts tested
- `count parts unique <file>`: Unique parts (excluding retests)
- `count sbins <file>`: Number of soft bins
- `count hbins <file>`: Number of hard bins

#### **tally** Subcommands
- `tally heads <file>`: Tally by test head
- `tally sites <file>`: Tally by site
- `tally hbins <file>`: Tally by hard bin
- `tally sbins <file>`: Tally by soft bin

#### **to** Conversion Commands
- `to xlsx <file>`: Convert to Excel format
- `to hdf5 <file>`: Convert to HDF5 format

## Key Implementation Details

### Performance Optimizations

1. **Memory-Mapped I/O**: Uses `memmap2` for zero-copy file access
   - 3GB files parsed in ~3.6 seconds
   - ~822 MB/s throughput
   
2. **Lazy Parsing**: Iterator pattern avoids loading entire file
   - Memory usage independent of file size
   - Early exit for single-record queries

3. **Zero-Copy Parsing**: `byte` crate enables in-place deserialization
   - No intermediate allocations for fixed-size types
   - Minimal string copying

### Special Parsing Logic

#### Temperature Parsing (`parse_temperature_as_int`)
```rust
fn parse_temperature_as_int(temp_str: &str) -> Option<i32>
```
Handles diverse temperature formats:
- Normalized strings: "R"/"ROOM" → 25, "C"/"COLD" → -40, "H"/"HOT" → 150
- Suffix handling: "25C" → 25, "170F" → 76 (converts F→C)
- Plain integers: "+25" → 25, "85" → 85
- Formula: `(F - 32) * 5 / 9` for Fahrenheit conversion

#### ATDF Generation
Uses procedural macro to generate `ascii()` method:
- Collects all fields into Vec<String>
- Joins with "|" separator (no trailing pipe)
- Handles optional fields (empty string if None)
- Array fields: placeholder "[array]" (future enhancement)

### Error Handling

- **File I/O errors**: Propagated to caller with context
- **Parse errors**: Graceful handling with error messages
- **Validation failures**: Exit code 1 for boolean checks, stderr messages
- **Truncated files**: Detected via parse errors during iteration

## Testing

### Test Files
Located in `data/` directory:
- Production STDF files (1-3GB each)
- Various ATE vendors and test types
- Known issue: `3xjxnh04.std` is corrupted (doesn't start with FAR)

### Test Strategy
1. Unit tests for individual parsers (60 tests passing)
2. Integration tests with real STDF files
3. Performance benchmarks on large files

### Missing Test Coverage
- No truncated file samples (TODO: create test files)
- No incomplete file samples (TODO: create file without MRR)
- Array field handling not fully tested

## Python Integration

### Current State
- Python bindings scaffolded via PyO3
- Placeholder functions (`add`, `multiply`) for testing
- Full API not yet exposed to Python

### Future Python API
```python
import stdf

# Parser
parser = stdf.Parser("test.stdf")
for record in parser:
    print(record.type, record.fields)

# High-level operations
info = stdf.info("test.stdf")
df = stdf.to_dataframe("test.stdf")  # Pandas DataFrame
```

## Build & Distribution

### Rust
- Cargo workspace with main crate + procedural macro
- Target: Windows, Linux, macOS

### Python
- Maturin for building wheels
- PyPI distribution: `Semi-ATE-stdf`
- Conda distribution: `semi-ate-stdf` (conda-forge)

### Native Installers
- Windows: WiX-based MSI installer
- Linux: Debian packages (.deb)

## Dependencies

### Core Dependencies
- `memmap2 ^0.9`: Memory-mapped file I/O
- `byte ^0.2`: Binary parsing with endianness
- `clap ^4.5`: CLI argument parsing
- `indicatif ^0.17`: Progress bars (not yet used)

### Python Dependencies
- `pyo3 ^0.23`: Rust-Python bindings
- `maturin ^1.7`: Wheel building

### Development Environment
- Conda environment: `Semi-ATE-STDF-dev`
- Python 3.13
- `py7zr`: For test data compression

## Recent Changes (Session Log)

### Commit 2a51179 (2024-12-08)
**Summary:** Implemented tally records, tester commands, and various improvements

1. **ATDF Fix**: Removed trailing pipe from ascii() output
   - Changed from append-based to join-based field concatenation
   - File: `stdf-record-derive/src/lib.rs`

2. **Tally Records**: Full implementation
   - HashMap-based counting of each record type
   - Sorted output by count descending
   - File: `src/stdf.rs`

3. **Tester Commands**: 
   - `stdf tester`: Extract NODE_NAM from MIR
   - `stdf tester type`: Extract TSTR_TYP from MIR
   - File: `src/stdf.rs`

4. **Temperature Parsing**: Intelligent int conversion
   - Special case handling (R/ROOM, C/COLD, H/HOT)
   - Fahrenheit to Celsius conversion
   - Suffix parsing (25C, 170F)
   - File: `src/stdf.rs`

5. **Validation Commands**: is hot/cold/room/truncated/complete
   - Exit code-based boolean returns
   - File: `src/stdf.rs`

6. **Infrastructure**:
   - Added TODO.md for feature tracking
   - Added environment.yml for Python dev setup
   - Added scripts/decompress_data.py for test data management
   - Updated .gitignore and Cargo.toml

**Stats**: 7 files changed, 806 insertions(+), 66 deletions(-)

## Future Roadmap

### Priority 1: Complete Command Implementation
- Implement all count/tally subcommands
- Add xlsx/hdf5 conversion
- Create test files for validation commands

### Priority 2: Python API
- Expose full parser API to Python
- Add high-level convenience functions
- Pandas DataFrame conversion

### Priority 3: Performance
- Parallel parsing for multi-core systems
- Streaming output for large conversions
- Progress bars for long operations

### Priority 4: Features
- STDF writing from other formats
- Filtering/transformation operations
- Statistical analysis commands

## Design Patterns

### Iterator Pattern
All parsing uses Rust iterators for lazy evaluation:
```rust
for record in parser.iter() {
    // Process one record at a time
}
```

### Type Safety
STDF's weakly-typed binary format mapped to strongly-typed Rust:
```rust
enum V4 {
    MIR(MIR),  // Master Information Record
    PIR(PIR),  // Part Information Record
    PTR(PTR),  // Parametric Test Record
    // ... 60+ record types
}
```

### Procedural Macros
Code generation for repetitive parsing logic:
```rust
#[derive(StdfRecord)]
#[stdf(typ = 1, sub = 10)]
struct MIR {
    setup_t: U4,
    start_t: U4,
    // ... 30+ fields
}
// Generates: parsing, Display, ascii() method
```

## Columnar Data Export

### Polars & Parquet

**Strategy**: Use **polars** DataFrames for columnar data representation and export to **Parquet** format.

**Rationale:**
- **polars**: High-performance DataFrame library built on Apache Arrow
  - Native Rust implementation (no FFI overhead)
  - Lazy evaluation for memory efficiency
  - Built-in Parquet writer
  - Rich DataFrame API for transformations and aggregations
  - Python interoperability (polars-python)

- **Parquet**: Columnar storage format ideal for STDF data
  - Efficient compression (typical 10-20x reduction)
  - Fast columnar queries (e.g., "get all results for test 1234")
  - Schema preservation with strong typing
  - Wide ecosystem support (Python pandas, DuckDB, Spark, Arrow, etc.)
  - Industry standard for analytical workloads

**Usage Pattern:**
```rust
use polars::prelude::*;

let df = DataFrame::new(vec![
    Series::new("test_num", test_nums),
    Series::new("result", results),
    Series::new("part_id", part_ids),
])?;

df.write_parquet("output.parquet", ParquetWriter::default())?;
```

**Benefits for STDF:**
- Efficient storage of millions of test results
- Fast filtering and aggregation queries
- Easy integration with data analysis pipelines
- Natural fit for test data (test parameters as columns, parts as rows)

### Excel (XLSX) Export

**Two-tier strategy** for Excel file generation:

1. **Simple exports**: Use polars' built-in Excel writer
   - Basic data export without formatting
   - Limited by Excel's ~1M row limit
   - Good for small reports or summaries
   - Fast and simple API

2. **Advanced exports**: Use **rust_xlsxwriter** for full Excel features
   - Cell formatting (colors, fonts, borders, alignment)
   - Conditional formatting (highlight failures, color scales)
   - Data validation and dropdown lists
   - Cell comments and notes
   - Merged cells and row/column grouping
   - Charts and sparklines
   - Formulas and calculations
   - Multiple worksheets with cross-references
   - Images and logos

**Workflow for advanced Excel:**
```rust
use rust_xlsxwriter::*;

// Build data with polars, then extract for formatting
let mut workbook = Workbook::new();
let worksheet = workbook.add_worksheet();

// Apply rich formatting
let header_format = Format::new()
    .set_bold()
    .set_background_color(Color::RGB(0x4472C4))
    .set_font_color(Color::White);
    
worksheet.write_with_format(0, 0, "TEST_NUM", &header_format)?;

// Add conditional formatting for failures
let fail_format = Format::new().set_background_color(Color::Red);
worksheet.conditional_format(1, 3, 1000, 3, 
    &ConditionalFormatCell::new()
        .set_criteria(ConditionalFormatCellCriteria::LessThan)
        .set_value(0.0)
        .set_format(&fail_format)
)?;

workbook.save("report.xlsx")?;
```

**Recommendation:**
- Use **Parquet** for large datasets and data pipelines
- Use **polars.write_excel()** for quick data dumps
- Use **rust_xlsxwriter** for presentation-quality reports with formatting

## Compression Support

### Supported Formats

The parser can support streaming decompression for the following formats:

1. **flate2** - gzip/zlib/deflate compression (`.gz`, `.z`)
   - Most widely compatible
   - Moderate speed and compression ratio
   - Standard library support

2. **bzip2** - BZ2 compression (`.bz2`)
   - Better compression than gzip
   - Slower than gzip

3. **xz2** - LZMA/XZ compression (`.xz`)
   - Excellent compression ratio
   - Slower compression/decompression

4. **zstd** - Zstandard compression (`.zst`)
   - **RECOMMENDED**: Best balance of speed and compression ratio
   - Fast decompression
   - Modern algorithm by Facebook

5. **lz4** - LZ4 compression (`.lz4`)
   - Extremely fast decompression
   - Lower compression ratio
   - Good for real-time processing

6. **zip** - ZIP archives (`.zip`)
   - Archive format (can contain multiple files)
   - Uses deflate compression internally
   - Cross-platform standard

7. **tar** - TAR archives (`.tar`, `.tar.gz`, `.tar.bz2`, `.tar.xz`, `.tar.zst`)
   - Archive format (can contain multiple files)
   - Commonly combined with compression formats above

### Implementation Notes

- All formats support streaming/on-the-fly decompression via Rust's `Read` trait
- No need to decompress entire file into memory
- Can wrap file handles transparently with decoders
- Existing iterator code works unchanged with compressed input
- **File format detection**: Using `infer` crate for content-based detection (magic bytes)
  - Detects compression format by reading file header, not by extension
  - Works even if file extension is wrong or missing
  - Supports all compression formats listed above
  - Lightweight and fast (only reads first few bytes)

## Notes & Gotchas

1. **Endianness**: FAR record determines byte order for entire file
2. **Variable-length fields**: Strings and arrays have length prefix
3. **Optional fields**: Missing fields treated as None/empty
4. **Array fields**: Not fully implemented in ATDF conversion
5. **File corruption**: Some real-world files don't start with FAR (handle gracefully)
6. **MRR requirement**: Valid files should end with MRR, but many don't

## Contact & Contribution

This is an active development project. See TODO.md for pending work.

---

*Last updated: 2024-12-08*
*Commit: 2a51179*
