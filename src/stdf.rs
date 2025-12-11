use std::process;
use std::collections::HashMap;

use clap::{Parser, Subcommand};
use stdf_lib::parsers::StdfParser;

/// Semi-ATE STDF Tool - Fast STDF file parser and analyzer
#[derive(Parser)]
#[command(name = "stdf")]
#[command(version)]
#[command(about = "Standard Test Data Format (STDF) file parser and analyzer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Count records, parts, tests, or wafers
    Count {
        #[command(subcommand)]
        subcommand: CountCommands,
    },
    /// Show tallies of records, heads, sites, bins
    Tally {
        #[command(subcommand)]
        subcommand: TallyCommands,
    },
    /// Display specific field values from STDF records or list all record types
    Show {
        #[command(subcommand)]
        subcommand: ShowCommands,
    },
    /// Dump records from STDF file
    Dump {
        /// Specific record types to dump (optional, e.g., MIR PRR)
        #[arg(num_args = 0..)]
        record_types: Vec<String>,
        /// STDF file path
        file: String,
    },
    /// Check file properties (ft, ws, hot, cold, room, truncated, complete)
    Is {
        /// Test type: ft, ws, hot, cold, room, truncated, complete
        test_type: String,
        /// STDF file path
        file: String,
    },
    /// Get file endianness (LE or BE)
    Endian {
        /// STDF file path
        file: String,
    },
    /// Get lot ID
    Lot {
        /// STDF file path
        file: String,
    },
    /// Get tester name or type
    Tester {
        /// Get tester type instead of name
        #[arg(long)]
        type_only: bool,
        /// STDF file path
        file: String,
    },
    /// Get test temperature
    Temperature {
        /// Parse temperature as integer
        #[arg(long)]
        int: bool,
        /// STDF file path
        file: String,
    },
    /// Convert STDF file to another format
    To {
        /// Output format: xlsx, atdf, hdf5
        format: String,
        /// Force overwrite if output exists
        #[arg(short, long)]
        force: bool,
        /// STDF file path
        file: String,
    },
    /// Compress STDF file
    Compress {
        /// Use gzip compression
        #[arg(long, visible_alias = "gz", conflicts_with_all = ["zlib", "bzip2", "xz", "zstd", "lz4"])]
        gzip: bool,
        /// Use zlib compression
        #[arg(long, visible_alias = "z", conflicts_with_all = ["gzip", "bzip2", "xz", "zstd", "lz4"])]
        zlib: bool,
        /// Use bzip2 compression
        #[arg(long, visible_alias = "bz2", conflicts_with_all = ["gzip", "zlib", "xz", "zstd", "lz4"])]
        bzip2: bool,
        /// Use xz compression
        #[arg(long, visible_alias = "lzma", conflicts_with_all = ["gzip", "zlib", "bzip2", "zstd", "lz4"])]
        xz: bool,
        /// Use zstd compression (default)
        #[arg(long, visible_alias = "zst", conflicts_with_all = ["gzip", "zlib", "bzip2", "xz", "lz4"])]
        zstd: bool,
        /// Use lz4 compression
        #[arg(long, conflicts_with_all = ["gzip", "zlib", "bzip2", "xz", "zstd"])]
        lz4: bool,
        /// Verify compression integrity with SHA-256
        #[arg(short, long)]
        verify: bool,
        /// Show progress bar
        #[arg(short, long)]
        progress: bool,
        /// Force overwrite if output file exists
        #[arg(short, long)]
        force: bool,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
        /// STDF file or directory path
        path: String,
    },
    /// Decompress STDF file
    Decompress {
        /// Verify decompression integrity with SHA-256
        #[arg(short, long, global = true)]
        verify: bool,
        /// Show progress bar
        #[arg(short, long)]
        progress: bool,
        /// Force overwrite if output file exists
        #[arg(short, long)]
        force: bool,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
        /// STDF file or directory path
        path: String,
    },
}

