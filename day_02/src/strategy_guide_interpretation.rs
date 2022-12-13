/// Enumerates every way that "X", "Y", "Z" could be interpreted in the strategy
/// guide.
#[derive(Clone, Copy, Debug)]
pub(crate) enum StrategyGuideInterpretation {
    /// "X", "Y", "Z" map to hand shapes.
    HandShape,
    /// "X", "Y", "Z" map to round outcomes.
    RoundOutcome,
}
