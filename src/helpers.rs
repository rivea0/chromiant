use anyhow::{Result, bail};
use dirs::data_local_dir;
use jiff::{SignedDuration, Timestamp};
use reqwest::blocking::{Client, Response};

use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    time::{Duration, SystemTime},
};

use crate::api_resp::ApiResp;

fn is_recent(timestamp: Timestamp) -> bool {
    let age = timestamp.duration_until(Timestamp::now());
    let day = SignedDuration::from_hours(24);

    age >= SignedDuration::ZERO && age < day
}

pub(crate) fn get_data_path() -> Result<PathBuf> {
    let dir = get_data_dir();

    Ok(dir.join("colornames.csv"))
}

fn get_data_dir() -> PathBuf {
    let mut dir = match data_local_dir() {
        Some(dir) => dir,
        None => std::env::current_dir()
            .expect("Failed to get current dir")
            .to_path_buf(),
    };

    dir.push(env!("CARGO_PKG_NAME"));

    dir
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
pub(crate) fn update_local_data_file(file_path: &Path) -> Result<()> {
    match get_colornames_data_from_remote_src(Duration::from_secs(30)) {
        Ok(res) => {
            if !file_path.is_file() {
                println!(
                    "Local data file doesn't exist, creating it at {}",
                    file_path.display()
                );
                let Some(data_dir) = file_path.parent() else {
                    bail!("Couldn't get parent directory");
                };
                fs::File::create(Path::new(&data_dir).join("colornames.csv"))?;
            }
            let data = res.text()?;
            println!("Got data from remote source file");
            println!("Writing to file at {}", file_path.display());
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
pub(crate) fn write_data_to_file(file_path: &Path) -> Result<()> {
    let Some(data_dir) = file_path.parent() else {
        bail!("Couldn't get parent directory");
    };

    if !data_dir.is_dir() {
        println!("Creating data directory {}", data_dir.as_os_str().display());

        fs::create_dir(data_dir)?;

        update_local_data_file(file_path)?;
    } else {
        let file_path = get_data_path()?;
        if !file_path.is_file() {
            update_local_data_file(&file_path)?;
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

            update_local_data_file(&file_path)?;
        }
    }

    Ok(())
}

pub(crate) fn validate_hex_string(hex_str: &str) -> Result<bool> {
    if hex_str.chars().count() != 7 {
        bail!(
            "Expected 7 characters (such as #RRGGBB), got {}",
            hex_str.chars().count()
        );
    }

    let Some('#') = hex_str.chars().next() else {
        bail!("Expected string to start with '#'");
    };

    let hex_str = &hex_str[1..];
    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("Expected only hex digits after '#'");
    }

    Ok(true)
}
