use anyhow::{Context, Result};
use csv::Reader;
use serde::Deserialize;

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
    let mut rdr =
        Reader::from_path(concat!(env!("OUT_DIR"), "/colornames.csv")).context("File not found")?;
    let results = rdr.deserialize();
    let colors = results
        .filter_map(|result| result.ok())
        .filter(|color: &Color| color.name.contains(s))
        .collect::<Vec<Color>>();

    Ok((!colors.is_empty()).then_some(colors))
}

// Names and hex values are unique
fn search(query: &SearchQuery) -> Result<Option<Color>> {
    let mut rdr =
        Reader::from_path(concat!(env!("OUT_DIR"), "/colornames.csv")).context("File not found")?;
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

    let result = search(&search_query)?;

    Ok(result)
}

pub fn by_hex(s: &str) -> Result<Option<Color>> {
    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: s.to_string(),
    };

    let result = search(&search_query)?;

    Ok(result)
}

// Get colors that includes the pattern in name
pub fn by_name(pattern: &str) -> Result<Option<Vec<Color>>> {
    let results = search_multiple(pattern)?;

    Ok(results)
}
