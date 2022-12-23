extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod rope;
mod rope_move;

use anyhow::{Context, Result};
use rope::Rope;
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

    let knot_count = match config.part {
        1 => 2,
        _ => 10,
    };

    let mut rope = Rope::new(knot_count).context("Failed to create a rope")?;
    for rope_move in rope_moves {
        rope.move_head(&rope_move);
    }

    println!("# of tail positions: {:?}", rope.tail_positions.len());

    Ok(())
}
