use std::collections::HashSet;

use crate::{
    i32_tuple_ext::I32TupleExt,
    rope_move::{RopeMove, RopeMoveDirection},
};

/// Enumerates every variety of rope move.
#[derive(Debug)]
pub(crate) struct Rope {
    /// Position of the head of the rope.
    pub(crate) head_position: (i32, i32),
    /// Position of the tail of the rope.
    pub(crate) tail_position: (i32, i32),
    pub(crate) tail_positions: HashSet<(i32, i32)>,
}

impl Rope {
    /// Creates and returns a new [Rope].
    pub(crate) fn new() -> Rope {
        let starting_position = (0, 0);
        let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

        tail_positions.insert(starting_position);

        Rope {
            head_position: starting_position,
            tail_position: starting_position,
            tail_positions: tail_positions,
        }
    }

    /// Applies the provided [RopeMove] to this [Rope]'s `head_position`.
    pub(crate) fn move_head(&mut self, rope_move: &RopeMove) {
        let RopeMove {
            direction,
            distance,
        } = rope_move;

        let head_position = &mut self.head_position;
        let tail_position = &mut self.tail_position;
        let tail_positions = &mut self.tail_positions;

        for _ in 0..*distance {
            match direction {
                RopeMoveDirection::Down => head_position.1 = head_position.1 - 1,
                RopeMoveDirection::Left => head_position.0 = head_position.0 - 1,
                RopeMoveDirection::Right => head_position.0 = head_position.0 + 1,
                RopeMoveDirection::Up => head_position.1 = head_position.1 + 1,
            };

            let gap = head_position.gap_with(&tail_position);
            if gap > 1 {
                match rope_move.direction {
                    RopeMoveDirection::Down => {
                        tail_position.0 = head_position.0;
                        tail_position.1 = head_position.1 + 1;
                    }
                    RopeMoveDirection::Left => {
                        tail_position.0 = head_position.0 + 1;
                        tail_position.1 = head_position.1;
                    }
                    RopeMoveDirection::Right => {
                        tail_position.0 = head_position.0 - 1;
                        tail_position.1 = head_position.1;
                    }
                    RopeMoveDirection::Up => {
                        tail_position.0 = head_position.0;
                        tail_position.1 = head_position.1 - 1;
                    }
                };
            }

            tail_positions.insert(*tail_position);
        }
    }
}
