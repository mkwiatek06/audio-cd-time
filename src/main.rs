use colored::Colorize;
use lofty::{file::AudioFile, probe::Probe};
use std::env;
use std::path::Path;
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

fn display(pre: &str, time_ns: &u128) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut multi_folder = false;
    if args.len() > 2 {
        multi_folder = true;
    }

    let mut duration_comb_i: u128 = 0;

    for folder in args.iter().skip(1) {
        let mut duration_comb_f: u128 = 0;

        for entry in WalkDir::new(folder) {
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

            let duration = file.properties().duration().as_nanos();
            duration_comb_f += duration;
        }

        let folder_name = Path::new(folder)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap();

        display(&format!("::: {}:", folder_name), &duration_comb_f);

        if multi_folder {
            duration_comb_i += duration_comb_f;
        }
    }

    if multi_folder {
        println!("");
        display(":+: COMBINED:", &duration_comb_i)
    }

    Ok(())
}
