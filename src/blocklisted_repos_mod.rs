// blocklisted_repos_mod.rs

//! It is only one json file. Serialize and deserialize, read and write, add and delete

// TODO: move this json into ~/.crev...
// because /var/www is protected from normal user by permissions

use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use unwrap::unwrap;

// The debug build uses the files in `sample_data`
#[cfg(debug_assertions)]
lazy_static! {
    pub static ref BLOCKLISTED_REPOS_JSON: std::path::PathBuf =
        std::path::PathBuf::from("sample_data/blocklisted_repos.json");
}

// The Release build uses the files on the cargo_crev_web server:
#[cfg(not(debug_assertions))]
lazy_static! {
    pub static ref BLOCKLISTED_REPOS_JSON: std::path::PathBuf =
        std::path::PathBuf::from("/var/www/webapps/cargo_crev_web/blocklisted_repos.json");
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlocklistedRepos {
    file_path: std::path::PathBuf,
    list: Vec<(String, String)>,
}

impl BlocklistedRepos {
    /// read from default json file (different for debug and release)
    pub fn default() -> BlocklistedRepos {
        let file_path = (*BLOCKLISTED_REPOS_JSON).as_path();
        Self::force_open_specific_json_file(file_path)
    }
    /// force to read from specific folder. Use this only for tests.
    /// Use `default()` for normal code flow.
    pub fn force_open_specific_json_file(file_path: &std::path::Path) -> BlocklistedRepos {
        let content = unwrap!(std::fs::read_to_string(file_path));
        let mut list: Vec<(String, String)> = unwrap!(serde_json::from_str(&content));
        list.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        // return
        BlocklistedRepos {
            file_path: file_path.to_owned(),
            list,
        }
    }
    /// write to json file
    pub fn write(&mut self) {
        self.list
            .sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
        let blocklisted_repos_json_pretty = unwrap!(serde_json::to_string_pretty(&self.list));
        unwrap!(std::fs::write(
            &self.file_path,
            &blocklisted_repos_json_pretty
        ));
    }

    pub fn list(&self) -> Vec<(String, String)> {
        self.list.clone()
    }

    /// add repo
    /// if exists than delete first, so to have unique repo_urls
    pub fn add(&mut self, repo_url: &str, note: &str) {
        self.delete(repo_url);
        self.list.push((repo_url.to_string(), note.to_string()));
        self.list
            .sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    }
    /// delete repo_url (case insensitive)
    pub fn delete(&mut self, repo_url: &str) {
        self.list
            .retain(|x| x.0.to_lowercase() != repo_url.to_lowercase());
    }
    /// count
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.list.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use unwrap::unwrap;
    // Warning: Order of executing test functions is not fixed, so I must use 1 function if I change an external file sequentially
    // Warning: `cargo test` is executed from the debug build, but `cargo test --release` is executed from the release build
    // This 2 build modes have different paths to data files. Tests

    #[test]
    fn test_01() {
        // region: set initial content
        let file_path = std::path::Path::new("sample_data/blocklisted_repos.json");
        // copy the original file with the date/time
        let suffix = crate::datetime_now_for_file_names();
        let file_path_copy = format!("sample_data/blocklisted_repos_{}.json_copy", suffix);
        let file_path_copy = std::path::Path::new(&file_path_copy);
        std::fs::copy(file_path, file_path_copy).unwrap();

        let json = r#"[
            [
                "https://github.com/11ph22il/crev-proofs",
                "no id"
            ],
            [
                "https://github.com/2dav/crev-proofs",
                "no id"
            ]
        ]"#;
        unwrap!(std::fs::write(file_path, json));
        // endregion: set initial content
        // force open a specific json file
        let mut blocklisted = BlocklistedRepos::force_open_specific_json_file(file_path);

        assert_eq!(blocklisted.count(), 2);
        blocklisted.add("xxx", "xxx");
        assert_eq!(blocklisted.count(), 3);
        blocklisted.add("xxx", "yyy");
        assert_eq!(blocklisted.count(), 3);
        blocklisted.delete("xxx");
        assert_eq!(blocklisted.count(), 2);
        blocklisted.write();
        // return the renamed original file with the date/time
        std::fs::copy(file_path_copy, file_path).unwrap();
        std::fs::remove_file(file_path_copy).unwrap();
    }
}
