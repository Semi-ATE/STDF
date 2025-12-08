# TODO List

## Testing
- [ ] Create a truncated STDF file for testing the `stdf is truncated` command
- [ ] Create a non-complete STDF file (missing MRR) for testing the `stdf is complete` command

## Not Yet Implemented Commands

### Count Commands
- [ ] `count parts <file>` - Count number of parts tested
- [ ] `count parts unique <file>` - Count unique parts excluding retests
- [ ] `count sbins <file>` - Count number of soft bins
- [ ] `count hbins <file>` - Count number of hard bins

### Tally Commands
- [ ] `tally records <file>` - Show tally of each record type
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
