Usage:

```rust
use anyhow::Result;
use chromiant::{by_exact_name, by_hex, by_name, by_rgb};

fn main() -> Result<()> {
    let color1 = by_exact_name("Peach and Quiet")?;
    assert_eq!("#ffccb6".to_string(), color1.unwrap().hex);

    let color2 = by_hex("#225577")?;
    assert_eq!("3AM in Shibuya", color2.unwrap().name);

    let color3 = by_rgb(1, 1, 1)?;
    assert_eq!("Binary Black", color3.unwrap().name);

    let colors = by_name("Iceland")?;
    println!("{:#?}", colors);

    Ok(())
}
```