use std::cmp::max;

/// Extension trait for `(i32, i32)`.
pub(crate) trait I32TupleExt {
    /// Returns how big the gap between this tuple and the `other` tuple is.
    fn gap_with(&self, other: &Self) -> u32;
}

impl I32TupleExt for (i32, i32) {
    fn gap_with(&self, other: &Self) -> u32 {
        let (x, y) = self;
        let (other_x, other_y) = other;

        let gap_x = (other_x - x).abs().try_into().unwrap();
        let gap_y = (other_y - y).abs().try_into().unwrap();

        max(gap_x, gap_y)
    }
}
