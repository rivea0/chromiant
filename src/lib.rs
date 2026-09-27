use anyhow::Result;
use serde::Deserialize;

mod api_resp;
mod helpers;
mod internals;
mod search;

use crate::helpers::get_local_file_path;
use crate::internals::{by_exact_name_in, by_hex_in, by_hsl_in, by_name_in, by_rgb_in};

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Color {
    pub name: String,
    pub hex: String,
    #[serde(rename = "good name")]
    pub good_name: Option<char>,
}

pub fn by_exact_name(s: &str) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_exact_name_in(s, file_path.as_path())
}

pub fn by_hex(s: &str) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_hex_in(s, file_path.as_path())
}

pub fn by_rgb(r: u8, g: u8, b: u8) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_rgb_in(r, g, b, file_path.as_path())
}

// Get colors that includes the pattern in name
pub fn by_name(pattern: &str) -> Result<Option<Vec<Color>>> {
    let file_path = get_local_file_path()?;

    by_name_in(pattern, file_path.as_path())
}

pub fn by_hsl(h: f64, s: f64, l: f64) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_hsl_in(h, s, l, file_path.as_path())
}
