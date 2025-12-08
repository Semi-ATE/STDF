use stdf::{StdfRecordIterator, StdfRecordFPIterator};
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <stdf-file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("Benchmarking iterators on: {}", file_path);
    println!("{}", "=".repeat(80));

    // Benchmark StdfRecordIterator
    println!("\nTesting StdfRecordIterator...");
    let start = Instant::now();
    let mut count1 = 0;
    let mut total_bytes1 = 0;

    match StdfRecordIterator::new(file_path) {
        Ok(iter) => {
            for result in iter {
                match result {
                    Ok(record) => {
                        count1 += 1;
                        total_bytes1 += record.len();
                    }
                    Err(e) => {
                        eprintln!("Error reading record {}: {}", count1, e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create StdfRecordIterator: {}", e);
            std::process::exit(1);
        }
    }

    let duration1 = start.elapsed();
    println!("  Records: {}", count1);
    println!("  Total bytes processed: {}", total_bytes1);
    println!("  Time: {:.3}s", duration1.as_secs_f64());
    println!("  Throughput: {:.2} MB/s", total_bytes1 as f64 / duration1.as_secs_f64() / 1_000_000.0);

    // Benchmark StdfRecordFPIterator
    println!("\nTesting StdfRecordFPIterator...");
    let start = Instant::now();
    let mut count2 = 0;
    let mut total_bytes2 = 0;
    let mut last_fp = 0;

    match StdfRecordFPIterator::new(file_path) {
        Ok(iter) => {
            for result in iter {
                match result {
                    Ok((record, fp)) => {
                        count2 += 1;
                        total_bytes2 += record.len();
                        last_fp = fp;
                    }
                    Err(e) => {
                        eprintln!("Error reading record {}: {}", count2, e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to create StdfRecordFPIterator: {}", e);
            std::process::exit(1);
        }
    }

    let duration2 = start.elapsed();
    println!("  Records: {}", count2);
    println!("  Total bytes processed: {}", total_bytes2);
    println!("  Last file pointer: {}", last_fp);
    println!("  Time: {:.3}s", duration2.as_secs_f64());
    println!("  Throughput: {:.2} MB/s", total_bytes2 as f64 / duration2.as_secs_f64() / 1_000_000.0);

    // Comparison
    println!("\n{}", "=".repeat(80));
    println!("COMPARISON:");
    println!("  StdfRecordIterator:   {:.3}s", duration1.as_secs_f64());
    println!("  StdfRecordFPIterator: {:.3}s", duration2.as_secs_f64());
    
    let diff_ms = (duration2.as_millis() as i128 - duration1.as_millis() as i128) as f64;
    let diff_pct = (duration2.as_secs_f64() / duration1.as_secs_f64() - 1.0) * 100.0;
    
    if diff_ms.abs() < 10.0 {
        println!("  Difference: ~{:.1}ms (essentially identical)", diff_ms);
    } else if diff_pct > 0.0 {
        println!("  Difference: StdfRecordFPIterator is {:.1}ms slower ({:.1}% overhead)", diff_ms, diff_pct);
    } else {
        println!("  Difference: StdfRecordFPIterator is {:.1}ms faster ({:.1}% faster)", -diff_ms, -diff_pct);
    }
    
    println!("\nConclusion:");
    if diff_pct.abs() < 1.0 {
        println!("  The performance difference is negligible (<1%).");
        println!("  The file pointer tracking adds virtually no overhead.");
    } else if diff_pct > 5.0 {
        println!("  The file pointer tracking adds {:.1}% overhead.", diff_pct);
    } else {
        println!("  The file pointer tracking adds minimal overhead ({:.1}%).", diff_pct);
    }
}
