extern crate advent;
extern crate anyhow;
extern crate tokio;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    println!("Hey I found something! {:?}", config);

    let data = advent::data(&config).await?;
    println!("here is the data! {:?}", data);

    Ok(())
}
