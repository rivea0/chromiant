use anyhow::Result;
use colorsys::{Hsl, Rgb};

use std::path::Path;

use crate::{
    Color,
    helpers::{validate_hex_string, write_data_to_file},
    search::{SearchQuery, SearchType, search, search_multiple},
};

pub(crate) fn by_exact_name_in(s: &str, file_path: &Path) -> Result<Option<Color>> {
    let search_query = SearchQuery {
        search_type: SearchType::ByExactName,
        query: s.to_string(),
    };

    write_data_to_file(file_path)?;

    let result = search(&search_query, file_path)?;

    Ok(result)
}

pub(crate) fn by_hex_in(s: &str, file_path: &Path) -> Result<Option<Color>> {
    validate_hex_string(s)?;

    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: s.to_string(),
    };

    write_data_to_file(file_path)?;

    let result = search(&search_query, file_path)?;

    Ok(result)
}

pub(crate) fn by_rgb_in(r: u8, g: u8, b: u8, file_path: &Path) -> Result<Option<Color>> {
    let rgb = Rgb::from([r, g, b]);
    let hex = rgb.to_hex_string();

    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: hex,
    };

    write_data_to_file(file_path)?;

    let result = search(&search_query, file_path)?;

    Ok(result)
}

pub(crate) fn by_name_in(pattern: &str, file_path: &Path) -> Result<Option<Vec<Color>>> {
    write_data_to_file(file_path)?;

    let results = search_multiple(pattern, file_path)?;

    Ok(results)
}

