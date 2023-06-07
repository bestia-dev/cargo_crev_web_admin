//! cargo_crev_web_admin.rs

use cargo_crev_web_admin::*;
use std::env;

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
            _ => println!("{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}"),
        },
        Some("trusted_delete") => match env::args().nth(2).as_deref() {
            Some(url) => {
                let ns_started = ns_start("trusted_delete");
                trusted_delete(url);
                ns_print_ms("trusted_delete", ns_started);
            }
            _ => println!("{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}"),
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
                    _ => println!(
                        "{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}"
                    ),
                },
                _ => {
                    println!("{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}")
                }
            }
            ns_print_ms("blocklisted_add", ns_started);
        }
        Some("blocklisted_delete") => {
            let ns_started = ns_start("blocklisted_delete");
            match env::args().nth(2).as_deref() {
                Some(repo_url) => blocklisted_delete(repo_url),
                _ => {
                    println!("{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}")
                }
            }
            ns_print_ms("blocklisted_delete", ns_started);
        }
        Some("find_repos_with_reviews_on_github") => {
            let ns_started = ns_start("find_repos_with_reviews_on_github");
            find_repos_with_reviews_on_github();
            ns_print_ms("find_repos_with_reviews_on_github", ns_started);
        }
        Some("list_new_repos") => {
            let ns_started = ns_start("list_new_repos");
            list_new_repos();
            ns_print_ms("list_new_repos", ns_started);
        }

        Some("delete_untrusted_repos") => {
            let ns_started = ns_start("delete_untrusted_repos");
            delete_untrusted_repos();
            ns_print_ms("delete_untrusted_repos", ns_started);
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
        Some("publish_to_github") => {
            let ns_started = ns_start("publish_to_github");
            publish_to_github();
            ns_print_ms("publish_to_github", ns_started);
        }
        /*
        Some("list_and_sync") => match env::args().nth(2).as_deref() {
            Some(path) => {
                let ns_started = ns_start("list_and_sync");
                print!("{}", *CLEAR_ALL);
                list_and_sync(path);
                ns_print_ms("list_and_sync", ns_started);
            }
            _ => println!("{RED}Unrecognized arguments. Try cargo_crev_web_admin --help{RESET}"),
        },
         */
        _ => println!("{RED}Unrecognized arguments. Try `cargo_crev_web_admin --help`{RESET}"),
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
                println!("{sub_command}");
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{sub_command}");
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
            "find_repos_with_reviews_on_github",
            "delete_untrusted_repos",
            "fetch",
            "reindex",
            "publish_to_github",
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
    {YELLOW}Welcome to cargo_crev_web_admin{RESET}
    {YELLOW}Admin tasks for the cargo_crev_web server.{RESET}

    {YELLOW}Command line arguments:{RESET}
{GREEN}trusted_list              {RESET}{YELLOW}- list the explicit trusted reviewers from the /trust/*.crev files{RESET}
{GREEN}trusted_add "url"         {RESET}{YELLOW}- add a trusted repo url (needs CREV_PASSPHRASE){RESET}
{GREEN}trusted_delete "url"      {RESET}{YELLOW}- delete a trusted repo url{RESET}
{GREEN}trusted_from_crev_command {RESET}{YELLOW}- list the explicit trusted reviewers from cargo-crev command{RESET}

{GREEN}blocklisted_list          {RESET}{YELLOW}- list blocklisted{RESET}
{GREEN}blocklisted_add "url", "note" {RESET}{YELLOW}- add repo_url to blocklisted{RESET}
{GREEN}blocklisted_delete "url"  {RESET}{YELLOW}- delete repo_url from blocklisted{RESET}

    {YELLOW}To access GitHub, export GITHUB_TOKEN to env var:{RESET}
{GREEN}find_repos_with_reviews_on_github  {RESET}{YELLOW}- finds proof repos with reviews on github{RESET}
{GREEN}list_new_repos  {RESET}{YELLOW}- list the new repos to add manually to web.crev.dev{RESET}

{GREEN}delete_untrusted_repos            {RESET}{YELLOW}- delete fetched repos from /remote/ if they are not in trusted_list{RESET}
{GREEN}fetch                     {RESET}{YELLOW}- fetch the repos of explicit trusted reviewers {RESET}
{GREEN}reindex                   {RESET}{YELLOW}- web app reads and reindex new or changed data {RESET}
{GREEN}publish_to_github         {RESET}{YELLOW}- after changing trust files it is mandatory to publish this repo{RESET}

    {YELLOW}Type the crev passphrase into env variable (add one space before the command to avoid storing in bash history):{RESET}
{GREEN}export CREV_PASSPHRASE=xxx{RESET}

    {YELLOW}Save the autocompletion command in ~/.bashrc:{RESET}
{GREEN}complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin{RESET}

    {YELLOW}© bestia.dev 2023, MIT License, github.com/bestia-dev/cargo_crev_web_admin{RESET}
"#
    );
}

/// list the new repos to add to web.crev.dev
pub fn list_new_repos() {
    println!("List of new repos to add to web.crev.dev");
    println!("");

    let my_trusted_repos = cargo_crev_web_admin::list_new_repos();
    for x in my_trusted_repos {
        println!("{x}");
    }
}
