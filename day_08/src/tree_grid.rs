use std::collections::HashSet;

use anyhow::{anyhow, Context, Result};

/// One call-and-response within a terminal shell session.
#[derive(Debug)]
pub(crate) struct TreeGrid {
    /// How many rows of trees there are.
    pub(crate) column_count: usize,
    /// Tallest tree in each column.
    pub(crate) column_maxima: Vec<u32>,
    /// How many columns of trees there are.
    pub(crate) row_count: usize,
    /// Tallest tree in each row.
    pub(crate) row_maxima: Vec<u32>,
    /// 2D grid of tree heights organized in a rectangle.
    pub(crate) tree_height_rows: Vec<Vec<u32>>,
}

impl TreeGrid {
    /// Interprets `encoded_tree_grid` as a [TreeGrid], returning an [Err] if
    /// that is impossible.
    pub(crate) fn parse(encoded_tree_grid: &str) -> Result<TreeGrid> {
        let tree_height_rows = encoded_tree_grid
            .lines()
            .map(|row| {
                row.trim()
                    .chars()
                    .map(|raw_tree_height| {
                        raw_tree_height.to_digit(10).with_context(|| {
                            format!("\"{}\" is not a legal tree height", raw_tree_height)
                        })
                    })
                    .collect::<Result<Vec<u32>>>()
                    .with_context(|| format!("Failed to parse tree height row \"{}\"", row))
            })
            .collect::<Result<Vec<Vec<u32>>>>()
            .context("Failed to parse encoded tree grid")?;

        let row_count = tree_height_rows.len();
        if row_count < 3 {
            return Err(anyhow!(
                "\"{}\" is not a valid tree grid: not tall enough",
                encoded_tree_grid
            ));
        }

        let column_count = tree_height_rows[0].len();
        if column_count < 3 {
            return Err(anyhow!(
                "\"{}\" is not a valid tree grid: not wide enough",
                encoded_tree_grid
            ));
        }

        if tree_height_rows
            .iter()
            .any(|tree_height_row| tree_height_row.len() != column_count)
        {
            return Err(anyhow!(
                "\"{}\" is not a valid tree grid: width is not consistent",
                encoded_tree_grid
            ));
        }

        let column_maxima = (0..column_count)
            .map(|column_index| {
                tree_height_rows
                    .iter()
                    .map(|tree_height_row| tree_height_row[column_index])
                    .max()
                    .unwrap()
            })
            .collect::<Vec<u32>>();

        let row_maxima = tree_height_rows
            .iter()
            .map(|tree_height_row| tree_height_row.iter().max().unwrap().to_owned())
            .collect::<Vec<u32>>();

        Ok(TreeGrid {
            column_count: column_count,
            column_maxima: column_maxima,
            row_count: row_count,
            row_maxima: row_maxima,
            tree_height_rows: tree_height_rows,
        })
    }

    /// Calculates and returns the scenic scores for each tree co-ordinate in
    /// `(score, x, y)` format.
    pub(crate) fn scenic_scores(&self) -> Vec<ScenicScoreEvaluation> {
        let mut scenic_scores: Vec<ScenicScoreEvaluation> =
            Vec::with_capacity(self.column_count * self.row_count);

        for column_index in 0..self.column_count {
            for row_index in 0..self.row_count {
                let tree_height = self.tree_height_rows[row_index][column_index];

                let mut down_tree_count: usize = 0;
                for other_row_index in row_index + 1..self.row_count {
                    down_tree_count = down_tree_count + 1;

                    if tree_height <= self.tree_height_rows[other_row_index][column_index] {
                        break;
                    }
                }

                let mut left_tree_count: usize = 0;
                for other_column_index in (0..column_index).rev() {
                    left_tree_count = left_tree_count + 1;

                    if tree_height <= self.tree_height_rows[row_index][other_column_index] {
                        break;
                    }
                }

                let mut right_tree_count: usize = 0;
                for other_column_index in column_index + 1..self.column_count {
                    right_tree_count = right_tree_count + 1;

                    if tree_height <= self.tree_height_rows[row_index][other_column_index] {
                        break;
                    }
                }

                let mut up_tree_count: usize = 0;
                for other_row_index in (0..row_index).rev() {
                    up_tree_count = up_tree_count + 1;

                    if tree_height <= self.tree_height_rows[other_row_index][column_index] {
                        break;
                    }
                }

                let scenic_score =
                    down_tree_count * left_tree_count * right_tree_count * up_tree_count;

                scenic_scores.push(ScenicScoreEvaluation {
                    coordinate: (column_index, row_index),
                    scenic_score: scenic_score,
                    viewing_distances: (
                        up_tree_count,
                        right_tree_count,
                        down_tree_count,
                        left_tree_count,
                    ),
                });
            }
        }

        scenic_scores
    }

    /// Returns the `(x, y)` co-ordinates of trees that are visible from the outside.
    pub(crate) fn visible_trees(&self) -> HashSet<(usize, usize)> {
        let mut visible_trees: HashSet<(usize, usize)> =
            HashSet::with_capacity((self.column_count - 2) * (self.row_count - 2));

        for column_index in 0..self.column_count {
            let tallest = self.column_maxima[column_index];
            let mut tallest_so_far: u32 = 0;

            for row_index in 0..self.row_count {
                let tree_height = self.tree_height_rows[row_index][column_index];

                if row_index == 0 || tallest_so_far < tree_height {
                    visible_trees.insert((column_index, row_index));

                    tallest_so_far = tree_height;
                }

                if tree_height == tallest {
                    break;
                }
            }
            for row_index in (0..self.row_count).rev() {
                let tree_height = self.tree_height_rows[row_index][column_index];

                if row_index == self.row_count - 1 || tallest_so_far < tree_height {
                    visible_trees.insert((column_index, row_index));

                    tallest_so_far = tree_height;
                }

                if tree_height == tallest {
                    break;
                }
            }
        }

        for row_index in 0..self.row_count {
            let tallest = self.row_maxima[row_index];
            let mut tallest_so_far: u32 = 0;

            for column_index in 0..self.column_count {
                let tree_height = self.tree_height_rows[row_index][column_index];

                if column_index == 0 || tallest_so_far < tree_height {
                    visible_trees.insert((column_index, row_index));

                    tallest_so_far = tree_height;
                }

                if tree_height == tallest {
                    break;
                }
            }
            for column_index in (0..self.column_count).rev() {
                let tree_height = self.tree_height_rows[row_index][column_index];

                if column_index == self.row_count - 1 || tallest_so_far < tree_height {
                    visible_trees.insert((column_index, row_index));

                    tallest_so_far = tree_height;
                }

                if tree_height == tallest {
                    break;
                }
            }
        }

        visible_trees
    }
}

/// One call-and-response within a terminal shell session.
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct ScenicScoreEvaluation {
    /// Position from which scenic score was evaluated.
    pub(crate) coordinate: (usize, usize),
    /// Calculated scneic score.
    pub(crate) scenic_score: usize,
    /// Number of visible trees in each direction (up, right, bottom, left).
    pub(crate) viewing_distances: (usize, usize, usize, usize),
}
