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

    let mut visible_trees = tree_grid
        .visible_trees()
        .into_iter()
        .collect::<Vec<(usize, usize)>>();

    visible_trees.sort();

    println!("# of visible trees: {}", visible_trees.len());

    Ok(())
}
