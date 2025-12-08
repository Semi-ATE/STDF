# Merge Plan: stdf-export → STDF

This document outlines the systematic steps to merge the stdf-export visualization system into the STDF parser project.

## Current Status

✅ **Completed:**
- DESIGN.md files merged conceptually (stdf-export version copied to STDF/docs)
- Todo list created
- Both projects analyzed

## Merge Steps

### Phase 1: Documentation Migration ✅ READY

**Goal:** Consolidate all documentation in STDF/docs

**Actions:**
1. Copy unique files from `stdf-export/doc/` to `STDF/docs/`:
   - ✅ Already exists: ATDF-spec.pdf, STDF specs (skip)
   - 📋 **COPY**: BRAINSTORM.md
   - 📋 **COPY**: Capture.PNG
   - 📋 **COPY**: TestPerWafer.png  
   - 📋 **COPY**: SummaryPage.pdf
   - 📋 **COPY**: tables.xlsx
   - 📋 **COPY**: autoreports.pptx
   - 📋 **COPY**: home.png
   - 📋 **COPY**: ProbingTop.xcf

2. Merge DESIGN.md (manual):
   - Current DESIGN.md is from stdf-export (visualization focus)
   - Need to add parser information from DESIGN-old.md
   - Create comprehensive document covering both

**Commands:**
```powershell
cd C:\Users\thor\repos\GitHub\Semi-ATE\STDF

# Copy unique documentation
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\BRAINSTORM.md" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\Capture.PNG" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\TestPerWafer.png" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\SummaryPage.pdf" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\tables.xlsx" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\autoreports.pptx" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\home.png" docs\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\doc\ProbingTop.xcf" docs\
```

---

### Phase 2: Source Code Migration 📋 NEXT

**Goal:** Move stdf-export modules into STDF/src

#### Step 2.1: Restructure writers/

**Current state:**
- STDF has: `src/writer.rs` (STDF binary writer)
- Need: `src/writers/` directory with multiple writers

**Actions:**
```powershell
# Create writers directory
New-Item -ItemType Directory -Path src\writers

# Move existing writer to writers/stdf.rs
Move-Item src\writer.rs src\writers\stdf.rs

# Create writers/mod.rs
# (see template below)
```

**Template for `src/writers/mod.rs`:**
```rust
// Output writers for multiple formats

pub mod stdf;   // Existing STDF binary writer
pub mod pdf;    // PDF report writer
pub mod xlsx;   // Excel spreadsheet writer
pub mod parquet; // Parquet cache writer

// Writer trait for abstraction
pub trait ReportWriter {
    fn write_report(&mut self, report: &crate::models::TestReport) -> anyhow::Result<()>;
}
```

#### Step 2.2: Copy new modules

**Copy from stdf-export to STDF:**

```powershell
# Copy entire module directories
Copy-Item -Recurse "C:\Users\thor\repos\nerohmot\stdf-export\src\models" src\
Copy-Item -Recurse "C:\Users\thor\repos\nerohmot\stdf-export\src\statistics" src\
Copy-Item -Recurse "C:\Users\thor\repos\nerohmot\stdf-export\src\charts" src\

# Copy writer implementations (skip mod.rs, we'll create custom)
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\src\writers\pdf.rs" src\writers\
Copy-Item "C:\Users\thor\repos\nerohmot\stdf-export\src\writers\xlsx.rs" src\writers\
```

**Create `src/writers/parquet.rs`:**
```rust
// Parquet writer for lot-level caching

use anyhow::Result;
use polars::prelude::*;

pub struct ParquetWriter {
    output_path: String,
}

impl ParquetWriter {
    pub fn new(output_path: String) -> Self {
        ParquetWriter { output_path }
    }
    
    pub fn write(&mut self, df: &DataFrame) -> Result<()> {
        df.clone().lazy().collect()?.write_parquet(
            &self.output_path,
            ParquetWriter::default()
        )?;
        Ok(())
    }
}
```

---

### Phase 3: Update lib.rs 📋

**Goal:** Export new modules in STDF/src/lib.rs

**Current modules in lib.rs:**
```rust
pub mod types;
pub mod records;
pub mod tally;
pub mod conversions;
```

