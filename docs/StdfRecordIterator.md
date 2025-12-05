# StdfRecordIterator

An efficient iterator for reading STDF (Standard Test Data Format) files record by record, returning raw byte arrays including headers.

## Usage

### Basic Iteration

```rust
use stdf::StdfRecordIterator;

fn main() {
    let iter = StdfRecordIterator::new("test.std").unwrap();
    
    for result in iter {
        match result {
            Ok(record_bytes) => {
                // record_bytes contains the complete record including header
                println!("Record size: {} bytes", record_bytes.len());
                
                // First 4 bytes are the header:
                // [0-1]: Record length (U2, little-endian)
                // [2]:   Record type (U1)
                // [3]:   Record subtype (U1)
                let rec_type = record_bytes[2];
                let rec_subtype = record_bytes[3];
                println!("Type: {}, Subtype: {}", rec_type, rec_subtype);
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
```

### Creating from File or Buffer

```rust
// From file path (automatically handles .7z, .gz, .zip decompression)
let iter = StdfRecordIterator::new("test.std.gz")?;

// From pre-loaded buffer
let buffer = std::fs::read("test.std")?;
let iter = StdfRecordIterator::from_buffer(buffer)?;
```

### Filtering Records

```rust
use stdf::StdfRecordIterator;

fn main() {
    let iter = StdfRecordIterator::new("test.std").unwrap();
    
    for result in iter {
        let record_bytes = result.unwrap();
        let rec_type = record_bytes[2];
        let rec_subtype = record_bytes[3];
        
        // Filter for PRR records (type=5, subtype=20)
        if rec_type == 5 && rec_subtype == 20 {
            println!("Found PRR record: {} bytes", record_bytes.len());
            // Process PRR record...
        }
    }
}
```

## Record Type Reference

Common STDF record types:

| Type | Subtype | Name | Description |
|------|---------|------|-------------|
| 0 | 10 | FAR | File Attributes Record |
| 0 | 20 | ATR | Audit Trail Record |
| 1 | 10 | MIR | Master Information Record |
| 1 | 20 | MRR | Master Results Record |
| 1 | 30 | PCR | Part Count Record |
| 1 | 40 | HBR | Hardware Bin Record |
| 1 | 50 | SBR | Software Bin Record |
| 2 | 10 | WIR | Wafer Information Record |
| 2 | 20 | WRR | Wafer Results Record |
| 5 | 10 | PIR | Part Information Record |
| 5 | 20 | PRR | Part Results Record |
| 15 | 10 | PTR | Parametric Test Record |
| 15 | 20 | FTR | Functional Test Record |

## Examples

See the `examples/` directory:
- `record_iterator.rs` - Basic iteration example
- `filter_records.rs` - Advanced filtering with command-line options

Run examples:
```bash
cargo run --example record_iterator test.std
cargo run --example filter_records test.std 5 20  # Filter PRR records
cargo run --example filter_records test.std 1 10 --hex  # Show MIR with hex dump
```

### Parsing Records with the Factory

The `parse_record` factory function converts raw bytes into typed V4 records:

```rust
use stdf::{StdfRecordIterator, parse_record, V4};

fn main() {
    let iter = StdfRecordIterator::new("test.std").unwrap();
    let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result.unwrap();
        
        // Parse into typed V4 enum
        match parse_record(&record_bytes, endian) {
            Ok(V4::PRR(prr)) => {
                println!("Part {}: bin {}", prr.part_id, prr.hard_bin);
            },
            Ok(V4::PTR(ptr)) => {
                println!("Test {}: result {}", ptr.test_num, ptr.result);
            },
            Ok(_) => {}, // Other record types
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}
```

## Use Cases

1. **Record Filtering**: Extract specific record types from large files
2. **File Analysis**: Count and categorize records
3. **Data Extraction**: Pull raw records for custom processing
4. **File Rewriting**: Read, filter, and write modified STDF files
5. **Type-Safe Processing**: Use the factory to convert raw bytes to typed records
6. **Performance**: Process large files without loading entire file into memory

## Performance

The iterator uses memory-mapped files internally for efficient reading of large STDF files. The entire file is loaded once, but records are yielded on-demand as you iterate.
