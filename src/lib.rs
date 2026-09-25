use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use csv::Reader;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{io::Write, time::Duration};

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Color {
    pub name: String,
    pub hex: String,
    #[serde(rename = "good name")]
    pub good_name: Option<String>,
}

#[derive(Debug)]
enum SearchType {
    ByExactName,
    ByHex,
}

#[derive(Debug)]
struct SearchQuery {
    search_type: SearchType,
    query: String,
}

fn search_multiple(s: &str) -> Result<Option<Vec<Color>>> {
    let mut rdr = Reader::from_path("./colornames.csv")?;
    let results = rdr.deserialize();
    let colors = results
        .filter_map(|result| result.ok())
        .filter(|color: &Color| color.name.contains(s))
        .collect::<Vec<Color>>();

    Ok((!colors.is_empty()).then_some(colors))
}

// Names and hex values are unique
fn search(query: &SearchQuery) -> Result<Option<Color>> {
    let p = "./colornames.csv";
    let mut rdr = Reader::from_path(p).context("File not found")?;
    let results = rdr.deserialize();

    let color = match query.search_type {
        SearchType::ByExactName => results
            .filter_map(|result| result.ok())
            .find(|color: &Color| color.name == query.query),
        SearchType::ByHex => results
            .filter_map(|result| result.ok())
            .find(|color: &Color| color.hex == query.query),
    };

    Ok(color)
}

pub fn by_exact_name(s: &str) -> Result<Option<Color>> {
    let search_query = SearchQuery {
        search_type: SearchType::ByExactName,
        query: s.to_string(),
    };

    write_data_to_file()?;

    let result = search(&search_query)?;

    Ok(result)
}

pub fn by_hex(s: &str) -> Result<Option<Color>> {
    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: s.to_string(),
    };

    write_data_to_file()?;

    let result = search(&search_query)?;

    Ok(result)
}

// Get colors that includes the pattern in name
pub fn by_name(pattern: &str) -> Result<Option<Vec<Color>>> {
    write_data_to_file()?;

    let results = search_multiple(pattern)?;

    Ok(results)
}

fn write_colornames_data() -> Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(30)).build()?;

    let req = client
        .get("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv")
        .send()?;

    // If timeout, don't do anything, `search` will use the existing file
    let result = req.text()?;

    let mut f = std::fs::File::create("./colornames.csv")?;
    f.write_all(result.as_bytes())?;

    Ok(())
}

fn write_data_to_file() -> Result<()> {
    let file_path = std::path::Path::new("./colornames.csv");
    if !file_path.is_file() {
        println!(
            "Creating colornames.csv at {}",
            file_path.as_os_str().display()
        );
        write_colornames_data()?;
    } else {
        // Get the modified info of the local file
        let metadata = std::fs::metadata(file_path)?;
        let file_modified = metadata.modified()?;
        let file_modified = file_modified
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let file_modified = i64::try_from(file_modified)?;

        // Get the remote file's modified info
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("test package")
            .build()?;

        let req = client
            .get("https://api.github.com/repos/meodai/color-names/commits?path=src/colornames.csv&per_page=1")
            .send()?;
        let api_resp = req.json::<Vec<ApiResp>>()?;
        let Some(last_commit) = api_resp.first() else {
            anyhow::bail!("Can't get last commit");
        };

        let orig_file_modified = DateTime::parse_from_rfc3339(&last_commit.commit.author.date)?
            .with_timezone(&Utc)
            .timestamp();

        // If the remote file is more recent, write to file
        if file_modified < orig_file_modified {
            println!(
                "Updating colornames.csv at {}",
                file_path.as_os_str().display()
            );
            write_colornames_data()?;
        }
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResp {
    sha: String,
    node_id: String,
    commit: Commit,
    url: String,
    html_url: String,
    comments_url: String,
    author: ApiRespAuthor,
    committer: ApiRespAuthor,
    parents: Vec<Parents>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    author: Author,
    committer: Author,
    message: String,
    tree: Tree,
    url: String,
    comment_count: u32,
    verification: Verification,
}

#[derive(Debug, Serialize, Deserialize)]
struct Author {
    name: String,
    email: String,
    date: String,
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
