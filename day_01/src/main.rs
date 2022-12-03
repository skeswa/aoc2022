extern crate advent;
extern crate anyhow;
extern crate lazy_static;
extern crate regex;
extern crate tokio;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<()> {
    let config = advent::begin();

    let elven_inventory_data = advent::data(&config).await?;

    let elven_inventory_lines = elven_inventory_data.lines().collect::<Vec<&str>>();

    let elven_inventory_line_clusters = elven_inventory_lines
        .split(|elven_inventory_line| {
            elven_inventory_line.is_empty() || !EMPTY_LINE_PATTERN.is_match(elven_inventory_line)
        })
        .map(|s| s.to_owned())
        .collect::<Vec<Vec<&str>>>();

    let elven_inventory_calorie_counts =
        elven_inventory_line_clusters
            .iter()
            .map(|elven_inventory_line_cluster| {
                elven_inventory_line_cluster
                    .iter()
                    .map(|elven_inventory_line| {
                        elven_inventory_line.parse::<i32>().with_context(|| {
                            format!("Failed to parse inventory line: {}", elven_inventory_line)
                        })
                    })
                    .collect::<Result<Vec<i32>>>()
            })
            .collect::<Result<Vec<Vec<i32>>>>()?;

    let elven_inventory_calorie_totals = elven_inventory_calorie_counts.iter().map(|elven_inventory_calorie_counts| elven_inventory_calorie_counts.iter().fold(0, |total_calories, calorie_count| total_calories + calorie_count)).collect::<Vec<i32>>();

    let max_elven_inventory_calorie_total = elven_inventory_calorie_totals.iter().max().unwrap_or(&0);

    println!("Maximum elven calorie total: {}", max_elven_inventory_calorie_total);

    Ok(())
}

lazy_static! {
  /// Regular expression designed to match strings containing purely whitespace.
  static ref EMPTY_LINE_PATTERN: Regex =
      Regex::new(r"\s*").unwrap();
}
