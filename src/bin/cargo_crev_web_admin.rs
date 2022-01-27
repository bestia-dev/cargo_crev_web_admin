//! cargo_crev_web_admin.rs

use cargo_crev_web_admin::*;
use std::env;

fn main() {
    pretty_env_logger::init();

    match env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("completion") => completion(),
        Some("trusted_from_crev_command") => {
            let ns_started = ns_start("trusted_from_crev_command");
            trusted_from_crev_command();
            ns_print_ms("trusted_from_crev_command", ns_started);
        }
        Some("trusted_from_files") => {
            let ns_started = ns_start("trusted_from_files");
            trusted_from_files();
            ns_print_ms("trusted_from_files", ns_started);
        }
        Some("fetch") => {
            let ns_started = ns_start("fetch");
            fetch();
            ns_print_ms("fetch", ns_started);
        }
        Some("add_trust") => match env::args().nth(2).as_deref() {
            Some(url) => {
                let ns_started = ns_start("add_trust");
                add_trust(url);
                ns_print_ms("add_trust", ns_started);
            }
            _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
        },
        Some("delete_trust") => match env::args().nth(2).as_deref() {
            Some(url) => {
                let ns_started = ns_start("delete_trust");
                delete_trust(url);
                ns_print_ms("delete_trust", ns_started);
            }
            _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
        },
        Some("reindex") => {
            let ns_started = ns_start("reindex");
            reindex();
            ns_print_ms("reindex", ns_started);
        }
        Some("add_blocklisted") => {
            let ns_started = ns_start("add_blocklisted");
            match env::args().nth(2).as_deref() {
                Some(repo_url) => match env::args().nth(3).as_deref() {
                    Some(note) => add_blocklisted(repo_url, note),
                    _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
                },
                _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
            }
            ns_print_ms("add_blocklisted", ns_started);
        }
        Some("delete_blocklisted") => {
            let ns_started = ns_start("delete_blocklisted");
            match env::args().nth(2).as_deref() {
                Some(repo_url) => delete_blocklisted(repo_url),
                _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
            }
            ns_print_ms("delete_blocklisted", ns_started);
        }
        /*
        Some("list_and_sync") => match env::args().nth(2).as_deref() {
            Some(path) => {
                let ns_started = ns_start("list_and_sync");
                print!("{}", *CLEAR_ALL);
                list_and_sync(path);
                ns_print_ms("list_and_sync", ns_started);
            }
            _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
        },
         */
        _ => println!("Unrecognized arguments. Try `cargo_crev_web_admin --help`"),
    }
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
/// `complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin`
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(
        sub_commands: Vec<&str>,
        word_being_completed: &str,
    ) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    // `complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin`
    // this completion always sends this arguments:
    // 0. executable path
    // 1. word completion
    // 2. executable file name
    // 3. word_being_completed (even if it is empty)
    // 4. last_word
    let word_being_completed = args[3].as_str();
    let last_word = args[4].as_str();

    if last_word == "cargo_crev_web_admin" {
        let sub_commands = vec![
            "--help",
            "-h",
            "trusted_from_crev_command",
            "trusted_from_files",
            "fetch",
            "add_trust",
            "delete_trust",
            "reindex",
            "add_blocklisted",
            "delete_blocklisted",
        ];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "list_and_sync" || last_word == "local_list" || last_word == "all_list" {
        let sub_commands = vec!["/mnt/d/DropboxBackup1"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    } else if last_word == "second_backup" {
        let sub_commands = vec!["/mnt/f/DropboxBackup2"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
     */
}

/// print help
fn print_help() {
    println!(
        r#"
{y}Welcome to cargo_crev_web_admin{rs}
Admin tasks for the cargo_crev_web server:
cargo_crev_web_admin trusted_from_crev_command - list the explicit trusted reviewers from cargo crev command
cargo_crev_web_admin trusted_from_files - list the explicit trusted reviewers from the /trust/*.crev files
cargo_crev_web_admin fetch - fetch the repos of explicit trusted reviewers 
cargo_crev_web_admin add_trust "url" - add a trusted repo url
cargo_crev_web_admin delete_trust "url" - delete a trusted repo url
cargo_crev_web_admin reindex - web app reads and reindex new or changed data 
cargo_crev_web_admin add_blocklisted "url", "note" - add repo_url to blocklisted
cargo_crev_web_admin delete_blocklisted "url" - delete repo_url from blocklisted

open-source: {g}https://github.com/LucianoBestia/cargo_crev_web_admin{rs}
    "#,
        g = *GREEN,
        y = *YELLOW,
        rs = *RESET,
    );
}