**Add new modules:**
```rust
// Add after existing modules
pub mod models;
pub mod statistics;
pub mod charts;
pub mod writers;

// Re-export key types for convenience
pub use models::{TestReport, ParametricTest, FunctionalTest, TestLimits};
pub use statistics::Statistics;
```

**Note:** May need to adjust visibility (pub/pub(crate)) based on what should be public API

---

### Phase 4: Update Cargo.toml 📋

**Goal:** Add new dependencies for visualization

**Current dependencies:**
```toml
pyo3 = { version = "0.22", features = ["extension-module"], optional = true }
byte = "0.2"
memmap2 = "0.9"
stdf-record-derive = { path = "stdf-record-derive", version = "0.2" }
flate2 = "1.0"
zip = { version = "0.6", default-features = false, features = ["deflate"] }
indicatif = "0.17"
infer = "0.16"
# polars = { version = "0.44", features = ["parquet", "lazy"] }  # TODO: Re-enable
rust_xlsxwriter = "0.68"
regex = "1.11"
```

**Add/uncomment:**
```toml
# Uncomment and update polars
polars = { version = "0.44", features = ["parquet", "lazy", "dtype-datetime"] }

# Add new dependencies
plotters = "0.3"
printpdf = "0.7"
anyhow = "1.0"  # If not already present
```

**Update rust_xlsxwriter:**
```toml
rust_xlsxwriter = "0.79"  # Update from 0.68
```

---

### Phase 5: Fix Import Paths 📋

**Goal:** Update imports in moved files

After moving files, many imports will break. Need to update:

**In `src/writers/stdf.rs`:**
```rust
// OLD (if it had these)
use crate::records::*;
use crate::types::*;

// STAYS THE SAME (already correct)
```

**In `src/writers/pdf.rs`, `xlsx.rs`:**
```rust
// Update imports
use crate::models::*;
use crate::statistics::*;
use crate::charts::*;
```

**In `src/models/mod.rs`, `statistics/mod.rs`, `charts/mod.rs`:**
```rust
// These should mostly work as-is since they're self-contained
// May need to import from parent crate if they reference records
use crate::records::*;  // If needed
use crate::types::*;    // If needed
```

---

### Phase 6: Compilation & Fixes 📋

**Goal:** Get everything compiling

**Expected issues:**
1. **Dead code warnings**: Unused fields in writers (OK for now)
2. **Missing trait implementations**: May need to implement Debug, Clone, etc.
3. **Polars/PyO3 version conflicts**: Comment in Cargo.toml mentions this
4. **Import path errors**: Fix as discovered

**Iterative process:**
```powershell
# Try to build
cargo check

# Fix errors one by one
# Repeat until clean
```

---

### Phase 7: Update CLI (src/stdf.rs or src/bin/stdf.rs) 📋

**Goal:** Add new export commands

**New commands to implement:**
```bash
stdf export pdf <input.stdf> <output.pdf>
stdf export xlsx <input.stdf> <output.xlsx>
stdf export parquet <input.stdf> <output.parquet>
stdf export all <input.stdf> <output-prefix>  # Generates all formats
```

**Stub implementation in CLI:**
```rust
#[derive(Subcommand)]
enum Commands {
    // ... existing commands ...
    
    /// Export STDF data to various formats
    Export {
        #[command(subcommand)]
        format: ExportFormat,
    },
}

#[derive(Subcommand)]
enum ExportFormat {
    /// Export as PDF report
    Pdf {
        input: PathBuf,
        output: PathBuf,
    },
    /// Export as Excel spreadsheet
    Xlsx {
        input: PathBuf,
        output: PathBuf,
    },
    /// Export as Parquet
    Parquet {
        input: PathBuf,
        output: PathBuf,
    },
    /// Export all formats
    All {
        input: PathBuf,
        output_prefix: String,
    },
}
```

---

### Phase 8: Integration Layer 📋

**Goal:** Connect parser to visualization

