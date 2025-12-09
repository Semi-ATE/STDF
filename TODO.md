# TODO List

## Testing
- [ ] Create a truncated STDF file for testing the `stdf is truncated` command
- [ ] Create a non-complete STDF file (missing MRR) for testing the `stdf is complete` command

## Not Yet Implemented Commands

### Count Commands
- [x] `count [record_types...] <path> [-r]` - Count specific or all records (COMPLETED)
- [x] `count records <path> [-r]` - Count total number of records (COMPLETED)
- [x] `count parts <path> [-r]` - Count number of parts with crash detection (COMPLETED)
- [x] `count tests <path> [-r]` - Count test records PTR+FTR+MPR (COMPLETED)
- [x] `count wafers <path> [-r]` - Count wafers with crash detection (COMPLETED)
- [ ] `count parts unique <path>` - Count unique parts excluding retests
- [ ] `count sbins <path> [-r]` - Count number of soft bins (needs proper implementation)
- [ ] `count hbins <path> [-r]` - Count number of hard bins (needs proper implementation)

### Tally Commands
- [x] `tally records <file>` - Show tally of each record type (COMPLETED)
- [ ] `tally heads <file>` - Show tally of test heads
- [ ] `tally sites <file>` - Show tally of sites
- [ ] `tally hbins <file>` - Show tally of hard bins
- [ ] `tally sbins <file>` - Show tally of soft bins

### Conversion Commands
- [ ] `to xlsx <file>` - Convert STDF to XLSX format
- [ ] `to hdf5 <file>` - Convert STDF to HDF5 format

## Code Cleanup
- [ ] Deprecate `StdfRecordIterator` in favor of `StdfRecordFPIterator`
  - Performance testing shows identical speed (~906 MB/s)
  - FP iterator provides file position with zero overhead
  - Update all internal code to use `StdfRecordFPIterator`
  - Add deprecation warning to `StdfRecordIterator`
  - Remove `StdfRecordIterator` in next major version

## Future Enhancements
- [ ] Array field handling in ascii() method (currently placeholders)
- [ ] Complete ATDF parsing implementation (atdf_parse_record is stub)

## Build & Distribution
- [ ] Set up GitHub Actions workflow for multi-platform wheel building
  - Build wheels for Windows, Linux (manylinux), macOS
  - Support Python 3.10, 3.11, 3.12
  - Use maturin for PyO3 cross-compilation
  - Upload artifacts and optionally publish to PyPI
  - Consider using cibuildwheel for comprehensive platform coverage
- [ ] Generate and package man pages for Linux
  - Add clap_mangen to build dependencies
  - Create build.rs to auto-generate man pages from clap CLI structure
  - Generate main man page: stdf.1 (user commands)
  - Consider subcommand man pages: stdf-show.1, stdf-dump.1, stdf-count.1, etc.
  - Create STDF file format man page: stdf.5 (file formats section)
  - Package man pages in .deb installer (/usr/share/man/man1/)
  - Compress man pages with gzip -9
  - Test with `man stdf`, `apropos stdf`, `whatis stdf`

## Testing & Quality Assurance
- [ ] Set up code coverage with Codecov or Coveralls
  - Install and configure cargo-llvm-cov for coverage generation
  - Create GitHub Actions workflow for coverage reporting
  - Target: 80%+ coverage (excellent), 70-80% (good), 60-70% (acceptable)
- [ ] Add coverage badge to README.md
- [ ] Expand test coverage:
  - [ ] Parser tests for all record types
  - [ ] Iterator tests (memory-mapped and streaming)
  - [ ] CLI command tests (execution and output validation)
  - [ ] Edge cases: truncated files, corrupted data, endianness variations
  - [ ] Integration tests with real STDF files from data/
- [ ] Set up additional quality badges:
  - [ ] CI/CD status badge
  - [ ] Codecov badge
  - [ ] Crates.io version and downloads badges
  - [ ] docs.rs documentation badge
  - [ ] License badges (MIT + Apache 2.0)
  - [ ] Security audit badge (cargo-audit)
  - [ ] Dependency status badge (deps.rs)
