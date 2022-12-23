use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::rope_move::{RopeMove, RopeMoveDirection};

/// Enumerates every variety of rope move.
#[derive(Debug)]
pub(crate) struct Rope {
    /// Positions of each know in the rope.
    pub(crate) knot_positions: Vec<(i32, i32)>,
    /// Every position the last knot has ever had.
    pub(crate) tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    /// Creates and returns a new [Rope].
    ///
    /// * `knot_count` is the number knots the resulting [Rope] will have.
    pub(crate) fn new(knot_count: usize) -> Result<Rope> {
        if knot_count < 2 {
            return Err(anyhow!("Knot count must be at least 2"));
        }

        let starting_position = (0, 0);

        let knot_positions: Vec<(i32, i32)> = vec![starting_position; knot_count];
        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

        tail_positions.insert(starting_position);

        Ok(Rope {
            knot_positions: knot_positions,
            tail_positions: tail_positions,
        })
    }

    /// Applies the provided [RopeMove] to this [Rope]'s head knot, moving the
    /// other knots thereafter if necessary.
    pub(crate) fn move_head(&mut self, rope_move: &RopeMove) {
        let RopeMove {
            direction,
            distance,
        } = rope_move;

        let knot_count = self.knot_positions.len();
        let tail_positions = &mut self.tail_positions;

        for _ in 0..*distance {
            {
                let head_knot_position = &mut self.knot_positions[0];

                match direction {
                    RopeMoveDirection::Down => head_knot_position.1 = head_knot_position.1 - 1,
                    RopeMoveDirection::Left => head_knot_position.0 = head_knot_position.0 - 1,
                    RopeMoveDirection::Right => head_knot_position.0 = head_knot_position.0 + 1,
                    RopeMoveDirection::Up => head_knot_position.1 = head_knot_position.1 + 1,
                }
            };

            for i in 1..knot_count {
                let (previous_knot_x, previous_knot_y) = self.knot_positions[i - 1];
                let knot_position = &mut self.knot_positions[i];

                let delta_x = previous_knot_x - knot_position.0;
                let delta_y = previous_knot_y - knot_position.1;
                if delta_x.abs() < 2 && delta_y.abs() < 2 {
                    continue;
                }

                if previous_knot_x - knot_position.0 > 0 {
                    knot_position.0 = knot_position.0 + 1;
                } else if delta_x < 0 {
                    knot_position.0 = knot_position.0 - 1;
                }

                if previous_knot_y - knot_position.1 > 0 {
                    knot_position.1 = knot_position.1 + 1;
                } else if delta_y < 0 {
                    knot_position.1 = knot_position.1 - 1;
                }
            }

            tail_positions.insert(self.knot_positions[self.knot_positions.len() - 1]);
        }
    }
}
