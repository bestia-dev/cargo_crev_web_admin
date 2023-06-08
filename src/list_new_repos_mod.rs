// list_new_repos.rs

/// between repos_with_reviews_github and my_trusted_repos, find the new repos
/// that will be manually added
pub fn list_new_repos() -> Vec<String> {
    let my_trusted_repos = crate::my_trusted_repos_mod::MyTrustedRepos::new();
    let my_trusted_repos_list: Vec<String> = my_trusted_repos
        .list_from_files()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let repos_with_reviews_github =
        crate::repos_with_reviews_mod::ReposWithReviewsGithub::read_from_default_file();
    let repos_with_reviews_list = repos_with_reviews_github.list();

    let difference: Vec<String> = repos_with_reviews_list
        .into_iter()
        .filter(|item| !my_trusted_repos_list.contains(item))
        .collect();
    difference
}

/// list of blocklisted repos, that are maybe ok now.
/// Later you can manually unblock the repos if they are now ok.
pub fn list_repos_to_unblock() -> Vec<(String, String)> {
    let repos_with_reviews_github =
        crate::repos_with_reviews_mod::ReposWithReviewsGithub::read_from_default_file();
    let repos_with_reviews_list = repos_with_reviews_github.list();

    let blocklisted_repos =
        super::blocklisted_repos_mod::BlocklistedRepos::read_from_default_file();

    let difference: Vec<(String, String)> = blocklisted_repos
        .list()
        .into_iter()
        .filter(|item| repos_with_reviews_list.contains(&item.0))
        .collect();
    difference
}
