extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let data = advent::data(&config).await?;

    println!("Hello advent! {}", data);

    Ok(())
}
