extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod hand_shape;
mod round;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let strategy_guide = advent::data(&config).await?;

    println!("Hello, world! {}", strategy_guide);

    Ok(())
}
