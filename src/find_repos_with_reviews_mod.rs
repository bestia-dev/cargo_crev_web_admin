// find_repos_with_reviews_mod.rs

use serde_derive::{Deserialize, Serialize};
use unwrap::unwrap;

#[allow(dead_code)]
pub const RED: &str = "\x1b[31m";
#[allow(dead_code)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(dead_code)]
pub const GREEN: &str = "\x1b[32m";
#[allow(dead_code)]
pub const RESET: &str = "\x1b[0m";

#[derive(Serialize, Deserialize, Debug)]
struct ForkedRepo {
    html_url: String,
    contents_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoContent {
    name: String,
    r#type: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReposWithReviewsGithub {
    file_path: std::path::PathBuf,
    list: Vec<String>,
}

impl ReposWithReviewsGithub {
    /// get default path
    pub fn file_default_path() -> std::path::PathBuf {
        crate::utils_mod::get_data_dir().join("repos_with_reviews_github.json")
    }
    /// new empty with default path
    pub fn new() -> ReposWithReviewsGithub {
        ReposWithReviewsGithub {
            file_path: ReposWithReviewsGithub::file_default_path(),
            list: vec![],
        }
    }
    /// read from default json file
    pub fn read_from_default_file() -> ReposWithReviewsGithub {
        Self::read_from_specific_file(&ReposWithReviewsGithub::file_default_path())
    }
    /// force to read from specific folder. Use this only for tests.
    pub fn read_from_specific_file(file_path: &std::path::Path) -> ReposWithReviewsGithub {
        let content = unwrap!(std::fs::read_to_string(file_path));
        let mut list: Vec<String> = unwrap!(serde_json::from_str(&content));
        list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        // return
        ReposWithReviewsGithub {
            file_path: file_path.to_owned(),
            list,
        }
    }
    /// write to json file
    pub fn write(&mut self) {
        self.list
            .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        let json_pretty = unwrap!(serde_json::to_string_pretty(&self.list));
        unwrap!(std::fs::write(&self.file_path, &json_pretty));
    }
    pub fn list(&self) -> Vec<String> {
        self.list.clone()
    }
}

/// finds proof repos with reviews on github
/// store it in json file:
///  ~/.local/share/crev/proofs/github_com_web-crev-dev_crev-proofs-POHSrDcUUmA6qBxSX6zy1w/UpOPNplVEwBS2RhF7SS9gSP3bPJlfg-ZEoZ89gEMDwU/repos_with_reviews_github.json
pub fn find_repos_with_reviews_on_github() {
    let mut blocklisted_repos =
        super::blocklisted_repos_mod::BlocklistedRepos::read_from_default_file();
    let mut repos_with_reviews_github = ReposWithReviewsGithub::new();
    let client = reqwest::blocking::Client::new();

    let mut page = 1;
    // loop pages that represent forks
    loop {
        // there can be more pages. Max per_page is 100
        let url_for_page = &format!(
            "https://api.github.com/repos/crev-dev/crev-proofs/forks?per_page=100&page={page}"
        );

        let response = client
            .get(url_for_page)
            .header(
                "User-Agent",
                "cargo_crev_web_admin (github.com/bestia-dev/cargo_crev_web_admin)",
            )
            .header(
                "Authorization",
                &format!(
                    "Bearer {}",
                    std::env::var("GITHUB_TOKEN").expect("Error: missing env var GITHUB_TOKEN.")
                ),
            )
            .send()
            .unwrap();
        let response_text = response.text().unwrap_or("".to_string());
        if response_text.is_empty() {
            println!("{RED}Error for call to url: {url_for_page}{RESET}");
            break;
        } else {
            let vec_forked_repo: Vec<ForkedRepo> = serde_json::from_str(&response_text).unwrap();
            // TODO: use rayon to have more threads in parallel
            for forked_repo in vec_forked_repo.iter() {
                // "html_url": "https://github.com/dcsommer/crev-proofs",
                // "contents_url": "https://api.github.com/repos/dcsommer/crev-proofs/contents/{+path}",
                let url_for_content = forked_repo.contents_url.trim_end_matches("/{+path}");
                dbg!(url_for_content);
                check_repo_on_github(
                    forked_repo,
                    &client,
                    url_for_content,
                    &mut blocklisted_repos,
                    &mut repos_with_reviews_github,
                );
            }
            // the last page has less then 100 items
            if vec_forked_repo.len() < 100 {
                break;
            }
            page += 1;
        }
    }
    blocklisted_repos.write();
    repos_with_reviews_github.write();
}

fn check_repo_on_github(
    forked_repo: &ForkedRepo,
    client: &reqwest::blocking::Client,
    url_for_content: &str,
    blocklisted_repos: &mut crate::blocklisted_repos_mod::BlocklistedRepos,
    repos_with_reviews_github: &mut ReposWithReviewsGithub,
) {
    let response = client
        .get(url_for_content)
        .header(
            "User-Agent",
            "cargo_crev_web_admin (github.com/bestia-dev/cargo_crev_web_admin)",
        )
        .header(
            "Authorization",
            &format!("Bearer {}", unwrap!(std::env::var("GITHUB_TOKEN"))),
        )
        .send()
        .unwrap();
    let response_text = response.text().unwrap_or("".to_string());
    if response_text.is_empty() {
        // add this url to blocklist.json
        blocklisted_repos.add(&forked_repo.html_url, "url not exist");
        println!("{RED}Error for call to url: {url_for_content}{RESET}",);
    } else {
        let rsl = serde_json::from_str::<Vec<RepoContent>>(&response_text);
        match rsl {
            Err(_err) => {
                log::debug!("Cannot deserialize: {:?}", &response_text);
                blocklisted_repos.add(&forked_repo.html_url, "Cannot deserialize");
            }
            Ok(vec_repo_content) => {
                let mut count_ids = 0;
                for content in vec_repo_content.iter() {
                    // "name": "24YKeuThJDNFSlJyxcl5diSZcKcRbh-0zXM0YxTOFJw",
                    // "type": "dir",
                    if content.name.len() == 43 && content.r#type == "dir" {
                        count_ids += 1;
                        // dbg!("    {} {}", content.name, forked_repo.html_url);

                        repos_with_reviews_github
                            .list
                            .push(forked_repo.html_url.clone());
                    }
                }
                // if there is no id in the repo then add it to blocklisted
                if count_ids == 0 {
                    blocklisted_repos.add(&forked_repo.html_url, "no id");
                }
            }
        }
    }
}
