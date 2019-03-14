extern crate page_size;
extern crate procinfo;

use std::cmp::max;
use std::env;
use std::{thread, time};

fn print_help() {
    println!("usage: memwatcher pid");
}

fn convert_to_mb(mem_size: usize) -> f64 {
    return mem_size as f64 / (1024.0_f64 * 1024.0_f64);
}

fn watch_pid(pid: i32) {
    println!("Watching pid: {}", pid);
    let page_size_kb = page_size::get();
    println!("Page size: {} B", page_size_kb);

    // Save previous value and peak memory usage
    let mut previous_value: usize = 0;
    let mut peak_mem: usize = 0;
    loop {
        let statm = procinfo::pid::statm(pid);
        match statm {
            Ok(s) => {
                // Only print if the value has changed.
                if s.resident != previous_value {
                    println!(
                        "Memory usage: {} MB",
                        convert_to_mb(s.resident * page_size_kb)
                    );
                }
                previous_value = s.resident;

                // Save highest value
                peak_mem = max(peak_mem, s.resident);
            }
            Err(_) => {
                println!("Process no longer running.");
                if peak_mem > 0 {
                    println!(
                        "Peak memory usage: {} MB",
                        convert_to_mb(peak_mem * page_size_kb)
                    );
                }
                return;
            }
        }

        let sleep_time = time::Duration::from_secs(5);
        thread::sleep(sleep_time);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // Expects pid argument
        2 => {
            let pid: i32 = match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid pid: must be an integer.");
                    return;
                }
            };

            watch_pid(pid);
        }
        _ => {
            print_help();
        }
    }
}
