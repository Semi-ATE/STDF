# COMMANDS.md - TODO Checklist

This document tracks commands that need more detailed documentation before merging into DESIGN.md.

## Legend
- ✅ Complete - Has detailed specification with examples, output format, implementation notes
- 🔄 Partial - Has basic syntax but needs more details
- ❌ Minimal - Only has command syntax, needs full specification

---

## Command Groups Status

### 1. `stdf show` - Extract single values from STDF files
- 🔄 **Basic commands** (tester, endian, temperature, lot, sublot, device, version)
  - Has: Basic syntax and field mappings
  - Needs: 
    - Output format examples (especially for directory mode)
    - Error handling (missing fields, corrupted files)
    - Exit codes specification
    - Temperature parsing logic (multiple sources: MIR, WCR, specific records?)

- ❌ **Generic field extraction** (mir, mrr, wcr)
  - Has: Basic syntax only
  - Needs:
    - List of available field names for each record type
    - Output format for different data types (strings, numbers, arrays)
    - Handling of missing/optional fields
    - Examples for common use cases

### 2. `stdf dump` - Output records in human-readable format
- ❌ **Needs full specification**
  - Has: Command syntax only
  - Needs:
    - Output format (ATDF? JSON? Custom format?)
    - Record type filtering syntax (comma-separated? multiple flags?)
    - Complete example with input/output
    - Performance considerations for large files

### 3. `stdf count` - Count various entities
- ✅ **Basic counting** (records, parts, tests, wafers)
  - Reasonably complete with return value notes
  - Minor needs: Exit code meanings, error handling

- 🔄 **Specific record type counting**
  - Has: Basic syntax
  - Needs: List of valid record type names/abbreviations

### 4. `stdf tally` - Generate summary tables
- ✅ **tally tests** - Complete with table format and implementation notes
- ✅ **tally sites** - Complete with table format and implementation notes
- ✅ **tally records** - Complete with table format

- ❌ **tally sbins** - Empty, needs full specification
  - Needs: Table format, which records to scan (SBR? PRR?), sorting order
  
- ❌ **tally hbins** - Empty, needs full specification
  - Needs: Table format, which records to scan (HBR? PRR?), sorting order
  
- ❌ **tally bins** - Empty, needs clarification
  - Needs: What does this show? Both hbins and sbins? Relationship between them?
  
- ❌ **tally parts per sbin** - Empty, needs full specification
  - Needs: Table format, show part count distribution by sbin
  
- ❌ **tally parts per hbin** - Empty, needs full specification
  - Needs: Table format, show part count distribution by hbin

### 5. `stdf is` - Boolean validation commands
- ✅ **Test type checks** (ft, ws) - Has clear logic
- ✅ **Temperature checks** (hot, cold, room) - Has temperature ranges
- 🔄 **Format checks** (binary, ascii) - Has basic info
  - Needs: How to distinguish STDF vs ATDF (file extension? magic bytes?)
  
- ✅ **Integrity checks** (complete, truncated) - Clear logic
- ❌ **Filename compliance** (correct) - Needs STDF filename spec
- 🔄 **stdf check** (stdf, sane) - Has some logic but needs more detail
  - `is stdf`: Good multi-step check, but needs details on what "compliant name" means
  - `is sane`: Good start on sanity checks, needs complete list
  
- 🔄 **Compression checks** (compressed, decompressed)
  - Has: Basic concept
  - Needs: List of detectable formats, how detection works

### 6. `stdf has` - Check for presence of elements
- ❌ **Needs full specification**
  - Has: Basic command syntax
  - Needs:
    - `has <record_type>`: How to specify record types? Exit codes?
    - `has compliant name`: What is the STDF filename specification?
    - `has supported compression`: Output format for the compression type

### 7. `stdf to` - Format conversion commands
- 🔄 **Export commands** (parquet, atdf, xlsx, pdf)
  - Has: Basic syntax for some
  - Needs:
    - `to parquet`: Full specification (schema, compression, options)
    - `to atdf`: Probably implemented, needs examples
    - `to xlsx`: Format specification (see DESIGN.md for details)
    - `to pdf`: Format specification (see DESIGN.md for details)
    
- ✅ **Endian conversion** (BE, LE) - Has good detail including filename conventions

### 8. `stdf repair` - Fix corrupted files
- ❌ **Needs full specification**
  - Has: Basic idea (repair truncated files)
  - Needs:
    - What repairs can be done? (add MRR record? truncate at last valid record?)
    - Success/failure criteria
    - Backup strategy
    - Examples of repairable vs non-repairable corruption

