use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ApiResp {
    sha: String,
    node_id: String,
    pub(crate) commit: Commit,
    url: String,
    html_url: String,
    comments_url: String,
    author: ApiRespAuthor,
    committer: ApiRespAuthor,
    parents: Vec<Parents>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Commit {
    pub(crate) author: Author,
    committer: Author,
    message: String,
    tree: Tree,
    url: String,
    comment_count: u32,
    verification: Verification,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Author {
    name: String,
    email: String,
    pub(crate) date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tree {
    sha: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Verification {
    verified: bool,
    reason: String,
    signature: Option<String>,
    payload: Option<String>,
    verified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiRespAuthor {
    login: String,
    id: u32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    type_: String,
    user_view_type: String,
    site_admin: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Parents {
    sha: String,
    url: String,
    html_url: String,
}
