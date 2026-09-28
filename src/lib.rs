/*!
A library to search for colors from a [curated collection of unique color names](https://github.com/meodai/color-names/blob/main/src/colornames.csv).

```toml
[dependencies]
chromiant = "0.1.0"
```

## Usage

Colors can be searched by exact matching name, by hex value, by HSL, by RGB, and by name (color names that contain the given pattern).

### Example

```no_run
# use anyhow::Result;
use chromiant::{by_exact_name, by_hex, by_hsl, by_name, by_rgb};

fn main() -> Result<()> {
    let color1 = by_exact_name("Peach and Quiet")?;
    assert_eq!("#ffccb6".to_string(), color1.unwrap().hex);

    let color2 = by_hex("#225577")?;
    assert_eq!("3AM in Shibuya", color2.unwrap().name);

    let color3 = by_rgb(1, 1, 1)?;
    assert_eq!("Binary Black", color3.unwrap().name);

    let color3 = by_rgb(148, 135, 126)?;
    assert_eq!("Abandoned Mansion", color3.unwrap().name);

    // HSL values need precision.
    // This will not have the result as `by_hsl(0.07, 0.09, 0.54)`:
    let color4 = by_hsl(0.068, 0.093, 0.537)?;
    assert_eq!("Abandoned Mansion", color4.unwrap().name);

    let colors = by_name("Iceland")?;
    println!("{:#?}", colors);

    Ok(())
}
```

*/

use anyhow::Result;
use serde::Deserialize;

mod api_resp;
mod helpers;
mod internals;
mod search;

use crate::helpers::get_local_file_path;
use crate::internals::{by_exact_name_in, by_hex_in, by_hsl_in, by_name_in, by_rgb_in};

/// A color in the collection.
#[derive(Deserialize, Default, Debug, PartialEq)]
pub struct Color {
    /// Name of the color.
    pub name: String,
    /// Hex value of the color.
    pub hex: String,
    /// Marker to indicate whether the color's name is considered "good" or well-crafted.
    ///
    /// The "good names" are marked with `'x'`.
    #[serde(rename = "good name")]
    pub good_name: Option<char>,
}

/// Find a color by the matching name.
///
/// Searches the local data file for the exact matching color name.
///
/// The color names data file is created and written if the local data directory
/// doesn't exist or is not recent (if the remote source file is recently modified).
///
/// If the file exists and is recent, it uses the existing file.
///
/// # Example
///
/// ```
/// use chromiant::by_exact_name;
///
/// let color = by_exact_name("Goldfish").unwrap();
/// assert_eq!("#f2ad62".to_string(), color.unwrap().hex);
/// ```
pub fn by_exact_name(s: &str) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_exact_name_in(s, file_path.as_path())
}

/// Find a color by a hex value.
///
/// Searches the local data file for the given hex value.
///
/// The color names data file is created and written if the local data directory
/// doesn't exist or is not recent (if the remote source file is recently modified).
///
/// If the file exists and is recent, it uses the existing file.
///
/// # Example
///
/// ```
/// use chromiant::by_hex;
///
/// let color = by_hex("#46473e").unwrap();
/// assert_eq!("Heavy Metal".to_string(), color.unwrap().name);
/// ```
pub fn by_hex(s: &str) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_hex_in(s, file_path.as_path())
}

/// Find a color by an RGB value.
///
/// Searches the local data file for the given RGB value.
///
/// The RGB color is converted to hex string, which is then searched in the data file.
/// The color names data file is created and written if the local data directory
/// doesn't exist or is not recent (if the remote source file is recently modified).
///
/// If the file exists and is recent, it uses the existing file.
///
/// # Example
///
/// ```
/// use chromiant::by_rgb;
///
/// let color = by_rgb(1, 2, 3).unwrap();
/// assert_eq!("Black Hole".to_string(), color.unwrap().name);
/// ```
pub fn by_rgb(r: u8, g: u8, b: u8) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_rgb_in(r, g, b, file_path.as_path())
}

/// Find colors that contain the pattern in their name.
///
/// Searches the local data file for the given name.
///
/// The color names data file is created and written if the local data directory
/// doesn't exist or is not recent (if the remote source file is recently modified).
///
/// If the file exists and is recent, it uses the existing file.
///
/// # Example
///
/// ```
/// use chromiant::{Color, by_name};
///
/// let colors = by_name("Fox").unwrap();
///
/// let arctic_fox = Color {
///     name: "Arctic Fox".to_string(),
///     hex: "#e7e7e2".to_string(),
///     good_name: None,
/// };
/// let foxglove = Color {
///     name: "Foxglove".to_string(),
///     hex: "#b57c8c".to_string(),
///     good_name: None,
/// };
///
/// let colors = colors.unwrap();
///
/// assert!(colors.contains(&arctic_fox) && colors.contains(&foxglove));
///
/// ```
pub fn by_name(pattern: &str) -> Result<Option<Vec<Color>>> {
    let file_path = get_local_file_path()?;

    by_name_in(pattern, file_path.as_path())
}

/// Find a color by an HSL value.
///
/// Searches the local data file for the given HSL value.
///
/// The HSL value is first converted to RGB, which is converted to hex string.
///
/// The value should be precise, for example:
/// `by_hsl(0.07, 0.09, 0.54)` won't have the same result as `by_hsl(0.068, 0.093, 0.537)`.
///
/// The color names data file is created and written if the local data directory
/// doesn't exist or is not recent (if the remote source file is recently modified).
///
/// If the file exists and is recent, it uses the existing file.
///
/// # Example
///
/// ```
/// use chromiant::by_hsl;
///
/// let color = by_hsl(0.0, 1.0, 0.666).unwrap();
/// assert_eq!("Fluorescent Red".to_string(), color.unwrap().name);
/// ```
pub fn by_hsl(h: f64, s: f64, l: f64) -> Result<Option<Color>> {
    let file_path = get_local_file_path()?;

    by_hsl_in(h, s, l, file_path.as_path())
}
