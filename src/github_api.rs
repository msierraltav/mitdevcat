use reqwest;
use serde::{Deserialize, Serialize};
use web_sys::console;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub login: String,
    pub id: u64,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub company: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub blog: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: u32,
    pub public_gists: u32,
    pub followers: u32,
    pub following: u32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: Option<String>,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub language: Option<String>,
    pub forks_count: u32,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: u32,
    pub license: Option<serde_json::Value>,
    pub allow_forking: bool,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub visibility: String,
    pub default_branch: String,
}

#[derive(Debug, Clone)]
pub struct GitHubStats {
    pub total_repos: u32,
    pub total_stars: u32,
    pub total_forks: u32,
    pub total_commits: u32, // Esto será una estimación
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

impl Default for GitHubStats {
    fn default() -> Self {
        Self {
            total_repos: 0,
            total_stars: 0,
            total_forks: 0,
            total_commits: 0,
            public_repos: 0,
            followers: 0,
            following: 0,
        }
    }
}

pub async fn fetch_github_user(username: &str) -> Result<GitHubUser, String> {
    let url = format!("https://api.github.com/users/{}", username);
    
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<GitHubUser>().await {
                    Ok(user) => Ok(user),
                    Err(e) => {
                        console::error_1(&format!("Error parsing user data: {}", e).into());
                        Err(format!("Error parsing user data: {}", e))
                    }
                }
            } else {
                let error_msg = format!("API request failed with status: {}", response.status());
                console::error_1(&error_msg.clone().into());
                Err(error_msg)
            }
        }
        Err(e) => {
            console::error_1(&format!("Network error: {}", e).into());
            Err(format!("Network error: {}", e))
        }
    }
}

pub async fn fetch_github_repos(username: &str) -> Result<Vec<GitHubRepo>, String> {
    let mut all_repos = Vec::new();
    let mut page = 1;
    let per_page = 100;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?page={}&per_page={}&sort=updated&direction=desc",
            username, page, per_page
        );

        match reqwest::get(&url).await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<Vec<GitHubRepo>>().await {
                        Ok(repos) => {
                            if repos.is_empty() {
                                break;
                            }
                            all_repos.extend(repos);
                            page += 1;
                        }
                        Err(e) => {
                            console::error_1(&format!("Error parsing repos data: {}", e).into());
                            return Err(format!("Error parsing repos data: {}", e));
                        }
                    }
                } else {
                    let error_msg = format!("API request failed with status: {}", response.status());
                    console::error_1(&error_msg.clone().into());
                    return Err(error_msg);
                }
            }
            Err(e) => {
                console::error_1(&format!("Network error: {}", e).into());
                return Err(format!("Network error: {}", e));
            }
        }
    }

    Ok(all_repos)
}

pub async fn calculate_github_stats(username: &str) -> Result<GitHubStats, String> {
    // Fetch user data first
    let user = fetch_github_user(username).await?;
    
    // Then fetch repos
    let repos = fetch_github_repos(username).await?;

    // Calculate stats
    let total_stars: u32 = repos.iter().map(|repo| repo.stargazers_count).sum();
    let total_forks: u32 = repos.iter().map(|repo| repo.forks_count).sum();
    
    // Estimate commits (this is a rough estimate)
    // We'll count repositories that have been updated recently as active
    let active_repos = repos.iter().filter(|repo| !repo.fork && !repo.archived).count();
    let estimated_commits = (active_repos as u32) * 50; // Rough estimate of 50 commits per active repo

    Ok(GitHubStats {
        total_repos: repos.len() as u32,
        total_stars,
        total_forks,
        total_commits: estimated_commits,
        public_repos: user.public_repos,
        followers: user.followers,
        following: user.following,
    })
}
