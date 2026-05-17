use lofty::{file::AudioFile, probe::Probe};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::DETAILED;
use crate::visual::Colors;
use crate::visual::display;

pub trait Compute {
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

            let duration = file.properties().duration().as_nanos();
            *combined_duration += duration;

            if *DETAILED.get().unwrap() {
                let file_name = entry.path().file_name().unwrap().to_string_lossy();
                display(format!("▐█▌ {}:", file_name), &duration, Colors::Static);
            }
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

            let duration = file.properties().duration().as_nanos();
            *combined_duration += duration;

            if *DETAILED.get().unwrap() {
                let file_name = entry.path().file_name().unwrap().to_string_lossy();
                display(format!("▐█▌ {}:", file_name), &duration, Colors::Static);
            }
        }
    }
}

pub fn handle_dir(dir: PathBuf, acc: Option<&mut u128>) {
    let mut duration_comb_f: u128 = 0;

    dir.compute(&mut duration_comb_f);

    let folder_name = dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    display(
        format!("::: {}:", folder_name),
        &duration_comb_f,
        Colors::Yes,
    );

    match acc {
        Some(v) => *v += duration_comb_f,
        None => (),
    }
}
