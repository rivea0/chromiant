use anyhow::Result;
use std::env::var_os;
use std::path::Path;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<()> {
    let resp = reqwest::get(
        "https://raw.githubusercontent.com/meodai/color-names/refs/heads/main/src/colornames.csv",
    )
    .await?
    .text()
    .await?;

    let out_dir = var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("colornames.csv");
    let mut f = tokio::fs::File::create(dest_path).await?;
    f.write_all(resp.as_bytes()).await?;
    Ok(())
}
