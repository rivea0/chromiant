# chromiant

A library to search for colors from a [curated collection of unique color names](https://github.com/meodai/color-names/blob/main/src/colornames.csv).

```toml
[dependencies]
chromiant = "0.1.0"
```

## Usage

Colors can be searched by exact matching name, by hex value, by HSL, by RGB, and by name (color names that contain the given pattern).

### Example

```rust
use anyhow::Result;
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