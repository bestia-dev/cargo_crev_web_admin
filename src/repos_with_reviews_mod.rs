// repos_with_reviews_mod.rs

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
struct RepoFileMetadata {
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
    let client = reqwest::blocking::Client::new();

    let mut page = 1;
    let mut vec_forked_repo = vec![];

    // loop pages that represent forks
    loop {
        // there can be more pages. Max per_page is 100
        println!("github api getting page: {page}...");
        let url_for_page = format!(
            "https://api.github.com/repos/crev-dev/crev-proofs/forks?per_page=100&page={page}"
        );
        let response_text = api_request(&client, &url_for_page);
        let mut vec_on_page: Vec<ForkedRepo> = serde_json::from_str(&response_text).unwrap();
        let item_on_page = vec_on_page.len();
        dbg!(item_on_page);
        vec_forked_repo.append(&mut vec_on_page);
        // the last page should have less then 100 items
        if item_on_page < 100 {
            break;
        }
        page += 1;
    }

    // read this from existing file, because items are added manually
    let mut blocklisted_repos =
        super::blocklisted_repos_mod::BlocklistedRepos::read_from_default_file();
    // start this with empty vector, because it has no manual entries
    let mut repos_with_reviews_github = ReposWithReviewsGithub::new();
    // TODO: use rayon to have more threads in parallel
    for forked_repo in vec_forked_repo.iter() {
        // "html_url": "https://github.com/dcsommer/crev-proofs",
        // "contents_url": "https://api.github.com/repos/dcsommer/crev-proofs/contents/{+path}",
        let url_for_content = forked_repo.contents_url.trim_end_matches("/{+path}");
        check_repo_on_github(
            forked_repo,
            &client,
            url_for_content,
            &mut blocklisted_repos,
            &mut repos_with_reviews_github,
        );
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
    // JUST_FOR_DEBUG: let url_for_content = "https://api.github.com/repos/bestia-dev/crev-proofs/contents";
    dbg!(url_for_content);
    let files_metadata = get_files_metadata(client, url_for_content);

    // one repository can have many ids. That makes things unnecessarily more complex.
    let mut count_ids = 0;
    for file_metadata in files_metadata.iter() {
        // "name": "24YKeuThJDNFSlJyxcl5diSZcKcRbh-0zXM0YxTOFJw",
        // "type": "dir",
        if file_metadata.name.len() == 43 && file_metadata.r#type == "dir" {
            // does exist reviews dir?
            let crev_id = file_metadata.name.clone();
            dbg!(&crev_id);
            let url_for_content = format!("{url_for_content}/{crev_id}");
            let files_metadata = get_files_metadata(client, &url_for_content);
            for file_metadata in files_metadata.iter() {
                if file_metadata.r#type == "dir" && file_metadata.name == "reviews" {
                    // check if there are files with reviews
                    let url_for_content = format!("{url_for_content}/reviews");
                    let files_metadata = get_files_metadata(client, &url_for_content);
                    for file_metadata in files_metadata.iter() {
                        if file_metadata.r#type != "dir"
                            && file_metadata.name.ends_with(".proof.crev")
                        {
                            // found files for reviews.
                            println!("{GREEN}Id {crev_id} has the dir reviews{RESET}");
                            count_ids += 1;
                            break;
                        }
                    }
                    break;
                }
            }
        }
    }
    // if there is no id in the repo then add it to blocklisted
    if count_ids == 0 {
        println!("{RED}Repo does not have Ids with reviews.{RESET}");
        blocklisted_repos.add(&forked_repo.html_url, "no reviews");
    } else {
        repos_with_reviews_github
            .list
            .push(forked_repo.html_url.clone());
    }
}

/// call an api request on github
fn api_request(client: &reqwest::blocking::Client, url: &str) -> String {
    let response = client
        .get(url)
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
        println!("{RED}Error: call api request: {url}{RESET}");
    }
    response_text
}

/// get files under a directory from github api.
/// Technically a directory is just a special type of file.
fn get_files_metadata(
    client: &reqwest::blocking::Client,
    url_for_content: &str,
) -> Vec<RepoFileMetadata> {
    let response_text = api_request(client, url_for_content);
    let files_metadata = if response_text.is_empty() {
        log::error!("{RED}Error on call to url: {url_for_content}{RESET}");
        vec![]
    } else {
        serde_json::from_str::<Vec<RepoFileMetadata>>(&response_text).unwrap_or_else(|_er| {
            log::error!("Cannot deserialize: {:?}", &response_text);
            vec![]
        })
    };
    files_metadata
}
