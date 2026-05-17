mod logic;
mod visual;

use crate::logic::Compute;
use crate::visual::Colors;
use crate::visual::display;
use std::collections::VecDeque;
use std::env;
use std::sync::OnceLock;

static DETAILED: OnceLock<bool> = OnceLock::new();
static MULTI_ARG: OnceLock<bool> = OnceLock::new();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: VecDeque<String> = env::args().collect();
    args.pop_front();

    match args.front() {
        Some(v) => {
            if v == "-d" {
                DETAILED.set(true).unwrap();
                args.pop_front();
                if args.is_empty() {
                    logic::handle_dir(
                        match env::current_dir() {
                            Ok(v) => v,
                            Err(e) => panic!("Failed to get current directory: {e}"),
                        },
                        None,
                    );
                }
            } else {
                DETAILED.set(false).unwrap();
            }

            if args.len() > 1 {
                MULTI_ARG.set(true).unwrap();
            } else {
                MULTI_ARG.set(false).unwrap();
            }
        }
        None => {
            DETAILED.set(false).unwrap();
            logic::handle_dir(
                match env::current_dir() {
                    Ok(v) => v,
                    Err(e) => panic!("Failed to get current directory: {e}"),
                },
                None,
            );
            return Ok(());
        }
    }

    let mut duration_comb_i: u128 = 0;
    for arg in args.iter() {
        let mut duration_comb_f: u128 = 0;

        arg.compute(&mut duration_comb_f);

        let canonical = std::fs::canonicalize(arg)?;
        let folder_name = canonical
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown");

        display(
            format!("::: {}:", folder_name),
            &duration_comb_f,
            Colors::Yes,
        );

        if *MULTI_ARG.get().unwrap() {
            duration_comb_i += duration_comb_f;
        }
    }

    if *MULTI_ARG.get().unwrap() {
        println!("");
        display(":+: COMBINED:".to_string(), &duration_comb_i, Colors::Yes)
    }

    Ok(())
}
