Usage:

```rust
use anyhow::Result;
use chromiant::{by_exact_name, by_hex, by_name};

fn main() -> Result<()> {
    let color1 = by_exact_name("Peach and Quiet")?;
    let color2 = by_hex("#225577")?;
    let colors = by_name("Iceland")?;

    println!("{:?}", color1);
    println!("{:?}", color2);
    println!("{:#?}", colors);

    Ok(())
}
```