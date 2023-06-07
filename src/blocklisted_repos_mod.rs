// blocklisted_repos_mod.rs

//! It is only one json file. Serialize and deserialize, read and write, add and delete

/// store it in json file:
/// ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/blocklisted_repos.json
use serde_derive::{Deserialize, Serialize};
use unwrap::unwrap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlocklistedRepos {
    file_path: std::path::PathBuf,
    list: Vec<(String, String)>,
}

impl BlocklistedRepos {
    /// get default path
    pub fn file_default_path() -> std::path::PathBuf {
        crate::utils_mod::get_data_dir().join("blocklisted_repos.json")
    }
    /// read from default json file
    pub fn read_from_default_file() -> BlocklistedRepos {
        Self::read_from_specific_file(&BlocklistedRepos::file_default_path())
    }
    /// force to read from specific folder. Use this only for tests.
    pub fn read_from_specific_file(file_path: &std::path::Path) -> BlocklistedRepos {
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
        let file_path_copy = format!("sample_data/blocklisted_repos_{suffix}.json_copy");
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
        let mut blocklisted = BlocklistedRepos::read_from_specific_file(file_path);

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
