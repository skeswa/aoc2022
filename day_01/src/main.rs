extern crate anyhow;
extern crate tokio;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello World!");

    Ok(())
}
