use crate::{
    round::Round, scorable::Scorable, strategy_guide_interpretation::StrategyGuideInterpretation,
};
use anyhow::{Context, Result};

/// Series of rock-paper-scissors rounds that dictate how each player should
/// play in each phase of the game.
#[derive(Debug)]
pub(crate) struct StrategyGuide(Vec<Round>);

impl StrategyGuide {
    /// Returns a [StrategyGuide] corresponding to the specified
    /// `encoded_strategy_guide` string.
    ///
    /// * `interpretation` configures how this method should interpret
    ///   `encoded_strategy_guide`
    pub(crate) fn parse(
        encoded_strategy_guide: &str,
        interpretation: StrategyGuideInterpretation,
    ) -> Result<StrategyGuide> {
        let rounds = encoded_strategy_guide
            .lines()
            .map(|encoded_strategy_guide_line| {
                Round::parse(encoded_strategy_guide_line, interpretation)
            })
            .collect::<Result<Vec<Round>>>()
            .context("Failed to parse encoded strategy guide")?;

        Ok(StrategyGuide(rounds))
    }
}

impl Scorable for StrategyGuide {
    fn score(&self) -> u32 {
        self.0.iter().fold(0, |acc, round| acc + round.score())
    }
}