**Create `src/export.rs`:**
```rust
// High-level export functions

use crate::parsers::StdfParser;
use crate::models::*;
use crate::writers::*;
use polars::prelude::*;
use anyhow::Result;

/// Parse STDF file into DataFrame
pub fn stdf_to_dataframe(path: &Path) -> Result<DataFrame> {
    let parser = StdfParser::new(path)?;
    
    // Collect all PTR/FTR records
    // Build vectors for each column
    // Create DataFrame
    
    todo!("Implement STDF → DataFrame conversion")
}

/// Generate PDF report from STDF
pub fn export_pdf(input: &Path, output: &Path) -> Result<()> {
    let df = stdf_to_dataframe(input)?;
    let report = build_test_report(df)?;
    
    let mut writer = pdf::PdfWriter::new(output.to_string_lossy().to_string());
    writer.write_report(&report)?;
    
    Ok(())
}

/// Generate Excel spreadsheet from STDF
pub fn export_xlsx(input: &Path, output: &Path) -> Result<()> {
    let df = stdf_to_dataframe(input)?;
    // ... similar to PDF
    todo!()
}

/// Cache as Parquet for lot analysis
pub fn export_parquet(input: &Path, output: &Path) -> Result<()> {
    let df = stdf_to_dataframe(input)?;
    df.lazy().collect()?.write_parquet(
        output.to_str().unwrap(),
        ParquetWriter::default()
    )?;
    Ok(())
}

// Helper function
fn build_test_report(df: DataFrame) -> Result<TestReport> {
    // Group by test, build ParametricTest/FunctionalTest structs
    todo!()
}
```

---

### Phase 9: Testing & Validation 📋

**Goal:** Verify everything works

**Test checklist:**
1. ✅ Project compiles (`cargo build`)
2. ✅ Unit tests pass (`cargo test`)
3. ✅ CLI help works (`cargo run -- --help`)
4. ✅ Existing commands still work
5. ✅ New export commands available (even if stubbed)
6. ✅ No regressions in parser performance

**Test with sample data:**
```powershell
# Existing functionality
cargo run -- count records data/sample.stdf
cargo run -- to atdf data/sample.stdf > output.atdf

# New functionality (when implemented)
cargo run -- export parquet data/sample.stdf output.parquet
cargo run -- export xlsx data/sample.stdf output.xlsx
```

---

### Phase 10: Git Commit 📋

**Goal:** Commit the merged codebase

```powershell
git add .
git status  # Review changes

git commit -m "Merge stdf-export visualization system into STDF

Integrated visualization and report generation capabilities:
- Added models/, statistics/, charts/ modules
- Restructured writers/ (stdf.rs, pdf.rs, xlsx.rs, parquet.rs)
- Updated dependencies: polars, plotters, printpdf
- Added export commands to CLI (stub implementation)
- Merged documentation (DESIGN.md, BRAINSTORM.md, examples)

Features to implement:
- STDF → DataFrame conversion
- PDF report generation with charts
- Excel wide data matrix export
- Parquet caching for lot analysis
- Chart rendering (plotters)
- Statistics calculations (Cp, CpK)
- Wafer map visualization

This merge combines the high-performance STDF parser with advanced
visualization capabilities for semiconductor test data analysis."
```

---

## Phase 11: Implementation Roadmap 📋 FUTURE

After merge is complete, implement in this order:

### 11.1 Core Data Conversion
1. STDF → DataFrame (STR/PTR/FTR records → columnar)
2. DataFrame → TestReport structures
3. Statistics calculations (using polars group_by)

### 11.2 Chart Generation
1. Color mapping functions
2. Wafer map renderer
3. Trend chart
4. Histogram (rotated)
5. Gradient bar

### 11.3 Excel Writer
1. Summary sheet
2. Wide data matrix
3. Cell formatting (colors, freeze panes)

### 11.4 PDF Writer
1. Summary page with test list
2. Embed SVG charts
3. Internal hyperlinks
4. Page layout

### 11.5 Lot-Level Analysis
1. Parquet caching system
2. Multi-file aggregation
3. Cross-wafer analysis
4. Temperature correlation
5. Lot summary reports

---

## Known Issues & Considerations

### PyO3/Polars Version Conflict
- Cargo.toml has polars commented out due to PyO3 version conflict
- May need to:
  - Update PyO3 to compatible version
  - Use separate feature flags for Python vs visualization
  - Build two separate binaries

