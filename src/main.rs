use lofty::{file::AudioFile, probe::Probe};
use std::env;
use walkdir::WalkDir;

fn convert(time_in_nanos: u128) -> (i32, i32) {
    let time_in_seconds = (time_in_nanos as f64 / 1_000_000_000.0).round() as i32;
    let display: (i32, i32) = (time_in_seconds / 60, time_in_seconds % 60);
    return display;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(" === Target for Standard CDs: 74:00 ===");
    println!(" === Target for Extended CDs: 80:00 ===");
    println!("");
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

            // println!("{}", entry.path().display());

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

        let folder_name;
        if let Some(pos) = folder.rfind('/') {
            folder_name = &folder[pos + 1..];
        } else {
            folder_name = folder;
        }

        let time_for_f = convert(duration_comb_f);
        println!(
            "::: {}: {:02}:{:02}",
            folder_name, time_for_f.0, time_for_f.1
        );

        if multi_folder {
            duration_comb_i += duration_comb_f;
        }
    }

    if multi_folder {
        let time_for_i = convert(duration_comb_i);
        println!("");
        println!(":+: COMBINED: {:02}:{:02}", time_for_i.0, time_for_i.1);
    }

    Ok(())
}
