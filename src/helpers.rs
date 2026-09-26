use anyhow::Result;
use dirs::data_local_dir;
use jiff::{SignedDuration, Timestamp};
use std::path::PathBuf;

pub(crate) fn is_recent(timestamp: Timestamp) -> bool {
    let age = timestamp.duration_until(Timestamp::now());
    let day = SignedDuration::from_hours(24);

    age >= SignedDuration::ZERO && age < day
}

pub(crate) fn get_data_path() -> Result<PathBuf> {
    let dir = get_data_dir();

    Ok(dir.join("colornames.csv"))
}

pub(crate) fn get_data_dir() -> PathBuf {
    let mut dir = match data_local_dir() {
        Some(dir) => dir,
        None => std::env::current_dir()
            .expect("Failed to get current dir")
            .to_path_buf(),
    };

    dir.push("chromiant");

    dir
}

