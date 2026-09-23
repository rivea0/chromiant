use anyhow::Result;
use std::env::var_os;
use std::path::Path;

fn main() -> Result<()> {
    let data = include_str!("./data.csv");
    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("data.csv");
    std::fs::write(&dest_path, data)?;

    Ok(())
}
