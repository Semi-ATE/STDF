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
