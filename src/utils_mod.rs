// utils_mod.rs
//! A module with often used functions.

use chrono::prelude::*;
use unwrap::unwrap;

// ANSI colors for Linux terminal
// https://github.com/shiena/ansicolor/blob/master/README.md
#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";

/// returns the now in nanoseconds
pub fn ns_start(text: &str) -> i64 {
    let now = Utc::now();
    if !text.is_empty() {
        let time_now = &Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{GREEN}{time_now}: {text}{RESET}");
    }
    now.timestamp_nanos()
}

/// returns the elapsed nanoseconds
pub fn ns_elapsed(ns_start: i64) -> i64 {
    let now_ns = Utc::now().timestamp_nanos();
    let duration_ns = now_ns - ns_start;
    // return
    duration_ns
}

/// print elapsed time in milliseconds and returns the new now in nanoseconds
pub fn ns_print_ms(name: &str, ns_start: i64) -> i64 {
    // milliseconds
    let duration_ns = ns_elapsed(ns_start) / 1_000_000;
    if !name.is_empty() {
        use num_format::{Locale, WriteFormatted};
        let mut string_duration_ns = String::new();
        unwrap!(string_duration_ns.write_formatted(&duration_ns, &Locale::en));

        println!("{GREEN}{string_duration_ns:>15} ms: {name}{RESET}");
    }
    // return new now_ns
    Utc::now().timestamp_nanos()
}

/// print elapsed time in nanoseconds and returns the new now in nanoseconds
pub fn ns_print_ns(name: &str, ns_start: i64) -> i64 {
    // milliseconds
    let duration_ns = ns_elapsed(ns_start);
    if !name.is_empty() {
        use num_format::{Locale, WriteFormatted};
        let mut string_duration_ns = String::new();
        unwrap!(string_duration_ns.write_formatted(&duration_ns, &Locale::en));

        println!("{GREEN}{string_duration_ns:>15} ns: {name}{RESET}");
    }
    // return new now_ns
    Utc::now().timestamp_nanos()
}

// datetime string to append to a file_name
pub fn datetime_now_for_file_names() -> String {
    let now = chrono::offset::Utc::now();
    let datetime_string = now.format("%Y-%m-%dT%TZ").to_string();
    // return
    datetime_string
}

// return is like: ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU
pub fn get_data_dir() -> std::path::PathBuf {
    // call "cargo-crev crev config data-dir" to get "~/.local/share/crev" in both development and production
    let output = std::process::Command::new("cargo-crev")
        .args(["crev", "config", "data-dir"])
        .output()
        .expect("failed to execute 'cargo-crev crev config data-dir'");

    let data_dir = String::from_utf8(output.stdout).unwrap();
    // "~/.local/share/crev\n"
    let mut data_dir = data_dir.trim_end_matches("\n").to_string();
    data_dir.push_str("/proofs");
    // "~/.local/share/crev/proofs"
    //dbg!(&data_dir);

    // get the current id
    let output = std::process::Command::new("cargo-crev")
        .args(["crev", "id", "current"])
        .output()
        .expect("failed to execute 'cargo-crev crev id current'");

    let id_current = String::from_utf8(output.stdout).unwrap();
    // "UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU https://github.com/web-crev-dev/crev-proofs (current)\n"
    let mut iter = id_current.split_ascii_whitespace();
    let id_current = iter.next().unwrap();
    // "UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU"
    // dbg!(id_current);

    // inside of that is a "crazy named" folder that contains the id folder. There can be more folders.
    // search all folders for a subfolder called id like "UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU"
    let dir_1 = std::fs::read_dir(&data_dir).unwrap();
    'outer: for path in dir_1 {
        let entry_1 = path.unwrap();
        if entry_1.path().is_dir() {
            let dir_1_name = entry_1.file_name();
            // dbg!(&dir_1_name);
            // if it contains a folder named as id
            let dir_2 = std::fs::read_dir(&entry_1.path()).unwrap();
            for path_2 in dir_2 {
                let entry_2 = path_2.unwrap();
                if entry_2.path().is_dir() {
                    let dir_2_name = entry_2.file_name();
                    // dbg!(&dir_2_name);
                    if dir_2_name.to_string_lossy() == id_current {
                        data_dir.push_str(&format!("/{}", dir_1_name.to_string_lossy()));
                        break 'outer;
                    }
                }
            }
        }
    }
    data_dir.push_str(&format!("/{id_current}"));

    // return
    // dbg!(&data_dir);
    std::path::PathBuf::from(&data_dir)
}
