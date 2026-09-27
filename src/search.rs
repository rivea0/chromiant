use anyhow::{Context, Result};
use csv::Reader;

use std::path::Path;

use crate::Color;

#[derive(Debug)]
pub(crate) enum SearchType {
    ByExactName,
    ByHex,
}

#[derive(Debug)]
pub(crate) struct SearchQuery {
    pub(crate) search_type: SearchType,
    pub(crate) query: String,
}

pub(crate) fn search_multiple(s: &str, path: &Path) -> Result<Option<Vec<Color>>> {
    let mut rdr = Reader::from_path(path).context("File not found")?;
    let results = rdr.deserialize();
    let colors = results
        .filter_map(|result| result.ok())
        .filter(|color: &Color| color.name.contains(s))
        .collect::<Vec<Color>>();

    Ok((!colors.is_empty()).then_some(colors))
}

// Names and hex values are unique
pub(crate) fn search(query: &SearchQuery, path: &Path) -> Result<Option<Color>> {
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
