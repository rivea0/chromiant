use anyhow::Result;
use colorsys::{Hsl, Rgb};
use serde::Deserialize;

mod api_resp;
mod helpers;
mod search;

use crate::helpers::write_data_to_file;
use crate::search::{SearchQuery, SearchType, search, search_multiple};

#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Color {
    pub name: String,
    pub hex: String,
    #[serde(rename = "good name")]
    pub good_name: Option<char>,
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