### 9. `stdf combine` - Merge multiple STDF files
- 🔄 **Has good constraints but needs more**
  - Has: Validation rules (same tester, program, temp, type)
  - Needs:
    - How records are merged/ordered
    - Output file naming if not specified
    - What to do with conflicting MIR fields
    - Handling of duplicate parts/wafers
    - Examples

### 10. `stdf anonymize` - Remove sensitive information
- 🔄 **Has basic concept**
  - Has: Command syntax, output naming convention
  - Needs:
    - **Critical**: Which fields are removed/anonymized?
    - Strategy: Redact, hash, or replace with generic values?
    - Examples: Before/after comparison
    - Use cases: Why anonymize? (sharing test data publicly?)

### 11. `stdf rename` - Rename files according to conventions
- 🔄 **Has good ideas but needs refinement**
  - `to hash`: SHA-256 hash of what? File content? Specific fields?
  - `to tt`: Good explanation of uniqueness, needs:
    - Exact filename format (NODE_NAM-START_T?)
    - Time format (Unix timestamp? ISO8601? Custom?)
    - Collision handling
    - Examples

### 12. `stdf check` - Validate file integrity/compliance
- ❌ **Needs full specification**
  - Has: Command syntax only
  - Needs:
    - What checks are performed?
    - Output format (table of issues? exit code only?)
    - Relationship to `is sane` command
    - Examples

### 13. `stdf find` - Search for files matching criteria
- 🔄 **Has good filter list but needs syntax**
  - Has: List of searchable attributes
  - Needs:
    - Query expression syntax (SQL-like? JSON? Custom?)
    - Wildcard syntax
    - Combining multiple criteria (AND/OR logic)
    - Output format
    - Complete examples

### 14. `atdf` - Commands for ATDF format
- ❌ **Section header only, no content**
  - Needs: All ATDF-specific commands (if any)
  - Or note that ATDF is handled via `stdf to atdf`

---

## Priority Recommendations

### High Priority (Most Impact)
1. **`stdf tally sbins/hbins/bins`** - Critical for test analysis
2. **`stdf anonymize`** - Need to specify which fields to remove
3. **`stdf dump`** - Common debugging command
4. **`stdf to parquet/xlsx/pdf`** - Core export functionality
5. **`stdf find`** - Very useful, needs clear syntax

### Medium Priority
6. **`stdf show <field>`** - Generic field extraction
7. **`stdf repair`** - Useful but edge case
8. **`stdf combine`** - Useful for lot analysis
9. **`stdf check`** - vs `is sane` - consolidate?
10. **`stdf rename`** - Nice-to-have for organization

### Low Priority (Can defer)
11. **`stdf has`** - Partially covered by `is` commands
12. **ATDF section** - If it's just `to atdf`, can document there

---

## Specific Questions to Answer

### For `stdf show`:
1. What happens when a field is missing/null? (print "null"? empty string? skip line?)
2. In directory mode, what's the output format? One line per file?
3. Temperature: Which record(s) to check? Priority order if multiple sources?

### For `stdf tally bins`:
1. Does this show the relationship between hbins and sbins?
2. Or is it a combined table of both?
3. Should it show pass/fail counts per bin?

### For `stdf anonymize`:
1. Which fields to anonymize?
   - LOT_ID, SUBLOT_ID?
   - PART_TYP (device name)?
   - NODE_NAM (tester name)?
   - Operator fields?
   - Serial numbers?
2. Should test names be anonymized (Test_0001, Test_0002...)?
3. Should coordinates be preserved (for wafer map analysis)?

### For `stdf rename`:
1. `to hash`: Hash of entire file content or just LOT_ID+timestamp+device?
2. `to tt`: Exact format? `<NODE_NAM>-<START_T>.std`? What about special chars in NODE_NAM?

### For `stdf find`:
1. Syntax: `stdf find "tester=ATE* AND temperature=hot" <dir> -r`?
2. Or separate flags: `stdf find --tester ATE* --temperature hot <dir> -r`?
3. Output: Just filenames? Or table with matched attributes?

---

## Notes
- Many commands have both file and directory modes - establish consistent pattern
- Exit codes should be standardized across all commands
- Progress indicators for directory operations (especially with -r)
- Consider adding `--json` output flag for all commands (machine-readable)
- Consider adding `--verbose` flag for detailed output
- Should there be a `stdf batch <command>` for processing multiple files efficiently?
