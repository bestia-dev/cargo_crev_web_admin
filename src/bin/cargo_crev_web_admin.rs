//! cargo_crev_web_admin.rs

use cargo_crev_web_admin::*;
use std::env;

fn main() {
    pretty_env_logger::init();

    match env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(),
        Some("completion") => completion(),       
        Some("trusted_list") => {
            let ns_started = ns_start("trusted_list");
            trusted_list();
            ns_print_ms("trusted_list", ns_started);
        }
        Some("trusted_add") => match env::args().nth(2).as_deref() {
            Some(url) => {
                let ns_started = ns_start("trusted_add");
                trusted_add(url);
                ns_print_ms("trusted_add", ns_started);
            }
            _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
        },
        Some("trusted_delete") => match env::args().nth(2).as_deref() {
            Some(url) => {
                let ns_started = ns_start("trusted_delete");
                trusted_delete(url);
                ns_print_ms("trusted_delete", ns_started);
            }
            _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
        },
        Some("trusted_from_crev_command") => {
            let ns_started = ns_start("trusted_from_crev_command");
            trusted_from_crev_command();
            ns_print_ms("trusted_from_crev_command", ns_started);
        }      
        Some("blocklisted_list") => {
            let ns_started = ns_start("blocklisted_list");
            blocklisted_list();
            ns_print_ms("blocklisted_list", ns_started);
        }
        Some("blocklisted_add") => {
            let ns_started = ns_start("blocklisted_add");
            match env::args().nth(2).as_deref() {
                Some(repo_url) => match env::args().nth(3).as_deref() {
                    Some(note) => blocklisted_add(repo_url, note),
                    _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
                },
                _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
            }
            ns_print_ms("blocklisted_add", ns_started);
        }
        Some("blocklisted_delete") => {
            let ns_started = ns_start("blocklisted_delete");
            match env::args().nth(2).as_deref() {
                Some(repo_url) => blocklisted_delete(repo_url),
                _ => println!("Unrecognized arguments. Try cargo_crev_web_admin --help"),
            }
            ns_print_ms("blocklisted_delete", ns_started);
        }
        Some("fetch") => {
            let ns_started = ns_start("fetch");
            fetch();
            ns_print_ms("fetch", ns_started);
        }
        Some("reindex") => {
            let ns_started = ns_start("reindex");
            reindex();
            ns_print_ms("reindex", ns_started);
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
            "trusted_list",
            "trusted_add",
            "trusted_delete",
            "trusted_from_crev_command",
            "blocklisted_list",
            "blocklisted_add",
            "blocklisted_delete",
            "fetch",
            "reindex",
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
Admin tasks for the cargo_crev_web server.

Command line arguments:
trusted_list              - list the explicit trusted reviewers from the /trust/*.crev files
trusted_add "url"         - add a trusted repo url (needs CREV_PASSPHRASE)
trusted_delete "url"      - delete a trusted repo url
trusted_from_crev_command - list the explicit trusted reviewers from cargo crev command

blocklisted_list          - list blocklisted
blocklisted_add "url", "note" - add repo_url to blocklisted
blocklisted_delete "url"  - delete repo_url from blocklisted

fetch                     - fetch the repos of explicit trusted reviewers 
reindex                   - web app reads and reindex new or changed data 


Put the autocompletion command in ~/.bash_profile or ~/.profile:
complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin

Put the crev passphrase in env variable (add a space before to avoid storing in bash history):
$ export CREV_PASSPHRASE=xxx

open-source: {g}https://github.com/LucianoBestia/cargo_crev_web_admin{rs}
    "#,
        g = *GREEN,
        y = *YELLOW,
        rs = *RESET,
    );
}