### File Organization
- STDF has `src/bin/` directory for binaries
- May want to move CLI to `src/bin/stdf.rs`
- Keep `src/lib.rs` for library API

### Performance
- Parser is very fast (~900 MB/s)
- Visualization will be slower (chart rendering)
- Use progress bars (indicatif) for long operations

### Testing
- Need test STDF files with known good data
- Create golden outputs for comparison
- Visual regression tests for charts

---

## Quick Reference: File Locations

### Before Merge (Current State)

**stdf-export:**
```
stdf-export/
├── doc/
│   ├── DESIGN.md
│   ├── BRAINSTORM.md
│   └── *.png, *.pdf
└── src/
    ├── models/
    ├── statistics/
    ├── charts/
    └── writers/
        ├── pdf.rs
        └── xlsx.rs
```

**STDF:**
```
STDF/
├── docs/
│   ├── DESIGN.md (parser focus)
│   └── *.pdf specs
└── src/
    ├── lib.rs
    ├── records.rs
    ├── types.rs
    ├── tally.rs
    ├── conversions.rs
    └── writer.rs (STDF binary)
```

### After Merge (Target State)

**STDF (merged):**
```
STDF/
├── docs/
│   ├── DESIGN.md (comprehensive - parser + viz)
│   ├── DESIGN-old.md (backup)
│   ├── BRAINSTORM.md
│   ├── *.pdf specs
│   └── *.png examples
└── src/
    ├── lib.rs (updated with new modules)
    ├── records.rs
    ├── types.rs
    ├── tally.rs
    ├── conversions.rs
    ├── export.rs (NEW - integration layer)
    ├── models/
    │   └── mod.rs
    ├── statistics/
    │   └── mod.rs
    ├── charts/
    │   ├── mod.rs
    │   └── colors.rs
    └── writers/
        ├── mod.rs (NEW)
        ├── stdf.rs (moved from writer.rs)
        ├── pdf.rs
        ├── xlsx.rs
        └── parquet.rs (NEW)
```

---

## Execution Checklist

Use this to track progress:

- [ ] Phase 1: Documentation Migration
  - [ ] Copy unique files from stdf-export/doc
  - [ ] Merge DESIGN.md (parser + visualization)
  - [ ] Verify no duplicates
  
- [ ] Phase 2: Source Code Migration
  - [ ] Create src/writers/ directory
  - [ ] Move writer.rs → writers/stdf.rs
  - [ ] Create writers/mod.rs
  - [ ] Copy models/, statistics/, charts/
  - [ ] Copy pdf.rs, xlsx.rs
  - [ ] Create parquet.rs
  
- [ ] Phase 3: Update lib.rs
  - [ ] Add module declarations
  - [ ] Add re-exports
  
- [ ] Phase 4: Update Cargo.toml
  - [ ] Uncomment/update polars
  - [ ] Add plotters, printpdf
  - [ ] Update rust_xlsxwriter version
  
- [ ] Phase 5: Fix Import Paths
  - [ ] Fix imports in writers/
  - [ ] Fix imports in models/
  - [ ] Fix cross-module references
  
- [ ] Phase 6: Compilation & Fixes
  - [ ] Run cargo check
  - [ ] Fix compilation errors
  - [ ] Resolve warnings
  
- [ ] Phase 7: Update CLI
  - [ ] Add export subcommands
  - [ ] Create command handlers (stubs)
  
- [ ] Phase 8: Integration Layer
  - [ ] Create src/export.rs
  - [ ] Implement stdf_to_dataframe stub
  - [ ] Implement export_pdf/xlsx/parquet stubs
  
- [ ] Phase 9: Testing & Validation
  - [ ] Verify compilation
  - [ ] Run existing tests
  - [ ] Test CLI commands
  - [ ] Check for regressions
  
- [ ] Phase 10: Git Commit
  - [ ] Review all changes
  - [ ] Commit with detailed message
  
- [ ] Phase 11: Implementation (Future)
  - [ ] See detailed roadmap above

---

## Contact for Questions

If unclear on any step:
1. Check DESIGN.md for architectural context
2. Check BRAINSTORM.md for design decisions
3. Look at stdf-export code for reference implementations
4. Ask me (GitHub Copilot) for clarification!

**Good luck with the merge! 🚀**
