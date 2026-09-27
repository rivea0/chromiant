use anyhow::Result;
use colorsys::{Hsl, Rgb};
use serde::Deserialize;

mod api_resp;
mod helpers;
mod search;

use crate::helpers::{validate_hex_string, write_data_to_file};
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
    validate_hex_string(s)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn by_exact_name_finds_existing_colors() {
        let color = by_exact_name("Peach and Quiet").unwrap();
        let color2 = by_exact_name("Black").unwrap();

        assert_eq!("#ffccb6".to_string(), color.unwrap().hex);
        assert_eq!(
            Some(Color {
                name: "Black".to_string(),
                hex: "#000000".to_string(),
                good_name: Some('x')
            }),
            color2
        );
    }

    #[test]
    fn by_exact_name_fails_on_non_existent_name() {
        let c = by_exact_name("bla bla");
        assert_eq!(None, c.unwrap());
    }

    #[test]
    fn by_hex_finds_existing_colors() {
        let color = by_hex("#d5762b").unwrap();
        let color2 = by_hex("#ffefc1").unwrap();

        assert_eq!("Bitter Orange", color.unwrap().name);

        assert_eq!(
            Some(Color {
                name: "Egg White".to_string(),
                hex: "#ffefc1".to_string(),
                good_name: None
            }),
            color2
        );
    }

    #[test]
    fn by_hex_bails_when_given_incorrect_number_of_chars() -> Result<()> {
        let result = by_hex("535535").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected 7 characters (such as #RRGGBB), got 6")
        );

        Ok(())
    }

    #[test]
    fn by_hex_bails_when_input_starts_with_incorrect_char() -> Result<()> {
        let result = by_hex("+000000").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected string to start with '#'")
        );

        Ok(())
    }

    #[test]
    fn by_hex_bails_when_given_input_includes_non_hex_digits() -> Result<()> {
        let result = by_hex("#0000gg").unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected only hex digits after '#'")
        );

        Ok(())
    }
}
