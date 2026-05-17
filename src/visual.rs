use colored::Colorize;

pub enum Colors {
    Yes,
    No,
    Static,
}

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

pub fn display(pre: String, time_ns: &u128, highlight: Colors) {
    match highlight {
        Colors::Yes => {
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
        Colors::No => {
            println!(
                "{} {}",
                pre,
                format!("{:02}:{:02}", minutes!(*time_ns), seconds!(*time_ns))
            );
        }
        Colors::Static => {
            println!(
                "{} {}",
                pre,
                format!("{:02}:{:02}", minutes!(*time_ns), seconds!(*time_ns)).cyan()
            );
        }
    }
}