#[derive(Subcommand)]
enum CountCommands {
    /// Count total number of records
    Records {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
    /// Count parts with crash detection
    Parts {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
        /// Count unique parts excluding retests
        #[arg(long)]
        unique: bool,
    },
    /// Count test records (PTR + FTR + MPR)
    Tests {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
    /// Count wafers with crash detection
    Wafers {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
    /// Count soft bins (not yet implemented)
    Sbins {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
    /// Count hard bins (not yet implemented)
    Hbins {
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
    /// Count specific record types
    Rectypes {
        /// Record types to count (e.g., PTR FTR)
        #[arg(num_args = 0..)]
        record_types: Vec<String>,
        /// Path to STDF file or directory
        path: String,
        /// Recursively process directories
        #[arg(short, long)]
        recursive: bool,
    },
}

#[derive(Subcommand)]
enum ShowCommands {
    /// List all STDF V4 record types
    Records,
    /// List supported compression formats
    Supported {
        #[command(subcommand)]
        subcommand: ShowSupportedCommands,
    },
    /// Display field values from a specific record type
    Field {
        /// Record type (e.g., MIR, PRR, PTR)
        record_type: String,
        /// Field name or 'fields' to list available fields
        field: String,
        /// STDF file path
        file: String,
        /// Limit number of results (e.g., -10 for first 10)
        #[arg(short = 'n', long)]
        limit: Option<i32>,
    },
}

#[derive(Subcommand)]
enum ShowSupportedCommands {
    /// List supported compression formats
    Compressions,
}

#[derive(Subcommand)]
enum TallyCommands {
    /// Show tally of each record type
    Records {
        /// STDF file path
        file: String,
    },
    /// Show tally of test heads (not yet implemented)
    Heads {
        /// STDF file path
        file: String,
    },
    /// Show tally of sites (not yet implemented)
    Sites {
        /// STDF file path
        file: String,
    },
    /// Show tally of hard bins (not yet implemented)
    Hbins {
        /// STDF file path
        file: String,
    },
    /// Show tally of soft bins (not yet implemented)
    Sbins {
        /// STDF file path
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Count { subcommand } => {
            match subcommand {
                CountCommands::Records { path, recursive } => {
                    handle_count_for_path(&path, recursive, CountMode::All);
                }
                CountCommands::Parts { path, recursive, unique } => {
                    if unique {
                        eprintln!("'count parts --unique' not yet implemented");
                        process::exit(1);
                    }
                    handle_count_for_path(&path, recursive, CountMode::Parts);
                }
                CountCommands::Tests { path, recursive } => {
                    let record_types = vec!["PTR".to_string(), "FTR".to_string(), "MPR".to_string()];
                    handle_count_for_path(&path, recursive, CountMode::Specific(record_types));
                }
                CountCommands::Wafers { path, recursive } => {
                    handle_count_for_path(&path, recursive, CountMode::Wafers);
                }
                CountCommands::Sbins { .. } => {
                    eprintln!("'count sbins' not yet implemented");
                    process::exit(1);
                }
                CountCommands::Hbins { .. } => {
                    eprintln!("'count hbins' not yet implemented");
                    process::exit(1);
                }
                CountCommands::Rectypes { record_types, path, recursive } => {
                    // Validate all record types
                    for rt in &record_types {
                        if get_record_type_codes(rt).is_none() {
                            eprintln!("Error: Unknown record type '{}'", rt);
                            eprintln!("Available records:");
                            eprintln!("FAR, ATR, MIR, MRR, PCR, HBR, SBR, PMR, PGR, PLR, RDR, SDR,");
                            eprintln!("WIR, WRR, WCR, PIR, PRR, TSR, PTR, MPR, FTR, BPS, EPS, GDR, DTR");
                            process::exit(1);
                        }
                    }
                    
                    let mode = if record_types.is_empty() {
                        CountMode::All
                    } else {
                        CountMode::Specific(record_types)
                    };
                    
                    handle_count_for_path(&path, recursive, mode);
                }
            }
        }
        Commands::Tally { subcommand } => {
            match subcommand {
                TallyCommands::Records { file } => {
                    match tally_records(&file) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                TallyCommands::Heads { .. } => {
                    eprintln!("'tally heads' not yet implemented");
                    process::exit(1);
                }
                TallyCommands::Sites { .. } => {
                    eprintln!("'tally sites' not yet implemented");
                    process::exit(1);
                }
                TallyCommands::Hbins { .. } => {
                    eprintln!("'tally hbins' not yet implemented");
                    process::exit(1);
                }
                TallyCommands::Sbins { .. } => {
                    eprintln!("'tally sbins' not yet implemented");
                    process::exit(1);
                }
            }
        }
        Commands::Dump { record_types, file } => {
            // Uppercase record types and remove duplicates
            let record_types: Vec<String> = record_types
                .iter()
                .map(|s| s.to_uppercase())
                .collect();
            
            let mut unique_types: Vec<String> = Vec::new();
            for rt in record_types {
                if !unique_types.contains(&rt) {
                    unique_types.push(rt);
                }
            }
            
            match dump_stdf(&file, &unique_types) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        Commands::To { format, force, file } => {
            let format = format.to_lowercase();
            
            match format.as_str() {
                "xlsx" => {
                    eprintln!("XLSX conversion not yet implemented");
                    process::exit(1);
                }
                "atdf" => {
                    match convert_to_atdf(&file, force) {
                        Ok(output_path) => {
                            println!("Successfully converted to: {}", output_path);
                        },
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                "hdf5" => {
                    eprintln!("HDF5 conversion not yet implemented");
                    process::exit(1);
                }
                _ => {
                    eprintln!("Error: Unknown format '{}'. Use 'xlsx', 'atdf', or 'hdf5'", format);
                    process::exit(1);
                }
            }
        }
        Commands::Is { test_type, file } => {
            let test_type = test_type.to_lowercase();
            
            match test_type.as_str() {
                "ft" => {
                    match check_is_ft(&file) {
                        Ok(true) => process::exit(0),  // Is FT
                        Ok(false) => process::exit(1), // Not FT (is WS)
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "ws" => {
                    match check_is_ws(&file) {
                        Ok(true) => process::exit(0),  // Is WS
                        Ok(false) => process::exit(1), // Not WS (is FT)
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "hot" => {
                    match check_is_hot(&file) {
                        Ok(true) => process::exit(0),  // Is hot (>50°C)
                        Ok(false) => process::exit(1), // Not hot
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "cold" => {
                    match check_is_cold(&file) {
                        Ok(true) => process::exit(0),  // Is cold (<10°C)
                        Ok(false) => process::exit(1), // Not cold
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "room" => {
                    match check_is_room(&file) {
                        Ok(true) => process::exit(0),  // Is room (10-50°C)
                        Ok(false) => process::exit(1), // Not room
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "truncated" => {
                    match check_is_truncated(&file) {
                        Ok(true) => process::exit(0),  // Is truncated
                        Ok(false) => process::exit(1), // Not truncated
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "complete" => {
                    match check_is_complete(&file) {
                        Ok(true) => process::exit(0),  // Is complete (ends with MRR)
                        Ok(false) => process::exit(1), // Not complete
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                _ => {
                    eprintln!("Error: Invalid test type '{}'. Use 'ft', 'ws', 'hot', 'cold', 'room', 'truncated', or 'complete'", test_type);
                    process::exit(1);
                }
            }
        }
        Commands::Endian { file } => {
            match get_endian(&file) {
                Ok(endian) => {
                    println!("{}", endian);
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Lot { file } => {
            match get_lot_id(&file) {
                Ok(lot_id) => {
                    println!("{}", lot_id);
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Tester { type_only, file } => {
            if type_only {
                match get_tester_type(&file) {
                    Ok(tester_type) => {
                        println!("{}", tester_type);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        process::exit(1);
                    }
                }
            } else {
                match get_tester(&file) {
                    Ok(tester) => {
                        println!("{}", tester);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Commands::Temperature { int, file } => {
            match get_temperature(&file) {
                Ok(temp) => {
                    if int {
                        match parse_temperature_as_int(&temp) {
                            Ok(value) => println!("{}", value),
                            Err(e) => {
                                eprintln!("Error parsing temperature as int: {}", e);
                                process::exit(1);
                            }
                        }
                    } else {
                        println!("{}", temp);
                    }
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Show { subcommand } => {
            match subcommand {
                ShowCommands::Records => {
                    print_all_record_types();
                }
                ShowCommands::Supported { subcommand } => {
                    match subcommand {
                        ShowSupportedCommands::Compressions => {
                            println!("gzip/zlib : .gz, .z");
                            println!("bzip2 : .bz2");
                            println!("xz/LZMA : .xz");
                            println!("zstd : .zst (default)");
                            println!("lz4 : .lz4");
                        }
                    }
                }
                ShowCommands::Field { record_type, field, file, limit } => {
                    handle_show_command_new(&record_type, &field, &file, limit);
                }
            }
        }
        Commands::Compress { gzip, zlib, bzip2, xz, zstd, lz4, verify, progress, force, recursive, path } => {
            let algo = if gzip {
                "gzip"
            } else if zlib {
                "zlib"
            } else if bzip2 {
                "bzip2"
            } else if xz {
                "xz"
            } else if lz4 {
                "lz4"
            } else {
                "zstd" // default
            };
            
            let path_obj = std::path::Path::new(&path);
            if path_obj.is_dir() {
                match process_directory_compress(path_obj, algo, verify, progress, force, recursive) {
                    Ok(_) => process::exit(0),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                match compress_file(&path, algo, verify, progress, force) {
                    Ok(_output_path) => {
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
        Commands::Decompress { verify, progress, force, recursive, path } => {
            let path_obj = std::path::Path::new(&path);
            if path_obj.is_dir() {
                match process_directory_decompress(path_obj, verify, progress, force, recursive) {
                    Ok(_) => process::exit(0),
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                match decompress_file(&path, verify, progress, force) {
                    Ok(_output_path) => {
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            }
        }
    }
}

fn print_all_record_types() {
    println!("STDF V4 Record Types:");
    println!();
    println!("Information Records:");
    println!("  FAR  - File Attributes Record");
    println!("  ATR  - Audit Trail Record");
    println!("  MIR  - Master Information Record");
    println!("  MRR  - Master Results Record");
    println!("  PCR  - Part Count Record");
    println!("  HBR  - Hardware Bin Record");
    println!("  SBR  - Software Bin Record");
    println!("  PMR  - Pin Map Record");
    println!("  PGR  - Pin Group Record");
    println!("  PLR  - Pin List Record");
    println!("  RDR  - Retest Data Record");
    println!("  SDR  - Site Description Record");
    println!();
    println!("Per Wafer Records:");
    println!("  WIR  - Wafer Information Record");
    println!("  WRR  - Wafer Results Record");
    println!("  WCR  - Wafer Configuration Record");
    println!();
    println!("Per Part Records:");
    println!("  PIR  - Part Information Record");
    println!("  PRR  - Part Results Record");
    println!();
    println!("Per Test Records:");
    println!("  TSR  - Test Synopsis Record");
    println!("  PTR  - Parametric Test Record");
    println!("  MPR  - Multiple-Result Parametric Record");
    println!("  FTR  - Functional Test Record");
    println!();
    println!("Generic Data:");
    println!("  BPS  - Begin Program Section");
    println!("  EPS  - End Program Section");
    println!("  GDR  - Generic Data Record");
    println!("  DTR  - Datalog Text Record");
    println!();
    println!("Use 'stdf tally records <file>' to see which records are in a specific file.");
}

// New show command handler for clap-based CLI
fn handle_show_command_new(record_type: &str, field: &str, file: &str, limit: Option<i32>) {
    // Build args vector in the old format for compatibility with existing handle_show_command
    let mut args = vec![
        "stdf".to_string(),
        "show".to_string(),
        record_type.to_string(),
        field.to_string(),
        file.to_string(),
    ];
    
    if let Some(n) = limit {
        args.push(format!("-{}", n));
    }
    
    handle_show_command(&args);
}

fn count_records(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    let count = parser.count_records(filename)?;
    println!("{}", count);
    Ok(())
}

fn count_all_records(filename: &str) -> Result<usize, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    
    let iter = StdfRecordIterator::new(filename)?;
    let mut count = 0;
    
    for result in iter {
        let _ = result?; // Just verify the record is readable
        count += 1;
    }
    
    Ok(count)
}

fn count_specific_records(filename: &str, record_types: &[String]) -> Result<usize, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    
    // Get the target record type codes for fast filtering
    let mut target_codes: Vec<(u8, u8)> = Vec::new();
    for rt in record_types {
        if let Some(codes) = get_record_type_codes(rt) {
            target_codes.push(codes);
        }
    }
    
    let iter = StdfRecordIterator::new(filename)?;
    let mut count = 0;
    
    for result in iter {
        let record_bytes = result?;
        
        // Quick check: see if this record matches any of our target types
        if record_bytes.len() >= 4 {
            let rec_typ = record_bytes[2];
            let rec_sub = record_bytes[3];
            
            for &(target_typ, target_sub) in &target_codes {
                if rec_typ == target_typ && rec_sub == target_sub {
                    count += 1;
                    break;
                }
            }
        }
    }
    
    Ok(count)
}

fn count_parts(filename: &str) -> Result<f64, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    
    let iter = StdfRecordIterator::new(filename)?;
    let mut pir_count = 0;
    let mut prr_count = 0;
    
    // PIR codes: REC_TYP=5, REC_SUB=10
    // PRR codes: REC_TYP=5, REC_SUB=20
    
    for result in iter {
        let record_bytes = result?;
        
        if record_bytes.len() >= 4 {
            let rec_typ = record_bytes[2];
            let rec_sub = record_bytes[3];
            
            if rec_typ == 5 {
                if rec_sub == 10 {
                    pir_count += 1;
                } else if rec_sub == 20 {
                    prr_count += 1;
                }
            }
        }
    }
    
    // Calculate remainder: Remainder = 1 / (PIRs - PRRs)
    let diff = pir_count as i32 - prr_count as i32;
    
    if diff == 0 {
        // Perfect match, no crash
        Ok(pir_count as f64)
    } else if diff > 0 {
        // More PIRs than PRRs - system crashed during test
        let remainder = 1.0 / (diff as f64);
        Ok(pir_count as f64 + remainder)
    } else {
        // More PRRs than PIRs - this is unusual, data corruption?
        // Return negative to indicate the issue
        let remainder = 1.0 / (diff.abs() as f64);
        Ok(pir_count as f64 - remainder)
    }
}

fn count_wafers(filename: &str) -> Result<f64, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    
    let iter = StdfRecordIterator::new(filename)?;
    let mut wir_count = 0;
    let mut wrr_count = 0;
    
    // WIR codes: REC_TYP=2, REC_SUB=10
    // WRR codes: REC_TYP=2, REC_SUB=20
    
    for result in iter {
        let record_bytes = result?;
        
        if record_bytes.len() >= 4 {
            let rec_typ = record_bytes[2];
            let rec_sub = record_bytes[3];
            
            if rec_typ == 2 {
                if rec_sub == 10 {
                    wir_count += 1;
                } else if rec_sub == 20 {
                    wrr_count += 1;
                }
            }
        }
    }
    
    // Calculate difference: WIRs - WRRs
    let diff = wir_count as i32 - wrr_count as i32;
    
    if diff == 0 {
        // All wafers completed
        Ok(wir_count as f64)
    } else if diff == 1 {
        // One incomplete wafer
        Ok(wir_count as f64 + 0.5)
    } else {
        // Unexpected case - multiple incomplete wafers?
        Ok(wir_count as f64 + 0.5)
    }
}

fn tally_records(filename: &str) -> Result<(), std::io::Error> {
    use stdf_lib::{StdfRecordIterator, stdf_parse_record, V4};
    
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    let mut tally: HashMap<String, usize> = HashMap::new();
    
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(record) => {
                let record_type = match record {
                    V4::FAR(_) => "FAR",
                    V4::MIR(_) => "MIR",
                    V4::MRR(_) => "MRR",
                    V4::PCR(_) => "PCR",
                    V4::HBR(_) => "HBR",
                    V4::SBR(_) => "SBR",
                    V4::PMR(_) => "PMR",
                    V4::PGR(_) => "PGR",
                    V4::PLR(_) => "PLR",
                    V4::RDR(_) => "RDR",
                    V4::SDR(_) => "SDR",
                    V4::WIR(_) => "WIR",
                    V4::WRR(_) => "WRR",
                    V4::WCR(_) => "WCR",
                    V4::PIR(_) => "PIR",
                    V4::PRR(_) => "PRR",
                    V4::TSR(_) => "TSR",
                    V4::PTR(_) => "PTR",
                    V4::MPR(_) => "MPR",
                    V4::FTR(_) => "FTR",
                    V4::ATR(_) => "ATR",
                    V4::BPS(_) => "BPS",
                    V4::EPS(_) => "EPS",
                    V4::GDR(_) => "GDR",
                    V4::DTR(_) => "DTR",
                    V4::Unknown(_) => "Unknown",
                    V4::Invalid(_) => "Invalid",
                };
                *tally.entry(record_type.to_string()).or_insert(0) += 1;
            }
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse record: {:?}", e)
                ));
            }
        }
    }
    
    // Print results sorted by record type name
    let mut sorted_types: Vec<_> = tally.iter().collect();
    sorted_types.sort_by_key(|&(k, _)| k);
    
    for (record_type, count) in sorted_types {
        println!("{}: {}", record_type, count);
    }
    
    Ok(())
}

enum CountMode {
    All,
    Specific(Vec<String>),
    Parts,
    Wafers,
}

fn handle_count_for_path(path: &str, recursive: bool, mode: CountMode) {
    use std::path::Path;
    
    let path_obj = Path::new(path);
    
    if path_obj.is_file() {
        // Single file
        match count_file(path, &mode) {
            Ok(result) => println!("{}", result),
            Err(e) => {
                eprintln!("Error processing {}: {:?}", path, e);
                process::exit(1);
            }
        }
    } else if path_obj.is_dir() {
        // Directory - process all STDF files
        match process_directory(path, recursive, &mode) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error processing directory: {:?}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Error: Path '{}' does not exist", path);
        process::exit(1);
    }
}

fn count_file(filename: &str, mode: &CountMode) -> Result<String, std::io::Error> {
    match mode {
        CountMode::All => {
            let count = count_all_records(filename)?;
            Ok(count.to_string())
        }
        CountMode::Specific(record_types) => {
            let count = count_specific_records(filename, record_types)?;
            Ok(count.to_string())
        }
        CountMode::Parts => {
            let count = count_parts(filename)?;
            Ok(format!("{}", count))
        }
        CountMode::Wafers => {
            let count = count_wafers(filename)?;
            Ok(format!("{}", count))
        }
    }
}

fn process_directory(dir_path: &str, recursive: bool, mode: &CountMode) -> Result<(), std::io::Error> {
    use std::fs;
    use std::path::Path;
    
    let entries = fs::read_dir(dir_path)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            // Check if it's likely an STDF file (common extensions)
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "std" || ext_str == "stdf" {
                    let path_str = path.to_string_lossy();
                    match count_file(&path_str, mode) {
                        Ok(result) => println!("{} : {}", path_str, result),
                        Err(e) => eprintln!("Error processing {}: {:?}", path_str, e),
                    }
                }
            }
        } else if path.is_dir() && recursive {
            // Recursively process subdirectories
            let subdir = path.to_string_lossy();
            let _ = process_directory(&subdir, recursive, mode);
        }
    }
    
    Ok(())
}

fn dump_stdf(filename: &str, record_types: &[String]) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    parser.dump_file(filename, record_types)
}

fn check_is_ft(filename: &str) -> Result<bool, std::io::Error> {
    let parser = StdfParser::new();
    let has_wir = parser.has_wir_record(filename)?;
    Ok(!has_wir) // FT if no WIR found
}

fn check_is_ws(filename: &str) -> Result<bool, std::io::Error> {
    let parser = StdfParser::new();
    let has_wir = parser.has_wir_record(filename)?;
    Ok(has_wir) // WS if WIR found
}

fn check_is_hot(filename: &str) -> Result<bool, std::io::Error> {
    let temp_str = get_temperature(filename)?;
    match parse_temperature_as_int(&temp_str) {
        Ok(temp) => Ok(temp > 50),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse temperature: {}", e)
        ))
    }
}

fn check_is_cold(filename: &str) -> Result<bool, std::io::Error> {
    let temp_str = get_temperature(filename)?;
    match parse_temperature_as_int(&temp_str) {
        Ok(temp) => Ok(temp < 10),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse temperature: {}", e)
        ))
    }
}

fn check_is_room(filename: &str) -> Result<bool, std::io::Error> {
    let temp_str = get_temperature(filename)?;
    match parse_temperature_as_int(&temp_str) {
        Ok(temp) => Ok(temp >= 10 && temp <= 50),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Failed to parse temperature: {}", e)
        ))
    }
}

fn check_is_truncated(filename: &str) -> Result<bool, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    let iter = StdfRecordIterator::new(filename)?;
    
    // Iterate through all records and check if any fail to parse
    for result in iter {
        if result.is_err() {
            // If we hit an error reading a record, file is truncated
            return Ok(true);
        }
    }
    
    // All records parsed successfully, file is not truncated
    Ok(false)
}

fn check_is_complete(filename: &str) -> Result<bool, std::io::Error> {
    use stdf_lib::{StdfRecordIterator, stdf_parse_record, V4};
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    
    let mut last_is_mrr = false;
    
    // Iterate through all records to find the last one
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(V4::MRR(_)) => last_is_mrr = true,
            Ok(_) => last_is_mrr = false,
            Err(_) => return Ok(false), // Parse error means not complete
        }
    }
    
    Ok(last_is_mrr)
}

fn get_lot_id(filename: &str) -> Result<String, std::io::Error> {
    use stdf_lib::{StdfStreamingIterator, stdf_parse_record, V4};
    let iter = StdfStreamingIterator::new(filename)?;
    let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(V4::MIR(mir)) => {
                let lot_id = String::from_utf8_lossy(mir.lot_id.0).to_string();
                return Ok(lot_id);
            }
            _ => continue,
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No MIR record found in file"
    ))
}

fn get_tester(filename: &str) -> Result<String, std::io::Error> {
    use stdf_lib::{StdfStreamingIterator, stdf_parse_record, V4};
    let iter = StdfStreamingIterator::new(filename)?;
    let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(V4::MIR(mir)) => {
                let tester = String::from_utf8_lossy(mir.node_nam.0).to_string();
                return Ok(tester);
            }
            _ => continue,
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No MIR record found in file"
    ))
}

fn get_tester_type(filename: &str) -> Result<String, std::io::Error> {
    use stdf_lib::{StdfStreamingIterator, stdf_parse_record, V4};
    let iter = StdfStreamingIterator::new(filename)?;
    let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(V4::MIR(mir)) => {
                let tester_type = String::from_utf8_lossy(mir.tstr_typ.0).to_string();
                return Ok(tester_type);
            }
            _ => continue,
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No MIR record found in file"
    ))
}

fn parse_temperature_as_int(temp_str: &str) -> Result<i32, String> {
    // Try to extract a numeric value from the temperature string
    let trimmed = temp_str.trim();
    
    // Handle common cases:
    // "R" or "ROOM" -> 25
    // "C" or "COLD" -> -40
    // "H" or "HOT" -> 150
    // "25" -> 25
    // "+25" -> 25
    // "170" -> 170
    // "25C" or "25 C" -> 25
    
    if trimmed.is_empty() {
        return Ok(25); // Empty string defaults to room temperature
    }
    
    let upper = trimmed.to_uppercase();
    
    // Handle "R" or "ROOM" as room temperature (25°C)
    if upper == "R" || upper == "ROOM" {
        return Ok(25);
    }
    
    // Handle "C" or "COLD" as cold temperature (-40°C)
    if upper == "C" || upper == "COLD" {
        return Ok(-40);
    }
    
    // Handle "H" or "HOT" as hot temperature (150°C)
    if upper == "H" || upper == "HOT" {
        return Ok(150);
    }
    
    // Check if Fahrenheit is indicated
    let is_fahrenheit = upper.contains("°F") || upper.ends_with('F');
    
    // Remove common suffixes like °C, °F (but not standalone C)
    let cleaned = upper
        .replace("°C", "")
        .replace("°F", "")
        .trim()
        .to_string();
    
    // Only remove trailing C or F if there are other characters
    let cleaned = if cleaned.len() > 1 {
        cleaned.trim_end_matches('C').trim_end_matches('F').trim().to_string()
    } else {
        cleaned
    };
    
    // Try to parse as float first
    let temp_value = cleaned.parse::<f64>()
        .map_err(|_| format!("Cannot parse '{}' as temperature", temp_str))?;
    
    // Convert Fahrenheit to Celsius if needed: C = (F - 32) * 5/9
    let celsius = if is_fahrenheit {
        (temp_value - 32.0) * 5.0 / 9.0
    } else {
        temp_value
    };
    
    Ok(celsius.round() as i32)
}

fn get_temperature(filename: &str) -> Result<String, std::io::Error> {
    use stdf_lib::{StdfStreamingIterator, stdf_parse_record, V4};
    let iter = StdfStreamingIterator::new(filename)?;
    let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result?;
        match stdf_parse_record(&record_bytes, endian) {
            Ok(V4::MIR(mir)) => {
                let temp = String::from_utf8_lossy(mir.tst_temp.0).to_string();
                return Ok(temp);
            }
            _ => continue,
        }
    }
    
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No MIR record found in file"
    ))
}

fn get_endian(filename: &str) -> Result<String, std::io::Error> {
    use stdf_lib::StdfRecordIterator;
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    Ok(match endian {
        byte::ctx::Endian::Little => "LE".to_string(),
        byte::ctx::Endian::Big => "BE".to_string(),
    })
}

fn convert_to_atdf(filename: &str, force: bool) -> Result<String, std::io::Error> {
    use stdf_lib::{StdfRecordIterator, stdf_parse_record, V4};
    use std::io::Write;
    use std::path::Path;
    use std::fs::File;
    
    // Generate output filename by replacing extension with .atdf
    let input_path = Path::new(filename);
    let output_path = input_path.with_extension("atdf");
    
    // Check if output file already exists
    if output_path.exists() && !force {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("Output file '{}' already exists. Use -f to overwrite.", output_path.display())
        ));
    }
    
    // Open input file and get iterator
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    
    // Create output file
    let mut output_file = File::create(&output_path)?;
    
    // Convert each record
    for (i, raw_record) in iter.enumerate() {
        let raw_record = raw_record?;
        let record = stdf_parse_record(&raw_record, endian)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Failed to parse record {}: {:?}", i, e)))?;
        
        let ascii_line = match record {
            V4::FAR(r) => r.ascii(),
            V4::MIR(r) => r.ascii(),
            V4::MRR(r) => r.ascii(),
            V4::PCR(r) => r.ascii(),
            V4::HBR(r) => r.ascii(),
            V4::SBR(r) => r.ascii(),
            V4::PMR(r) => r.ascii(),
            V4::PGR(r) => r.ascii(),
            V4::PLR(r) => r.ascii(),
            V4::RDR(r) => r.ascii(),
            V4::SDR(r) => r.ascii(),
            V4::WIR(r) => r.ascii(),
            V4::WRR(r) => r.ascii(),
            V4::WCR(r) => r.ascii(),
            V4::PIR(r) => r.ascii(),
            V4::PRR(r) => r.ascii(),
            V4::TSR(r) => r.ascii(),
            V4::PTR(r) => r.ascii(),
            V4::MPR(r) => r.ascii(),
            V4::FTR(r) => r.ascii(),
            V4::ATR(r) => r.ascii(),
            V4::BPS(r) => r.ascii(),
            V4::EPS(r) => r.ascii(),
            V4::GDR(r) => r.ascii(),
            V4::DTR(r) => r.ascii(),
            V4::Unknown(_) => {
                eprintln!("Warning: Skipping unknown record at index {}", i);
                continue;
            }
            V4::Invalid(_) => {
                eprintln!("Warning: Skipping invalid record at index {}", i);
                continue;
            }
        };
        
        writeln!(output_file, "{}", ascii_line)?;
    }
    
    Ok(output_path.to_string_lossy().to_string())
}

fn handle_show_command(args: &[String]) {
    let subcommand = &args[2];
    
    match subcommand.to_uppercase().as_str() {
        "ENDIAN" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show endian <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match get_endian(filename) {
                Ok(endian) => println!("{}", endian),
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "TEMPERATURE" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show temperature <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match get_temperature(filename) {
                Ok(temp) => println!("{}", temp),
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "LOT" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show lot <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match get_lot_id(filename) {
                Ok(lot_id) => println!("{}", lot_id),
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "TESTER" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument or 'type' keyword");
                eprintln!("Usage: {} show tester <file> or {} show tester type <file>", args[0], args[0]);
                process::exit(1);
            }
            
            if args.len() >= 5 && args[3].to_uppercase() == "TYPE" {
                let filename = &args[4];
                match get_tester_type(filename) {
                    Ok(tester_type) => println!("{}", tester_type),
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        process::exit(1);
                    }
                }
            } else {
                let filename = &args[3];
                match get_tester(filename) {
                    Ok(tester) => println!("{}", tester),
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        process::exit(1);
                    }
                }
            }
        }
        "SUBLOT" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show sublot <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match show_field("MIR", "SBLOT_ID", filename, None) {
                Ok(values) => {
                    if let Some(value) = values.first() {
                        println!("{}", value);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "DEVICE" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show device <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match show_field("MIR", "PART_TYP", filename, None) {
                Ok(values) => {
                    if let Some(value) = values.first() {
                        println!("{}", value);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "VERSION" => {
            if args.len() < 4 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} show version <file>", args[0]);
                process::exit(1);
            }
            let filename = &args[3];
            match show_field("FAR", "STDF_VER", filename, None) {
                Ok(values) => {
                    if let Some(value) = values.first() {
                        println!("{}", value);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "RECORDS" => {
            println!("FAR\nATR\nMIR\nMRR\nPCR\nHBR\nSBR\nPMR\nPGR\nPLR\nRDR\nSDR\nWIR\nWRR\nWCR\nPIR\nPRR\nTSR\nPTR\nMPR\nFTR\nBPS\nEPS\nGDR\nDTR");
        }
        "SUPPORTED" => {
            if args.len() >= 4 && args[3].to_uppercase() == "COMPRESSIONS" {
                println!("gzip/zlib : .gz, .z");
                println!("bzip2 : .bz2");
                println!("xz/LZMA : .xz");
                println!("zstd : .zst (default)");
                println!("lz4 : .lz4");
            } else {
                eprintln!("Error: Unknown 'show supported' subcommand");
                eprintln!("Usage: {} show supported compressions", args[0]);
                process::exit(1);
            }
        }
        _ => {
            // Check if it's a generic record/field query
            if args.len() >= 4 {
                let record_type = &args[2];
                
                // Check if it's "show <RECORD> fields"
                if args.len() == 4 && args[3].to_uppercase() == "FIELDS" {
                    match get_record_fields(record_type) {
                        Ok(fields) => {
                            for field in fields {
                                println!("{}", field);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    }
                    return;
                }
                
                // Otherwise it's: show <RECORD> <FIELD> <file> [-n]
                if args.len() >= 5 {
                    let field_name = &args[3];
                    let filename = &args[4];
                    let limit = if args.len() >= 6 && args[5].starts_with('-') {
                        match args[5][1..].parse::<i32>() {
                            Ok(n) => Some(n),
                            Err(_) => {
                                eprintln!("Error: Invalid limit value '{}'", args[5]);
                                process::exit(1);
                            }
                        }
                    } else {
                        None
                    };
                    
                    match show_field(record_type, field_name, filename, limit) {
                        Ok(values) => {
                            if values.is_empty() {
                                // Record type not found
                            } else if values.len() == 1 {
                                println!("{}", values[0]);
                            } else {
                                println!("{}", values.join(", "));
                            }
                        }
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    eprintln!("Error: Invalid 'show' command");
                    eprintln!("Usage: {} show <RECORD> fields | {} show <RECORD> <FIELD> <file> [-n]", args[0], args[0]);
                    process::exit(1);
                }
            } else {
                eprintln!("Error: Invalid 'show' command");
                eprintln!("Usage: {} show <subcommand> [args...]", args[0]);
                process::exit(1);
            }
        }
    }
}

fn get_record_type_codes(record_type: &str) -> Option<(u8, u8)> {
    match record_type.to_uppercase().as_str() {
        "FAR" => Some((0, 10)),
        "ATR" => Some((0, 20)),
        "MIR" => Some((1, 10)),
        "MRR" => Some((1, 20)),
        "PCR" => Some((1, 30)),
        "HBR" => Some((1, 40)),
        "SBR" => Some((1, 50)),
        "PMR" => Some((1, 60)),
        "PGR" => Some((1, 62)),
        "PLR" => Some((1, 63)),
        "RDR" => Some((1, 70)),
        "SDR" => Some((1, 80)),
        "WIR" => Some((2, 10)),
        "WRR" => Some((2, 20)),
        "WCR" => Some((2, 30)),
        "PIR" => Some((5, 10)),
        "PRR" => Some((5, 20)),
        "TSR" => Some((10, 30)),
        "PTR" => Some((15, 10)),
        "MPR" => Some((15, 15)),
        "FTR" => Some((15, 20)),
        "BPS" => Some((20, 10)),
        "EPS" => Some((20, 20)),
        "GDR" => Some((50, 10)),
        "DTR" => Some((50, 30)),
        _ => None,
    }
}

fn show_field(record_type: &str, field_name: &str, filename: &str, limit: Option<i32>) -> Result<Vec<String>, std::io::Error> {
    use stdf_lib::{StdfRecordIterator, StdfStreamingIterator, stdf_parse_record, V4};
    
    let mut results = Vec::new();
    let count_limit = limit.map(|n| n.abs() as usize);
    
    // Get the record type codes (REC_TYP, REC_SUB)
    let (target_rec_typ, target_rec_sub) = match get_record_type_codes(record_type) {
        Some(codes) => codes,
        None => {
            eprintln!("Error: Unknown record type '{}'. Available records:", record_type);
            eprintln!("FAR, ATR, MIR, MRR, PCR, HBR, SBR, PMR, PGR, PLR, RDR, SDR,");
            eprintln!("WIR, WRR, WCR, PIR, PRR, TSR, PTR, MPR, FTR, BPS, EPS, GDR, DTR");
            process::exit(1);
        }
    };
    
    // Determine optimal iteration strategy based on record location in file
    let record_name = record_type.to_uppercase();
    
    // Records that only occur once per file (singleton records)
    let is_singleton = matches!(record_name.as_str(), "FAR" | "ATR" | "MIR" | "MRR" | "PCR" | "HBR" | "SBR");
    let use_streaming = match record_name.as_str() {
        // Near beginning of file -> Always use streaming
        "FAR" | "ATR" | "MIR" | "RDR" | "SDR" | "WIR" => true,
        
        // Near end of file -> Always use memory-mapped
        "MRR" | "PCR" | "HBR" | "SBR" | "WRR" | "TSR" => false,
        
        // Middle of file -> Use streaming if small limit, memory-mapped otherwise
        "PMR" | "PGR" | "PLR" | "WCR" | "PIR" | "PRR" | "PTR" | "MPR" | "FTR" | "BPS" | "EPS" | "GDR" | "DTR" => {
            // Use streaming for small limits (< 1000 records) to avoid loading entire file
            count_limit.map_or(false, |n| n < 1000)
        }
        
        _ => false, // Default to memory-mapped for unknown types
    };
    
    if use_streaming {
        let iter = StdfStreamingIterator::new(filename)?;
        let endian = iter.endian();
        
        for result in iter {
            let record_bytes = result?;
            
            // Quick check: skip parsing if this isn't the record type we're looking for
            if record_bytes.len() >= 4 {
                let rec_typ = record_bytes[2];
                let rec_sub = record_bytes[3];
                
                if rec_typ != target_rec_typ || rec_sub != target_rec_sub {
                    continue;
                }
            }
            
            // Parse and extract field
            match stdf_parse_record(&record_bytes, endian) {
                Ok(record) => {
                    if let Some(value) = extract_field_value(&record, field_name) {
                        results.push(value);
                        
                        // Break early if we've reached the limit OR if this is a singleton record
                        if is_singleton || (count_limit.is_some() && results.len() >= count_limit.unwrap()) {
                            break;
                        }
                    }
                }
                Err(e) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Failed to parse record: {:?}", e)
                    ));
                }
            }
        }
    } else {
        // Use memory-mapped iterator for large scans
        let iter = StdfRecordIterator::new(filename)?;
        let endian = iter.endian();
    
    for result in iter {
        let record_bytes = result?;
        
        // Quick check: skip parsing if this isn't the record type we're looking for
        // Bytes [2] and [3] contain REC_TYP and REC_SUB
        if record_bytes.len() >= 4 {
            let rec_typ = record_bytes[2];
            let rec_sub = record_bytes[3];
            
            if rec_typ != target_rec_typ || rec_sub != target_rec_sub {
                continue; // Skip this record without parsing
            }
        }
        
        // Only parse records that match our target type (already filtered above)
        match stdf_parse_record(&record_bytes, endian) {
            Ok(record) => {
                // Extract the field value
                if let Some(value) = extract_field_value(&record, field_name) {
                    results.push(value);
                    
                    // Check if we should stop (limit reached OR singleton record found)
                    if is_singleton || (count_limit.is_some() && results.len() >= count_limit.unwrap()) {
                        break;
                    }
                }
            }
            Err(e) => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse record: {:?}", e)
                ));
            }
        }
    }
    }  // Close the else block
    
    Ok(results)
}

fn extract_field_value(record: &stdf_lib::V4, field_name: &str) -> Option<String> {
    use stdf_lib::V4;
    
    match record {
        V4::MIR(mir) => {
            match field_name.to_uppercase().as_str() {
                "LOT_ID" => Some(format!("{}", mir.lot_id)),
                "PART_TYP" => Some(format!("{}", mir.part_typ)),
                "NODE_NAM" => Some(format!("{}", mir.node_nam)),
                "TSTR_TYP" => Some(format!("{}", mir.tstr_typ)),
                "TST_TEMP" => Some(format!("{}", mir.tst_temp)),
                "SBLOT_ID" => Some(format!("{}", mir.sblot_id)),
                "USER_TXT" => Some(format!("{}", mir.user_txt)),
                _ => None,
            }
        }
        V4::FAR(far) => {
            match field_name.to_uppercase().as_str() {
                "STDF_VER" => Some(format!("{}", far.stdf_ver)),
                "CPU_TYPE" => Some(format!("{}", far.cpu_type)),
                _ => None,
            }
        }
        V4::PRR(prr) => {
            match field_name.to_uppercase().as_str() {
                "HEAD_NUM" => Some(format!("{}", prr.head_num)),
                "SITE_NUM" => Some(format!("{}", prr.site_num)),
                "NUM_TEST" => Some(format!("{}", prr.num_test)),
                "HARD_BIN" => Some(format!("{}", prr.hard_bin)),
                "SOFT_BIN" => Some(format!("{}", prr.soft_bin)),
                _ => None,
            }
        }
        _ => None,
    }
}

fn get_record_fields(record_type: &str) -> Result<Vec<String>, String> {
    match record_type.to_uppercase().as_str() {
        "FAR" => Ok(vec!["CPU_TYPE".to_string(), "STDF_VER".to_string()]),
        "MIR" => Ok(vec![
            "SETUP_T".to_string(), "START_T".to_string(), "STAT_NUM".to_string(), 
            "MODE_COD".to_string(), "RTST_COD".to_string(), "PROT_COD".to_string(),
            "BURN_TIM".to_string(), "CMOD_COD".to_string(), "LOT_ID".to_string(),
            "PART_TYP".to_string(), "NODE_NAM".to_string(), "TSTR_TYP".to_string(),
            "JOB_NAM".to_string(), "JOB_REV".to_string(), "SBLOT_ID".to_string(),
            "OPER_NAM".to_string(), "EXEC_TYP".to_string(), "EXEC_VER".to_string(),
            "TEST_COD".to_string(), "TST_TEMP".to_string(), "USER_TXT".to_string(),
            "AUX_FILE".to_string(), "PKG_TYP".to_string(), "FAMLY_ID".to_string(),
            "DATE_COD".to_string(), "FACIL_ID".to_string(), "FLOOR_ID".to_string(),
            "PROC_ID".to_string(), "OPER_FRQ".to_string(), "SPEC_NAM".to_string(),
            "SPEC_VER".to_string(), "FLOW_ID".to_string(), "SETUP_ID".to_string(),
            "DSGN_REV".to_string(), "ENG_ID".to_string(), "ROM_COD".to_string(),
            "SERL_NUM".to_string(), "SUPR_NAM".to_string(),
        ]),
        "PRR" => Ok(vec![
            "HEAD_NUM".to_string(), "SITE_NUM".to_string(), "PART_FLG".to_string(),
            "NUM_TEST".to_string(), "HARD_BIN".to_string(), "SOFT_BIN".to_string(),
            "X_COORD".to_string(), "Y_COORD".to_string(), "TEST_T".to_string(),
            "PART_ID".to_string(), "PART_TXT".to_string(), "PART_FIX".to_string(),
        ]),
        // Add more record types as needed
        _ => Err(format!("Fields for record type '{}' not yet implemented. Use 'show records' to see all available record types.", record_type)),
    }
}

// ============================================================================
// Compression / Decompression Functions
// ============================================================================

use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use flate2::Compression;
use flate2::read::{GzDecoder, ZlibDecoder};
use flate2::write::{GzEncoder, ZlibEncoder};
use bzip2::read::BzDecoder;
use bzip2::write::BzEncoder;
use xz2::read::XzDecoder;
use xz2::write::XzEncoder;
use sha2::{Sha256, Digest};

fn calculate_sha256<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    Ok(format!("{:x}", hasher.finalize()))
}

fn detect_compression(path: &Path) -> Result<Option<String>, String> {
    let mut file = File::open(path).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut buffer = vec![0u8; 16];
    file.read(&mut buffer).map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Use infer crate to detect file type
    if let Some(kind) = infer::get(&buffer) {
        match kind.mime_type() {
            "application/gzip" => Ok(Some("gzip".to_string())),
            "application/x-bzip2" => Ok(Some("bzip2".to_string())),
            "application/x-xz" => Ok(Some("xz".to_string())),
            "application/zstd" => Ok(Some("zstd".to_string())),
            "application/x-lz4" => Ok(Some("lz4".to_string())),
            "application/zip" => Ok(Some("zip".to_string())),
            "application/x-tar" => Ok(Some("tar".to_string())),
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}

fn process_directory_compress(
    dir_path: &Path,
    algorithm: &str,
    verify: bool,
    show_progress: bool,
    force: bool,
    recursive: bool
) -> Result<(), String> {
    use std::fs;
    
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;
    
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                errors.push(format!("Failed to read directory entry: {}", e));
                error_count += 1;
                continue;
            }
        };
        
        let path = entry.path();
        
        if path.is_dir() {
            if recursive {
                match process_directory_compress(&path, algorithm, verify, show_progress, force, recursive) {
                    Ok(_) => {},
                    Err(e) => {
                        errors.push(format!("{}: {}", path.display(), e));
                        error_count += 1;
                    }
                }
            }
        } else {
            // Check if it's an STDF file (*.std or *.stdf)
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "std" || ext_str == "stdf" {
                    match compress_file(path.to_str().unwrap(), algorithm, verify, show_progress, force) {
                        Ok(_) => success_count += 1,
                        Err(e) => {
                            errors.push(format!("{}: {}", path.display(), e));
                            error_count += 1;
                        }
                    }
                }
            }
        }
    }
    
    if error_count > 0 {
        eprintln!("\n{} file(s) compressed successfully, {} error(s):", success_count, error_count);
        for error in errors {
            eprintln!("  {}", error);
        }
        Err(format!("Failed to compress {} file(s)", error_count))
    } else if success_count == 0 {
        Err(format!("No STDF files found in {}", dir_path.display()))
    } else {
        println!("\n{} file(s) compressed successfully", success_count);
        Ok(())
    }
}

fn process_directory_decompress(
    dir_path: &Path,
    verify: bool,
    show_progress: bool,
    force: bool,
    recursive: bool
) -> Result<(), String> {
    use std::fs;
    
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;
    
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                errors.push(format!("Failed to read directory entry: {}", e));
                error_count += 1;
                continue;
            }
        };
        
        let path = entry.path();
        
        if path.is_dir() {
            if recursive {
                match process_directory_decompress(&path, verify, show_progress, force, recursive) {
                    Ok(_) => {},
                    Err(e) => {
                        errors.push(format!("{}: {}", path.display(), e));
                        error_count += 1;
                    }
                }
            }
        } else {
            // Check if file is compressed
            if let Ok(Some(_compression)) = detect_compression(&path) {
                match decompress_file(path.to_str().unwrap(), verify, show_progress, force) {
                    Ok(_) => success_count += 1,
                    Err(e) => {
                        errors.push(format!("{}: {}", path.display(), e));
                        error_count += 1;
                    }
                }
            }
        }
    }
    
    if error_count > 0 {
        eprintln!("\n{} file(s) decompressed successfully, {} error(s):", success_count, error_count);
        for error in errors {
            eprintln!("  {}", error);
        }
        Err(format!("Failed to decompress {} file(s)", error_count))
    } else if success_count == 0 {
        Err(format!("No compressed files found in {}", dir_path.display()))
    } else {
        println!("\n{} file(s) decompressed successfully", success_count);
        Ok(())
    }
}

fn compress_file(input_path: &str, algorithm: &str, verify: bool, show_progress: bool, force: bool) -> Result<String, String> {
    let path = Path::new(input_path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", input_path));
    }
    
    // Check if file is already compressed
    let compression = detect_compression(path)?;
    
    let (source_path, temp_file, should_cleanup) = if let Some(current_compression) = compression {
        // File is compressed
        if !is_supported_compression(&current_compression) {
            return Err(format!(
                "File is compressed with unsupported format '{}'. Please decompress manually.",
                current_compression
            ));
        }
        
        // Decompress first
        if !show_progress {
            println!("File is already compressed with {}. Decompressing first...", current_compression);
        }
        let decompressed = decompress_to_temp(input_path)?;
        (decompressed.clone(), Some(decompressed), true)
    } else {
        (input_path.to_string(), None, false)
    };
    
    // Calculate original hash if verify is enabled
    let original_hash = if verify {
        if !show_progress {
            println!("Calculating SHA-256 hash of source file...");
        }
        Some(calculate_sha256(&source_path).map_err(|e| format!("Failed to calculate hash: {}", e))?)
    } else {
        None
    };
    
    // Determine output extension and perform compression
    let (extension, output_path) = get_compression_extension(&source_path, algorithm)?;
    
    // Check if output file already exists
    if Path::new(&output_path).exists() && !force {
        return Err(format!("Output file already exists: {}. Use -f/--force to overwrite.", output_path));
    }
    
    if show_progress {
        compress_with_progress(&source_path, &output_path, algorithm)?;
    } else {
        use std::io::{self, Write};
        print!("Compressing with {} to {} ... ", algorithm, output_path);
        io::stdout().flush().unwrap();
        compress_with_algorithm(&source_path, &output_path, algorithm)?;
        println!("Done.");
    }
    
    // Verify if requested
    if verify {
        if !show_progress {
            println!("Verifying compression integrity...");
        }
        let decompressed_hash = verify_compressed_file(&output_path, algorithm)?;
        
        if Some(decompressed_hash.clone()) != original_hash {
            // Cleanup failed compression
            let _ = std::fs::remove_file(&output_path);
            if should_cleanup {
                let _ = std::fs::remove_file(&source_path);
            }
            
            return Err("Verification failed: Hash mismatch after compression. Do you want to retry?".to_string());
        }
        
        if !show_progress {
            println!("✓ Verification successful: Hashes match");
        }
    }
    
    // Cleanup temporary decompressed file if needed
    if should_cleanup {
        std::fs::remove_file(&source_path).map_err(|e| format!("Failed to cleanup temp file: {}", e))?;
    }
    
    Ok(output_path)
}

fn decompress_file(input_path: &str, verify: bool, show_progress: bool, force: bool) -> Result<String, String> {
    let path = Path::new(input_path);
    
    if !path.exists() {
        return Err(format!("File not found: {}", input_path));
    }
    
    // Detect compression format
    let compression = detect_compression(path)?;
    
    if compression.is_none() {
        return Err("File is not compressed".to_string());
    }
    
    let compression_format = compression.unwrap();
    
    if !is_supported_compression(&compression_format) {
        return Err(format!(
            "File is compressed with unsupported format '{}'. Supported formats: gzip, bzip2, xz, zstd, lz4",
            compression_format
        ));
    }
    
    // Calculate hash of compressed file if verify is enabled
    let compressed_hash = if verify {
        if !show_progress {
            println!("Calculating SHA-256 hash of compressed file...");
        }
        Some(calculate_sha256(input_path).map_err(|e| format!("Failed to calculate hash: {}", e))?)
    } else {
        None
    };
    
    // Determine output path (remove compression extension)
    let output_path = get_decompressed_path(input_path);
    
    // Check if output file already exists
    if Path::new(&output_path).exists() && !force {
        return Err(format!("Output file already exists: {}. Use -f/--force to overwrite.", output_path));
    }
    
    if show_progress {
        decompress_with_progress(input_path, &output_path, &compression_format)?;
    } else {
        use std::io::{self, Write};
        print!("Decompressing {} to {} ... ", compression_format, output_path);
        io::stdout().flush().unwrap();
        decompress_with_algorithm(input_path, &output_path, &compression_format)?;
        println!("Done.");
    }
    
    // Verify if requested
    if verify && compressed_hash.is_some() {
        if !show_progress {
            println!("Verifying decompression integrity...");
        }
        // Re-compress and compare hashes
        let temp_compressed = format!("{}.verify_temp", input_path);
        compress_with_algorithm(&output_path, &temp_compressed, &compression_format)?;
        
        let verify_hash = calculate_sha256(&temp_compressed).map_err(|e| format!("Failed to calculate verify hash: {}", e))?;
        std::fs::remove_file(&temp_compressed).map_err(|e| format!("Failed to cleanup temp file: {}", e))?;
        
        if Some(verify_hash) != compressed_hash {
            let _ = std::fs::remove_file(&output_path);
            return Err("Verification failed: Hash mismatch after decompression".to_string());
        }
        
        if !show_progress {
            println!("✓ Verification successful: Hashes match");
        }
    }
    
    Ok(output_path)
}

fn is_supported_compression(format: &str) -> bool {
    matches!(format, "gzip" | "bzip2" | "xz" | "zstd" | "lz4")
}

fn get_compression_extension(source: &str, algorithm: &str) -> Result<(&'static str, String), String> {
    let source_path = Path::new(source);
    let base = source_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid filename")?;
    let parent = source_path.parent().unwrap_or(Path::new(""));
    
    let (ext, output_name) = match algorithm.to_lowercase().as_str() {
        "gzip" | "gz" => ("gz", format!("{}.gz", source)),
        "zlib" | "z" => ("z", format!("{}.z", source)),
        "bzip2" | "bz2" => ("bz2", format!("{}.bz2", source)),
        "xz" | "lzma" => ("xz", format!("{}.xz", source)),
        "zstd" | "zst" => ("zst", format!("{}.zst", source)),
        "lz4" => ("lz4", format!("{}.lz4", source)),
        _ => return Err(format!("Unsupported algorithm: {}. Use: gzip, bzip2, xz, zstd, lz4", algorithm)),
    };
    
    Ok((ext, output_name))
}

fn get_decompressed_path(compressed_path: &str) -> String {
    let path = Path::new(compressed_path);
    
    // Try to remove known compression extensions
    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        if let Some(parent) = path.parent() {
            return parent.join(stem).to_string_lossy().to_string();
        }
        return stem.to_string();
    }
    
    format!("{}.decompressed", compressed_path)
}

fn compress_with_algorithm(input: &str, output: &str, algorithm: &str) -> Result<(), String> {
    let input_file = File::open(input).map_err(|e| format!("Failed to open input: {}", e))?;
    let mut reader = std::io::BufReader::new(input_file);
    
    let output_file = File::create(output).map_err(|e| format!("Failed to create output: {}", e))?;
    
    let normalized_algo = normalize_algorithm_name(algorithm);
    match normalized_algo.as_str() {
        "gzip" => {
            let mut encoder = GzEncoder::new(output_file, Compression::best());
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "zlib" => {
            let mut encoder = ZlibEncoder::new(output_file, Compression::best());
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "bzip2" => {
            let mut encoder = BzEncoder::new(output_file, bzip2::Compression::best());
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "xz" => {
            let mut encoder = XzEncoder::new(output_file, 9);
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "zstd" => {
            let mut encoder = zstd::Encoder::new(output_file, 22).map_err(|e| format!("Failed to create zstd encoder: {}", e))?;
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "lz4" => {
            let mut encoder = lz4::EncoderBuilder::new()
                .level(12)
                .build(output_file)
                .map_err(|e| format!("Failed to create lz4 encoder: {}", e))?;
            std::io::copy(&mut reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            let (_output, result) = encoder.finish();
            result.map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    }
    
    Ok(())
}

fn decompress_with_algorithm(input: &str, output: &str, algorithm: &str) -> Result<(), String> {
    let input_file = File::open(input).map_err(|e| format!("Failed to open input: {}", e))?;
    let output_file = File::create(output).map_err(|e| format!("Failed to create output: {}", e))?;
    let mut writer = std::io::BufWriter::new(output_file);
    
    match algorithm.to_lowercase().as_str() {
        "gzip" => {
            let mut decoder = GzDecoder::new(input_file);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "zlib" => {
            let mut decoder = ZlibDecoder::new(input_file);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "bzip2" => {
            let mut decoder = BzDecoder::new(input_file);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "xz" => {
            let mut decoder = XzDecoder::new(input_file);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "zstd" => {
            let mut decoder = zstd::Decoder::new(input_file).map_err(|e| format!("Failed to create zstd decoder: {}", e))?;
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "lz4" => {
            let mut decoder = lz4::Decoder::new(input_file).map_err(|e| format!("Failed to create lz4 decoder: {}", e))?;
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    }
    
    writer.flush().map_err(|e| format!("Failed to flush output: {}", e))?;
    Ok(())
}

fn decompress_to_temp(input: &str) -> Result<String, String> {
    let path = Path::new(input);
    let compression = detect_compression(path)?.ok_or("Not a compressed file")?;
    
    let temp_path = format!("{}.temp_decompressed", input);
    decompress_with_algorithm(input, &temp_path, &compression)?;
    
    Ok(temp_path)
}

fn verify_compressed_file(compressed_path: &str, algorithm: &str) -> Result<String, String> {
    // Decompress to temp and calculate hash
    let temp_decompressed = format!("{}.verify_temp", compressed_path);
    
    // Normalize algorithm name for decompression
    let normalized_algo = normalize_algorithm_name(algorithm);
    decompress_with_algorithm(compressed_path, &temp_decompressed, &normalized_algo)?;
    
    let hash = calculate_sha256(&temp_decompressed).map_err(|e| format!("Failed to calculate hash: {}", e))?;
    std::fs::remove_file(&temp_decompressed).map_err(|e| format!("Failed to cleanup temp file: {}", e))?;
    
    Ok(hash)
}

fn normalize_algorithm_name(algorithm: &str) -> String {
    match algorithm.to_lowercase().as_str() {
        "gz" => "gzip".to_string(),
        "z" => "zlib".to_string(),
        "bz2" => "bzip2".to_string(),
        "lzma" => "xz".to_string(),
        "zst" => "zstd".to_string(),
        _ => algorithm.to_string(),
    }
}

fn compress_with_progress(input: &str, output: &str, algorithm: &str) -> Result<(), String> {
    use indicatif::{ProgressBar, ProgressStyle};
    
    let input_file = File::open(input).map_err(|e| format!("Failed to open input: {}", e))?;
    let file_size = input_file.metadata().map_err(|e| format!("Failed to get file size: {}", e))?.len();
    
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(format!("Compressing with {} to {}", algorithm, output));
    
    let reader = pb.wrap_read(input_file);
    let mut buffered_reader = std::io::BufReader::new(reader);
    
    let output_file = File::create(output).map_err(|e| format!("Failed to create output: {}", e))?;
    let mut writer = std::io::BufWriter::new(output_file);
    
    let normalized_algo = normalize_algorithm_name(algorithm);
    
    match normalized_algo.as_str() {
        "gzip" => {
            let mut encoder = GzEncoder::new(writer, Compression::default());
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "zlib" => {
            let mut encoder = ZlibEncoder::new(writer, Compression::default());
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "bzip2" => {
            let mut encoder = BzEncoder::new(writer, bzip2::Compression::default());
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "xz" => {
            let mut encoder = XzEncoder::new(writer, 6);
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "zstd" => {
            let mut encoder = zstd::Encoder::new(writer, 3).map_err(|e| format!("Failed to create zstd encoder: {}", e))?;
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        "lz4" => {
            let mut encoder = lz4::EncoderBuilder::new().build(writer).map_err(|e| format!("Failed to create lz4 encoder: {}", e))?;
            std::io::copy(&mut buffered_reader, &mut encoder).map_err(|e| format!("Compression failed: {}", e))?;
            encoder.finish().0.flush().map_err(|e| format!("Failed to finalize compression: {}", e))?;
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    }
    
    pb.finish_and_clear();
    println!("✓ Compressed to {}", output);
    Ok(())
}

fn decompress_with_progress(input: &str, output: &str, algorithm: &str) -> Result<(), String> {
    use indicatif::{ProgressBar, ProgressStyle};
    
    let input_file = File::open(input).map_err(|e| format!("Failed to open input: {}", e))?;
    let file_size = input_file.metadata().map_err(|e| format!("Failed to get file size: {}", e))?.len();
    
    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(format!("Decompressing {} to {}", algorithm, output));
    
    let reader = pb.wrap_read(input_file);
    let mut buffered_reader = std::io::BufReader::new(reader);
    
    let output_file = File::create(output).map_err(|e| format!("Failed to create output: {}", e))?;
    let mut writer = std::io::BufWriter::new(output_file);
    
    match algorithm.to_lowercase().as_str() {
        "gzip" => {
            let mut decoder = GzDecoder::new(buffered_reader);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "zlib" => {
            let mut decoder = ZlibDecoder::new(buffered_reader);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "bzip2" => {
            let mut decoder = BzDecoder::new(buffered_reader);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "xz" => {
            let mut decoder = XzDecoder::new(buffered_reader);
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "zstd" => {
            let mut decoder = zstd::Decoder::new(buffered_reader).map_err(|e| format!("Failed to create zstd decoder: {}", e))?;
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        "lz4" => {
            let mut decoder = lz4::Decoder::new(buffered_reader).map_err(|e| format!("Failed to create lz4 decoder: {}", e))?;
            std::io::copy(&mut decoder, &mut writer).map_err(|e| format!("Decompression failed: {}", e))?;
        }
        _ => return Err(format!("Unsupported algorithm: {}", algorithm)),
    }
    
    writer.flush().map_err(|e| format!("Failed to flush output: {}", e))?;
    pb.finish_and_clear();
    println!("✓ Decompressed to {}", output);
    Ok(())
}

