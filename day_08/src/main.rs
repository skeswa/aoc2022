extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

mod tree_grid;

use anyhow::{Context, Result};
use tree_grid::TreeGrid;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let encoded_tree_grid = advent::data(&config).await?;

    let tree_grid = TreeGrid::parse(&encoded_tree_grid).context("Failed to parse tree grid")?;

    match config.part {
        1 => {
            let mut visible_trees = tree_grid
                .visible_trees()
                .into_iter()
                .collect::<Vec<(usize, usize)>>();

            visible_trees.sort();

            println!("# of visible trees: {}", visible_trees.len());
        }
        _ => {
            let mut scenic_scores = tree_grid.scenic_scores();

            scenic_scores.sort_by(|a, b| b.scenic_score.cmp(&a.scenic_score));

            println!(
                "Highest possible scenic score: {}",
                scenic_scores[0].scenic_score
            );
        }
    }

    Ok(())
}
