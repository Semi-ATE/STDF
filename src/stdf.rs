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
        "show" => {
            if args.len() < 3 {
                eprintln!("Error: Missing arguments");
                eprintln!("Usage: {} show <subcommand> [args...]", args[0]);
                process::exit(1);
            }
            
            handle_show_command(&args);
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
    use stdf::{StdfStreamingIterator, stdf_parse_record, V4};
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
    use stdf::{StdfStreamingIterator, stdf_parse_record, V4};
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
    use stdf::{StdfStreamingIterator, stdf_parse_record, V4};
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
    use stdf::{StdfStreamingIterator, stdf_parse_record, V4};
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
                println!("zip : .zip (Archive format)");
                println!("tar : .tar (Archive format)");
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
    use stdf::{StdfRecordIterator, StdfStreamingIterator, stdf_parse_record, V4};
    
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

fn extract_field_value(record: &stdf::V4, field_name: &str) -> Option<String> {
    use stdf::V4;
    
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
