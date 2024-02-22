// cargo_crev_web_admin lib.rs

#![doc=include_str!("../README.md")]

mod blocklisted_repos_mod;
mod list_new_repos_mod;
mod my_trusted_repos_mod;
mod repos_with_reviews_mod;
mod utils_mod;

// re-export
pub use list_new_repos_mod::list_new_repos;
pub use list_new_repos_mod::list_repos_to_unblock;
pub use repos_with_reviews_mod::find_repos_with_reviews_on_github;
pub use utils_mod::*;

// use unwrap::unwrap;
use crate::{blocklisted_repos_mod::BlocklistedRepos, my_trusted_repos_mod::MyTrustedRepos};
use lazy_static::lazy_static;

lazy_static! {
    // The Linux home folder ~ or /home/username
    pub static ref HOME_DIR:std::path::PathBuf = home::home_dir().unwrap();
    pub static ref CREV_REMOTES_DIR: std::path::PathBuf = HOME_DIR.join(".cache/crev/remotes");
}

/// list the explicit trusted reviewers from cargo-crev command
pub fn trusted_from_crev_command() {
    println!("List of explicit trusted reviewers from the cargo-crev command");
    println!("Warning: It shows also implicitly myself as high trust.");
    println!(
        "$ cargo-crev crev id query trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1"
    );
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_crev_command();

    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

/// list the explicit trusted reviewers from the /trust/*.crev files
pub fn trusted_list() {
    println!("List of explicit trusted reviewers from the /trust/*.crev files");
    println!("");

    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.list_from_files();

    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

/// delete fetched repos from /remote/ if they are not in trusted_list
pub fn delete_untrusted_repos() {
    println!("Delete fetched repos from /remote/ if they are not in trusted_list.");
    let mut output = String::new();
    let my_trusted_repos = MyTrustedRepos::new();
    let trusted_list = my_trusted_repos.list_from_files();

    for entry in CREV_REMOTES_DIR.read_dir().unwrap() {
        let entry = entry.unwrap();
        let entry_name = entry.file_name();
        let entry_name = entry_name.to_string_lossy();
        let mut is_found = false;
        for trusted_url in trusted_list.lines() {
            let trusted_name = trusted_url
                .trim_start_matches("https://")
                .trim_start_matches("http://")
                .trim_end_matches(".git")
                .replace("/", "_")
                .replace(".", "_")
                .replace("~", "_");
            if entry_name
                .to_lowercase()
                .starts_with(&trusted_name.to_lowercase())
            {
                is_found = true;
            }
        }
        if is_found == false {
            output.push_str(&format!("rm -rf {:#?}\n", &entry.path()));
        }
    }
    if !output.is_empty() {
        println!("Run these commands manually in bash:\n{output}");
    }

    println!("delete_untrusted_repos finished.");
}

/// fetch the explicit trusted reviewers from the /trust/*.crev files
pub fn fetch() {
    println!("Fetch the explicit trusted reviewers from the /trust/*.crev files");
    println!(
        "Warning: It will try to fetch also `myself`, but the local folder is deleted on purpose."
    );
    println!(
        "$ cargo-crev crev repo fetch trusted --high-cost 1 --medium-cost 1 --low-cost 1 --depth 1"
    );
    println!("");

    let output = std::process::Command::new("cargo-crev")
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
    println!("{output}\nLine count: {line_count}");
}

/// add new trusted repo
pub fn trusted_add(repo_url: &str) {
    println!("Add a trusted repo url.");
    let my_trusted_repos = MyTrustedRepos::new();
    let output = my_trusted_repos.trusted_add(repo_url);

    println!("{output}");
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

/// after changing trust files it is mandatory to publish this repo
pub fn publish_to_github() {
    println!("After changing trust files it is mandatory to publish this repo.");
    println!("Because crev uses the fetched files from remotes only, not the local copy, even for my repo.");
    let output = std::process::Command::new("cargo-crev")
        .arg("crev")
        .arg("publish")
        .output()
        .unwrap();
    let output = format!(
        "{} {}",
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap()
    );
    println!("{output}");
    if output.contains("git@github.com: Permission denied (publickey).") {
        println!("If you don't have permission to write to github, then we need to run ssh-agent and ssh-add:");
        println!("$ eval `ssh-agent`; ssh-add ~/.ssh/bestia2_for_github");
        println!("Enter your ssh passphrase for github and finally repeat:");
        println!("$ cargo_crev_web_admin publish");
    }
}

/// list of blocklisted
pub fn blocklisted_list() {
    println!("List of blocklisted");
    println!("");

    let bl = BlocklistedRepos::read_from_default_file();
    let mut output = String::new();
    for x in bl.list().iter() {
        output.push_str(&x.0);
        output.push_str("      ");
        output.push_str(&x.1);
        output.push('\n');
    }
    let line_count = count_newlines(&output);
    println!("{output}\nLine count: {line_count}");
}

/// add new blocklist repo
pub fn blocklisted_add(repo_url: &str, note: &str) {
    println!("Add blocklisted repo url.");
    let mut bl = BlocklistedRepos::read_from_default_file();
    bl.add(repo_url, note);
    bl.write();
    println!("Added to blocklist.");
}

/// delete from blocklist repo
pub fn blocklisted_delete(repo_url: &str) {
    println!("Delete from blocklisted repo.");
    let mut bl = BlocklistedRepos::read_from_default_file();
    bl.delete(repo_url);
    bl.write();
    println!("Deleted from blocklist.");
}
