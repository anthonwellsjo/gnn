use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "login")]
    login: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "node_id")]
    node_id: String,

    #[serde(rename = "avatar_url")]
    avatar_url: String,

    #[serde(rename = "gravatar_id")]
    gravatar_id: String,

    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "followers_url")]
    followers_url: String,

    #[serde(rename = "following_url")]
    following_url: String,

    #[serde(rename = "gists_url")]
    gists_url: String,

    #[serde(rename = "starred_url")]
    starred_url: String,

    #[serde(rename = "subscriptions_url")]
    subscriptions_url: String,

    #[serde(rename = "organizations_url")]
    organizations_url: String,

    #[serde(rename = "repos_url")]
    repos_url: String,

    #[serde(rename = "events_url")]
    events_url: String,

    #[serde(rename = "received_events_url")]
    received_events_url: String,

    #[serde(rename = "type")]
    welcome1_type: String,

    #[serde(rename = "site_admin")]
    site_admin: bool,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "company")]
    company: String,

    #[serde(rename = "blog")]
    blog: String,

    #[serde(rename = "location")]
    location: String,

    #[serde(rename = "email")]
    email: String,

    #[serde(rename = "hireable")]
    hireable: bool,

    #[serde(rename = "bio")]
    bio: String,

    #[serde(rename = "twitter_username")]
    twitter_username: String,

    #[serde(rename = "public_repos")]
    public_repos: i64,

    #[serde(rename = "public_gists")]
    public_gists: i64,

    #[serde(rename = "followers")]
    followers: i64,

    #[serde(rename = "following")]
    following: i64,

    #[serde(rename = "created_at")]
    created_at: String,

    #[serde(rename = "updated_at")]
    updated_at: String,

    #[serde(rename = "private_gists")]
    private_gists: i64,

    #[serde(rename = "total_private_repos")]
    total_private_repos: i64,

    #[serde(rename = "owned_private_repos")]
    owned_private_repos: i64,

    #[serde(rename = "disk_usage")]
    disk_usage: i64,

    #[serde(rename = "collaborators")]
    collaborators: i64,

    #[serde(rename = "two_factor_authentication")]
    two_factor_authentication: bool,

    #[serde(rename = "plan")]
    plan: Plan,
}

#[derive(Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "space")]
    space: i64,

    #[serde(rename = "private_repos")]
    private_repos: i64,

    #[serde(rename = "collaborators")]
    collaborators: i64,
}
