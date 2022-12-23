extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod rope_move;

use anyhow::{Context, Result};
use rope_move::RopeMove;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_rope_moves = advent::data(&config).await?;

    let rope_moves = encoded_rope_moves
        .lines()
        .map(|encoded_rope_move| RopeMove::parse(encoded_rope_move))
        .collect::<Result<Vec<RopeMove>>>()
        .context("Failed to read rope moves")?;

    println!("Hello advent! {:#?}", rope_moves);

    Ok(())
}
