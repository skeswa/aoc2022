extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod hand_shape;
mod round;
mod round_outcome;
mod scorable;
mod strategy_guide;

use anyhow::Result;
use scorable::Scorable;
use strategy_guide::StrategyGuide;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_strategy_guide = advent::data(&config).await?;

    let strategy_guide = StrategyGuide::parse(&encoded_strategy_guide)?;

    println!("Total score of strategy guide: {}", strategy_guide.score());

    Ok(())
}
