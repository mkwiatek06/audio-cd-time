use colored::Colorize;
use lofty::{file::AudioFile, probe::Probe};
use std::{env, path::PathBuf};
use walkdir::WalkDir;

macro_rules! minutes {
    ($time_nanos:expr) => {
        (($time_nanos as f64 / 1_000_000_000.0).ceil() as i32 / 60)
    };
}

macro_rules! seconds {
    ($time_nanos:expr) => {
        (($time_nanos as f64 / 1_000_000_000.0).ceil() as i32 % 60)
    };
}

fn display(pre: String, time_ns: &u128) {
    if *time_ns > 4_800_000_000_000 {
        // > 80 minutes
        println!(
            "{} {}",
            pre,
            format!("{:02}:{:02}", minutes!(*time_ns), seconds!(*time_ns)).red()
        );
    } else if *time_ns > 4_440_000_000_000 {
        // > 74 minutes
        println!(
            "{} {}",
            pre,
            format!("{:02}:{:02}", minutes!(*time_ns), seconds!(*time_ns)).blue()
        );
    } else {
        // < 74 minutes
        println!(
            "{} {}",
            pre,
            format!("{:02}:{:02}", minutes!(*time_ns), seconds!(*time_ns)).green()
        );
    }
}

trait Compute {
    fn compute(self, _: &mut u128);
}

impl Compute for &String {
    fn compute(self, combined_duration: &mut u128) {
        for entry in WalkDir::new(self) {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => continue,
            };

            let file = match Probe::open(entry.path()) {
                Ok(probe) => match probe.read() {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            *combined_duration += file.properties().duration().as_nanos();
        }
    }
}

impl Compute for &PathBuf {
    fn compute(self, combined_duration: &mut u128) {
        for entry in WalkDir::new(self) {
            let entry = match entry {
                Ok(v) => v,
                Err(_) => continue,
            };

            let file = match Probe::open(entry.path()) {
                Ok(probe) => match probe.read() {
                    Ok(v) => v,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };

            *combined_duration += file.properties().duration().as_nanos();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let current_dir = match env::current_dir() {
            Ok(v) => v,
            Err(e) => panic!("Failed to get current directory: {e}"),
        };
        let mut duration_comb_f: u128 = 0;

        current_dir.compute(&mut duration_comb_f);

        let folder_name = current_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        display(format!("::: {}:", folder_name), &duration_comb_f);

        return Ok(());
    }

    let mut duration_comb_i: u128 = 0;
    let mut multi_arg = false;
    if args.len() > 2 {
        multi_arg = true;
    }

    for arg in args.iter().skip(1) {
        let mut duration_comb_f: u128 = 0;

        arg.compute(&mut duration_comb_f);

        let canonical = std::fs::canonicalize(arg)?;
        let folder_name = canonical
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown");

        display(format!("::: {}:", folder_name), &duration_comb_f);

        if multi_arg {
            duration_comb_i += duration_comb_f;
        }
    }

    if multi_arg {
        println!("");
        display(":+: COMBINED:".to_string(), &duration_comb_i)
    }

    Ok(())
}
