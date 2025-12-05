use std::env;
use std::process;
use std::collections::HashMap;

use stdf::parsers::StdfParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "count" => {
            if args.len() < 3 {
                eprintln!("Error: Missing subcommand");
                eprintln!("Usage: {} count <records|parts|sbins|hbins> <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let subcommand = &args[2];
            match subcommand.as_str() {
                "records" => {
                    if args.len() < 4 {
                        eprintln!("Error: Missing file argument");
                        eprintln!("Usage: {} count records <stdf_file>", args[0]);
                        process::exit(1);
                    }
                    let filename = &args[3];
                    match count_records(filename) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                "parts" => {
                    if args.len() >= 4 && args[3] == "unique" {
                        eprintln!("'count parts unique' not yet implemented");
                    } else {
                        eprintln!("'count parts' not yet implemented");
                    }
                    process::exit(1);
                }
                "sbins" => {
                    eprintln!("'count sbins' not yet implemented");
                    process::exit(1);
                }
                "hbins" => {
                    eprintln!("'count hbins' not yet implemented");
                    process::exit(1);
                }
                _ => {
                    eprintln!("Error: Unknown subcommand '{}'", subcommand);
                    eprintln!("Usage: {} count <records|parts|sbins|hbins> <stdf_file>", args[0]);
                    process::exit(1);
                }
            }
        }
        "tally" => {
            if args.len() < 3 {
                eprintln!("Error: Missing subcommand");
                eprintln!("Usage: {} tally <records|heads|sites|hbins|sbins> <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let subcommand = &args[2];
            match subcommand.as_str() {
                "records" => {
                    if args.len() < 4 {
                        eprintln!("Error: Missing file argument");
                        eprintln!("Usage: {} tally records <stdf_file>", args[0]);
                        process::exit(1);
                    }
                    let filename = &args[3];
                    match tally_records(filename) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(1);
                        }
                    }
                }
                "heads" => {
                    eprintln!("'tally heads' not yet implemented");
                    process::exit(1);
                }
                "sites" => {
                    eprintln!("'tally sites' not yet implemented");
                    process::exit(1);
                }
                "hbins" => {
                    eprintln!("'tally hbins' not yet implemented");
                    process::exit(1);
                }
                "sbins" => {
                    eprintln!("'tally sbins' not yet implemented");
                    process::exit(1);
                }
                _ => {
                    eprintln!("Error: Unknown subcommand '{}'", subcommand);
                    eprintln!("Usage: {} tally <records|heads|sites|hbins|sbins> <stdf_file>", args[0]);
                    process::exit(1);
                }
            }
        }
        "dump" => {
            // Parse arguments: stdf dump [record_types...] <file>
            // The last argument is always the file
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} dump [record_types...] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let filename = &args[args.len() - 1];
            let record_types: Vec<String> = args[2..args.len() - 1]
                .iter()
                .map(|s| s.to_uppercase())
                .collect();
            
            // Remove duplicates
            let mut unique_types: Vec<String> = Vec::new();
            for rt in record_types {
                if !unique_types.contains(&rt) {
                    unique_types.push(rt);
                }
            }
            
            match dump_stdf(filename, &unique_types) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    process::exit(1);
                }
            }
        }
        "to" => {
            if args.len() < 4 {
                eprintln!("Error: Missing arguments");
                eprintln!("Usage: {} to <xlsx|atdf|hdf5> [-f] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            // Parse arguments: format and optional -f flag
            let format = args[2].to_lowercase();
            let mut force = false;
            let filename;
            
            // Check if -f flag is present
            if args.len() == 5 && args[3] == "-f" {
                force = true;
                filename = &args[4];
            } else if args.len() == 5 && args[4] == "-f" {
                force = true;
                filename = &args[3];
            } else if args.len() == 4 {
                filename = &args[3];
            } else {
                eprintln!("Error: Invalid arguments");
                eprintln!("Usage: {} to <xlsx|atdf|hdf5> [-f] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            match format.as_str() {
                "xlsx" => {
                    eprintln!("XLSX conversion not yet implemented");
                    process::exit(1);
                }
                "atdf" => {
                    match convert_to_atdf(filename, force) {
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
                    eprintln!("Usage: {} to <xlsx|atdf|hdf5> [-f] <stdf_file>", args[0]);
                    process::exit(1);
                }
            }
        }
        "is" => {
            if args.len() < 4 {
                eprintln!("Error: Missing arguments");
                eprintln!("Usage: {} is <ft|ws|hot|cold|room|truncated|complete> <stdf_file>", args[0]);
                process::exit(1);
            }
            let test_type = args[2].to_lowercase();
            let filename = &args[3];
            
            match test_type.as_str() {
                "ft" => {
                    match check_is_ft(filename) {
                        Ok(true) => process::exit(0),  // Is FT
                        Ok(false) => process::exit(1), // Not FT (is WS)
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "ws" => {
                    match check_is_ws(filename) {
                        Ok(true) => process::exit(0),  // Is WS
                        Ok(false) => process::exit(1), // Not WS (is FT)
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "hot" => {
                    match check_is_hot(filename) {
                        Ok(true) => process::exit(0),  // Is hot (>50°C)
                        Ok(false) => process::exit(1), // Not hot
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "cold" => {
                    match check_is_cold(filename) {
                        Ok(true) => process::exit(0),  // Is cold (<10°C)
                        Ok(false) => process::exit(1), // Not cold
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "room" => {
                    match check_is_room(filename) {
                        Ok(true) => process::exit(0),  // Is room (10-50°C)
                        Ok(false) => process::exit(1), // Not room
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "truncated" => {
                    match check_is_truncated(filename) {
                        Ok(true) => process::exit(0),  // Is truncated
                        Ok(false) => process::exit(1), // Not truncated
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            process::exit(2);
                        }
                    }
                }
                "complete" => {
                    match check_is_complete(filename) {
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
                    eprintln!("Usage: {} is <ft|ws|hot|cold|room|truncated|complete> <stdf_file>", args[0]);
                    process::exit(1);
                }
            }
        }
        "endian" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} endian <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match get_endian(filename) {
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
        "lot" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} lot <stdf_file>", args[0]);
                process::exit(1);
            }
            let filename = &args[2];
            match get_lot_id(filename) {
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
        "tester" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} tester [type] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let (get_type, filename) = if args.len() >= 4 && args[2] == "type" {
                (true, &args[3])
            } else {
                (false, &args[2])
            };
            
            if get_type {
                match get_tester_type(filename) {
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
                match get_tester(filename) {
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
        "temperature" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file argument");
                eprintln!("Usage: {} temperature [int] <stdf_file>", args[0]);
                process::exit(1);
            }
            
            let (as_int, filename) = if args.len() >= 4 && args[2] == "int" {
                (true, &args[3])
            } else {
                (false, &args[2])
            };
            
            match get_temperature(filename) {
                Ok(temp) => {
                    if as_int {
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
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("Semi-ATE STDF Tool");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("\nUsage: {} <command> [args...]", program);
    println!("\nCommands:");
    println!("  count records <file>        - Count total number of records");
    println!("  count parts <file>          - Count number of parts tested (not yet implemented)");
    println!("  count parts unique <file>   - Count unique parts excluding retests (not yet implemented)");
    println!("  count sbins <file>          - Count number of soft bins (not yet implemented)");
    println!("  count hbins <file>          - Count number of hard bins (not yet implemented)");
    println!("  tally records <file>        - Show tally of each record type (not yet implemented)");
    println!("  tally heads <file>          - Show tally of test heads (not yet implemented)");
    println!("  tally sites <file>          - Show tally of sites (not yet implemented)");
    println!("  tally hbins <file>          - Show tally of hard bins (not yet implemented)");
    println!("  tally sbins <file>          - Show tally of soft bins (not yet implemented)");
    println!("  dump <file>                 - Dump all records");
    println!("  dump <record_types...> <file> - Dump specific record types");
    println!("                                  Example: dump MIR PRR file.std");
    println!("  is <ft|ws|hot|cold|room|truncated|complete> <file>");
    println!("                                               - Check file properties");
    println!("                                               ft: Final Test, ws: Wafer Sort");
    println!("                                               hot: >50°C, cold: <10°C, room: 10-50°C");
    println!("                                               truncated: last record incomplete");
    println!("                                               complete: ends with MRR record");
    println!("                                               (exit 0=match, 1=no match, 2=error)");
    println!("  endian <file>               - Get file endianness (outputs: LE or BE)");
    println!("  lot <file>                  - Get lot ID from MIR record");
    println!("  tester [type] <file>        - Get tester name (NODE_NAM) from MIR record");
    println!("                                  type: get tester type (TSTR_TYP) instead");
    println!("  temperature [int] <file>    - Get test temperature from MIR record");
    println!("                                  int: parse as integer value");
    println!("  to <format> [-f] <file>     - Convert STDF file to another format");
    println!("                                  Formats: xlsx, atdf, hdf5");
    println!("                                  -f: force overwrite if output exists");
    println!("                                  Example: to atdf file.std");
    println!("                                  Example: to atdf -f file.std");
    println!("  help                        - Show this help message");
}

fn count_records(filename: &str) -> Result<(), std::io::Error> {
    let parser = StdfParser::new();
    let count = parser.count_records(filename)?;
    println!("{}", count);
    Ok(())
}

fn tally_records(filename: &str) -> Result<(), std::io::Error> {
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
    
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
    use stdf::StdfRecordIterator;
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
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
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
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
    let iter = StdfRecordIterator::new(filename)?;
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
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
    let iter = StdfRecordIterator::new(filename)?;
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
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
    let iter = StdfRecordIterator::new(filename)?;
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
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
    let iter = StdfRecordIterator::new(filename)?;
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
    use stdf::StdfRecordIterator;
    let iter = StdfRecordIterator::new(filename)?;
    let endian = iter.endian();
    Ok(match endian {
        byte::ctx::Endian::Little => "LE".to_string(),
        byte::ctx::Endian::Big => "BE".to_string(),
    })
}

fn convert_to_atdf(filename: &str, force: bool) -> Result<String, std::io::Error> {
    use stdf::{StdfRecordIterator, stdf_parse_record, V4};
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
