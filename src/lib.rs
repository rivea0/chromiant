use anyhow::{Context, Result, bail};
use colorsys::{Hsl, Rgb};
use csv::Reader;
use jiff::{SignedDuration, Timestamp};
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use std::{
    fs,
    io::Write,
    path::Path,
    time::{Duration, SystemTime},
};

mod api_resp;
mod helpers;

use crate::api_resp::ApiResp;
use crate::helpers::{get_data_dir, get_data_path, is_recent};

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Color {
    pub name: String,
    pub hex: String,
    #[serde(rename = "good name")]
    pub good_name: Option<char>,
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
    let path = get_data_path()?;
    let mut rdr = Reader::from_path(path).context("File not found")?;
    let results = rdr.deserialize();
    let colors = results
        .filter_map(|result| result.ok())
        .filter(|color: &Color| color.name.contains(s))
        .collect::<Vec<Color>>();

    Ok((!colors.is_empty()).then_some(colors))
}

// Names and hex values are unique
fn search(query: &SearchQuery) -> Result<Option<Color>> {
    let path = get_data_path()?;
    let mut rdr = Reader::from_path(path).context("File not found")?;
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

pub fn by_rgb(r: u8, g: u8, b: u8) -> Result<Option<Color>> {
    let rgb = Rgb::from([r, g, b]);
    let hex = rgb.to_hex_string();

    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: hex,
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

pub fn by_hsl(h: f64, s: f64, l: f64) -> Result<Option<Color>> {
    let hsl = Hsl::from(&(h * 360.0, s * 100.0, l * 100.0));
    let rgb = Rgb::from(&hsl);
    let hex = rgb.to_hex_string();

    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: hex,
    };

    write_data_to_file()?;

    let result = search(&search_query)?;

    Ok(result)
}

fn get_colornames_data_from_remote_src(timeout: Duration) -> Result<Response, reqwest::Error> {
    let client = Client::builder().timeout(timeout).build()?;

    client
        .get("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv")
        .send()
}

fn get_modified_timestamp_of_remote_src() -> Result<Timestamp> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()?;

    let req = client
            .get("https://api.github.com/repos/meodai/color-names/commits?path=src/colornames.csv&per_page=1")
            .send()?;
    let api_resp = req.json::<Vec<ApiResp>>()?;
    let Some(last_commit) = api_resp.first() else {
        anyhow::bail!("Can't get last commit");
    };

    let orig_file_modified: Timestamp = last_commit.commit.author.date.parse()?;

    Ok(orig_file_modified)
}

// Fetch data from remote source and write to file.
// Assumes that the data directory exists.
fn update_local_data_file() -> Result<()> {
    let file_path = get_data_path()?;

    match get_colornames_data_from_remote_src(Duration::from_secs(30)) {
        Ok(res) => {
            let data = res.text()?;
            println!("Got data from remote source file");
            if !file_path.is_file() {
                println!(
                    "Local data file doesn't exist, creating it at {}",
                    file_path.display()
                );
                let data_dir = get_data_dir();
                fs::File::create(Path::new(&data_dir).join("colornames.csv"))?;
            }
            println!("Writing to file at {}.", file_path.display());
            let mut f = fs::File::options().write(true).open(file_path)?;
            f.write_all(data.as_bytes())?;
        }
        Err(e) => {
            if e.is_timeout() {
                if !file_path.is_file() {
                    bail!(
                        "Connection timed out, and {} does not exist",
                        file_path.display()
                    );
                } else {
                    eprintln!(
                        "Connection timed out, using the existing file at {}",
                        file_path.display()
                    );
                }
            }
        }
    }

    Ok(())
}

// Create the data directory if it doesn't exist and write to file.
// If the data dir exists, check if the file is recent, and if not, write to file.
fn write_data_to_file() -> Result<()> {
    let data_dir = get_data_dir();
    if !data_dir.is_dir() {
        println!(
            "Creating colornames.csv at {}",
            data_dir.as_os_str().display()
        );

        fs::create_dir(&data_dir)?;

        update_local_data_file()?;
    } else {
        let file_path = get_data_path()?;
        if !file_path.is_file() {
            update_local_data_file()?;
        }
        let metadata = std::fs::metadata(&file_path)?;
        let file_modified = metadata.modified()?;
        let d = SignedDuration::system_until(SystemTime::UNIX_EPOCH, file_modified)?;
        let file_modified = Timestamp::from_duration(d)?;

        if is_recent(file_modified) {
            println!("Using existing file at {}", file_path.display());
            return Ok(());
        }

        let orig_file_modified = get_modified_timestamp_of_remote_src()?;

        if file_modified < orig_file_modified {
            println!(
                "Updating colornames.csv at {}",
                file_path.as_os_str().display()
            );

            update_local_data_file()?;
        }
    }

    Ok(())
}
