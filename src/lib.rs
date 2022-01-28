// cargo_crev_web_admin lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_web_admin
//!
//! **Admin CLI for cargo_crev_web**  
//! ***[repository](https://github.com/lucianobestia/cargo_crev_web_admin/); version: 2022.126.1230  date: 2022-01-26 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-157-green.svg)]()
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-18-blue.svg)]()
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-18-purple.svg)]()
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)]()
//!
//! [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/cargo_crev_web_admin/blob/main/LICENSE) [![Rust](https://github.com/LucianoBestia/cargo_crev_web_admin/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/cargo_crev_web_admin/)
//!
//! ## cargo_crev_web_admin CLI
//!
//! The web app cargo_crev_web on <https://web.crev.dev> fetches all proof repos it can find and shows the crate reviews online.  
//! Some admin tasks are needed and I don't want them to be accessible on the web.  
//! This will be a CLI app that can be used when logged on the linux terminal over SSH.  
//! So is sure that only an admin, who can log in on to the server, can use this tasks.
//!
//! 1. delete from `trust` someone from blocklist json (case insensitive)
//! 2. Delete from \\\Secure FTP\google_cloud\home\luciano_bestia\.cache\crev\remotes\
//! folders of reviewers that are not in
//! \\\Secure FTP\google_cloud\home\luciano_bestia\config\crev\proofs\github_com_cargo-crev-web_crev-proofs-..\...\trust\
//! 3. short command for cargo crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
//! 4. short command for cargo crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1
//! 5. short command for web reindex
//! 6. short command for cargo crev trust --level low <https://github.com/Alxandr/crev-proofs>
//! 7. command to add to blocklist
//! 8. command to delete from blocklist
//! 9. Integrity - warnings if a review have incorrect url or ID
//!
//! ## Development
//!
//! I use [cargo-auto](https://crates.io/crates/cargo-auto) for automation tasks in rust language. Install it:
//!
//! ```bash
//! cargo install cargo-auto
//! ```
//!
//! List user-defined automation tasks in `automation_tasks_rs`:
//!
//! ```bash
//! cargo auto
//! ```
//!
//! ## bash auto-completion
//!
//! This executable is prepared for auto-completion in bash.  
//! Run this command to define auto-completion in bash for the current session:  
//!
//! ```bash
//! complete -C "cargo_crev_web_admin completion" cargo_crev_web_admin
//! ```
//!
//! To make it permanent add this command to the file `~/.bashrc` or some other file that runs commands on bash initialization.  
//!
//! ## TODO
//!
//! all
//!
//! ## cargo crev reviews and advisory
//!
//! Please, spread this info !\
//! Open source code needs a community effort to express trustworthiness.\
//! Start with reading the reviews of the crates on [web.crev.dev](https://web.crev.dev/rust-reviews/crates). \
//! Then install the GUI [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) or the CLI [cargo-crev](https://github.com/crev-dev/cargo-crev)\.  
//! Your personal reviews are most important. If you have a boss, he will sooner or later ask you to show him your reviews for all the dependencies you use. With [cargo_crev_reviews](https://crates.io/crates/cargo_crev_reviews) you have a basic tool to do that. \
//! Write your reviews! Describe the crates you trust and why. Or warn about the crate versions you think are dangerous. Publish and share your opinion with other developers.\
//!
//! ## open-source and free as a beer
//!
//! My open-source projects are free as a beer (MIT license).  
//! I just love programming.  
//! But I need also to drink. If you find my projects and tutorials helpful,  
//! please buy me a beer or two donating on my [paypal](https://www.paypal.com/paypalme/LucianoBestia).  
//! You know the price of a beer in your local bar ;-)  
//! So I can drink a free beer for your health :-)  
//! [Na zdravje](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) !
//!
// endregion: auto_md_to_doc_comments include README.md A //!

mod blocklisted_repos_mod;
mod my_trusted_repos_mod;
mod utils_mod;

pub use utils_mod::*;

// use unwrap::unwrap;

use crate::{blocklisted_repos_mod::BlocklistedRepos, my_trusted_repos_mod::MyTrustedRepos};

/// list the explicit trusted reviewers from cargo crev command
pub fn trusted_from_crev_command() {
    println!("List of explicit trusted reviewers from the cargo crev command");
    println!("Warning: It shows also implicitly myself as high trust.");
    println!("$ cargo crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1");
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_crev_command();

    let line_count = count_newlines(&output);
    println!("{}\nLine count: {}", output, line_count);
}

/// list the explicit trusted reviewers from the /trust/*.crev files
pub fn trusted_list() {
    println!("List of explicit trusted reviewers from the /trust/*.crev files");
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_files();

    let line_count = count_newlines(&output);
    println!("{}\nLine count: {}", output, line_count);
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

/// fetch the explicit trusted reviewers from the /trust/*.crev files
pub fn fetch() {
    println!("Fetch the explicit trusted reviewers from the /trust/*.crev files");
    println!(
        "Warning: It will try to fetch also `myself`, but the local folder is deleted on purpose."
    );
    println!(
        "$ cargo crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1"
    );
    println!("");

    let output = std::process::Command::new("cargo")
        .args([
            "crev",
            "repo",
            "fetch",
            "trusted",
            "--high-cost",
            "1",
            "--medium-cost",
            "1",
            "--low-cost",
            "1",
            "--depth",
            "1",
        ])
        .output()
        .unwrap();
    let output = format!(
        "{} {}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
    let line_count = count_newlines(&output);
    println!("{}\nLine count: {}", output, line_count);
}

/// add new trusted repo
pub fn trusted_add(repo_url: &str) {
    println!("Add a trusted repo url.");
    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.trusted_add(repo_url);

    println!("{}", output);
}

/// delete from trusted repo
pub fn trusted_delete(repo_url: &str) {
    println!("Delete from trusted repo.");
    let my_trusted_repos = MyTrustedRepos::new();
    my_trusted_repos.trusted_delete(repo_url);
}

/// web app reads and reindex new or changed data
pub fn reindex() {
    println!("Web app reads and reindex new or changed data.");
    // curl --silent https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/
    let _output = std::process::Command::new("curl")
        .arg("--silent")
        .arg("https://bestia.dev/rust-reviews/reserved_folder/reindex_after_fetch_new_reviews/")
        .output()
        .unwrap();
    println!("Reindex finished.");
}

/// list of blocklisted
pub fn blocklisted_list() {
    println!("List of blocklisted");
    println!("");

    let bl = BlocklistedRepos::default();
    let mut output=String::new();
    for x in bl.list().iter(){
        output.push_str(&x.0);
        output.push_str("      ");
        output.push_str(&x.1);
        output.push('\n');
    }
    let line_count = count_newlines(&output);
    println!("{}\nLine count: {}", output, line_count);
}

/// add new blocklist repo
pub fn blocklisted_add(repo_url: &str, note: &str) {
    println!("Add blocklisted repo url.");
    let mut bl = BlocklistedRepos::default();
    bl.add(repo_url, note);
    bl.write();
    println!("Added to blocklist.");
}

/// delete from blocklist repo
pub fn blocklisted_delete(repo_url: &str) {
    println!("Delete from blocklisted repo.");
    let mut bl = BlocklistedRepos::default();
    bl.delete(repo_url);
    bl.write();
    println!("Deleted from blocklist.");
}
