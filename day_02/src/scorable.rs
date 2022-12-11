/// Trait describing anything that can contribute to a score.
pub(crate) trait Scorable {
    /// Returns a number signifying this [Scorable]'s score.
    fn score(&self) -> u32;
}
