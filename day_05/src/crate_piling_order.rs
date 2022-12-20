/// Enumerates every stacking direction that can be used when rearranging
/// crates.
pub(crate) enum CratePilingOrder {
    /// Flips moved crates upside down for the destination stack.
    Flipped,
    /// Keeps crates in the same order in the destination stack as in the
    /// original.
    InOrder,
}
