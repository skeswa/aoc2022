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
mod strategy_guide_interpretation;

use anyhow::Result;
use scorable::Scorable;
use strategy_guide::StrategyGuide;
use strategy_guide_interpretation::StrategyGuideInterpretation;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_strategy_guide = advent::data(&config).await?;
    let strategy_guide_interpretation = match config.part {
        1 => StrategyGuideInterpretation::HandShape,
        2 => StrategyGuideInterpretation::RoundOutcome,
        _ => todo!(),
    };

    let strategy_guide =
        StrategyGuide::parse(&encoded_strategy_guide, strategy_guide_interpretation)?;

    println!("Total score of strategy guide: {}", strategy_guide.score());

    Ok(())
}
