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
        crate::find_repos_with_reviews_mod::ReposWithReviewsGithub::read_from_default_file();
    let repos_with_reviews_list = repos_with_reviews_github.list();

    let difference: Vec<String> = repos_with_reviews_list
        .into_iter()
        .filter(|item| !my_trusted_repos_list.contains(item))
        .collect();
    difference
}