pub(crate) fn by_hsl_in(h: f64, s: f64, l: f64, file_path: &Path) -> Result<Option<Color>> {
    let hsl = Hsl::from(&(h * 360.0, s * 100.0, l * 100.0));
    let rgb = Rgb::from(&hsl);
    let hex = rgb.to_hex_string();

    let search_query = SearchQuery {
        search_type: SearchType::ByHex,
        query: hex,
    };

    write_data_to_file(file_path)?;

    let result = search(&search_query, file_path)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    use crate::helpers::{MockResourceFetcher, update_local_data_file};

    #[test]
    fn by_exact_name_in_finds_existing_colors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Peach and Quiet", "#ffccb6", "x"])
            .unwrap();
        wtr.write_record(&["Black", "#000000", "x"]).unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nPeach and Quiet,#ffccb6,x\nBlack,#000000,x";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let color = by_exact_name_in("Peach and Quiet", &p).unwrap();
        let color2 = by_exact_name_in("Black", &p).unwrap();

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
    fn by_exact_name_in_fails_on_non_existent_name() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Black", "#000000", "x"]).unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBlack,#000000,x";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let c = by_exact_name_in("bla bla", &p);
        assert_eq!(None, c.unwrap());
    }

    #[test]
    fn by_hex_in_finds_existing_colors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Bitter Orange", "#d5762b", ""]).unwrap();
        wtr.write_record(&["Egg White", "#ffefc1", ""]).unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBitter Orange,#d5762b,\nEgg White,#ffefc1,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let color = by_hex_in("#d5762b", &p).unwrap();
        let color2 = by_hex_in("#ffefc1", &p).unwrap();

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
    fn by_hex_in_bails_when_given_incorrect_number_of_chars() -> Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut fetcher = MockResourceFetcher::new();
        fetcher.expect_fetch().times(0);

        let result = by_hex_in("535535", &p).unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected 7 characters (such as #RRGGBB), got 6")
        );

        Ok(())
    }

    #[test]
    fn by_hex_in_bails_when_input_starts_with_incorrect_char() -> Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut fetcher = MockResourceFetcher::new();
        fetcher.expect_fetch().times(0);

        let result = by_hex_in("+000000", &p).unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected string to start with '#'")
        );

        Ok(())
    }

    #[test]
    fn by_hex_in_bails_when_given_input_includes_non_hex_digits() -> Result<()> {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut fetcher = MockResourceFetcher::new();
        fetcher.expect_fetch().times(0);

        let result = by_hex_in("#0000gg", &p).unwrap_err();

        assert!(
            result
                .to_string()
                .contains("Expected only hex digits after '#'")
        );

        Ok(())
    }

    #[test]
    fn by_rgb_in_finds_existing_colors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Binary Black", "#010101", ""]).unwrap();
        wtr.write_record(&["Abandoned Mansion", "#94877e", ""])
            .unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBinary Black,#010101,\nAbandoned Mansion,#94877e,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let color = by_rgb_in(1, 1, 1, &p).unwrap();
        assert_eq!("Binary Black", color.unwrap().name);

        let color2 = by_rgb_in(148, 135, 126, &p).unwrap();
        assert_eq!(
            Some(Color {
                name: "Abandoned Mansion".to_string(),
                hex: "#94877e".to_string(),
                good_name: None
            }),
            color2
        );
    }

    #[test]
    fn by_rgb_in_fails_on_non_existent_value() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Binary Black", "#010101", ""]).unwrap();
        wtr.write_record(&["Abandoned Mansion", "#94877e", ""])
            .unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBinary Black,#010101,\nAbandoned Mansion,#94877e,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let c = by_rgb_in(0, 0, 1, &p);
        assert_eq!(None, c.unwrap());
    }

    #[test]
    fn by_hsl_in_finds_existing_colors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Abandoned Mansion", "#94877e", ""])
            .unwrap();
        wtr.write_record(&["Fluorescent Red", "#ff5555", ""])
            .unwrap();
        wtr.flush().unwrap();

        let expected_data =
            "name,hex,good name\nAbandoned Mansion,#94877e,\nFluorescent Red,#ff5555,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let color = by_hsl_in(0.068, 0.093, 0.537, &p).unwrap();
        assert_eq!("Abandoned Mansion", color.unwrap().name);

        let color2 = by_hsl_in(0.0, 1.0, 0.666, &p).unwrap();
        assert_eq!(
            Some(Color {
                name: "Fluorescent Red".to_string(),
                hex: "#ff5555".to_string(),
                good_name: None
            }),
            color2
        );
    }

    #[test]
    fn by_hsl_in_fails_on_incorrect_value() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Binary Black", "#010101", ""]).unwrap();
        wtr.write_record(&["Abandoned Mansion", "#94877e", ""])
            .unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBinary Black,#010101,\nAbandoned Mansion,#94877e,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let c = by_hsl_in(0.07, 0.09, 0.54, &p);
        assert_eq!(None, c.unwrap());
    }

    #[test]
    fn by_name_in_finds_existing_colors() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Fluorescent Red", "#ff5555", ""])
            .unwrap();
        wtr.write_record(&["Gloomy Blue", "#3c416a", ""]).unwrap();
        wtr.write_record(&["Gloomy Purple", "#8756e4", ""]).unwrap();
        wtr.write_record(&["Gloomy Sea", "#4a657a", ""]).unwrap();

        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nFluorescent Red,#ff5555,\nGloomy Blue,#3c416a,\nGloomy Purple,#8756e4,\nGloomy Sea,#4a657a,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let colors = by_name_in("Fluorescent", &p).unwrap();
        assert!(!colors.unwrap().is_empty());

        let colors2 = by_name_in("Gloomy", &p).unwrap();
        assert_eq!(
            Some(vec![
                Color {
                    name: "Gloomy Blue".to_string(),
                    hex: "#3c416a".to_string(),
                    good_name: None,
                },
                Color {
                    name: "Gloomy Purple".to_string(),
                    hex: "#8756e4".to_string(),
                    good_name: None,
                },
                Color {
                    name: "Gloomy Sea".to_string(),
                    hex: "#4a657a".to_string(),
                    good_name: None,
                },
            ],),
            colors2
        );
    }

    #[test]
    fn by_name_fails_on_non_existent_name() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("colornames.csv");

        let mut wtr = csv::Writer::from_path(&p).unwrap();
        wtr.write_record(&["name", "hex", "good name"]).unwrap();
        wtr.write_record(&["Binary Black", "#010101", ""]).unwrap();
        wtr.write_record(&["Abandoned Mansion", "#94877e", ""])
            .unwrap();
        wtr.flush().unwrap();

        let expected_data = "name,hex,good name\nBinary Black,#010101,\nAbandoned Mansion,#94877e,";

        let mut fetcher = MockResourceFetcher::new();
        fetcher
            .expect_fetch()
            .with(mockall::predicate::eq(Duration::from_secs(30)), mockall::predicate::eq("https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv"))
            .times(1)
            .returning(|_, _| {
                Ok(expected_data.to_string())
            });

        update_local_data_file(&p, &fetcher).unwrap();

        let c = by_name_in("abcde", &p);
        assert_eq!(None, c.unwrap());
    }
}
