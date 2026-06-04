//! Ternary agent dynamics visualizer — ASCII/text visualizations for terminals, logs, and docs.
//!
//! Provides visualization types for ternary agent populations where each agent
//! can be in one of three states: **Avoid** (░), **Unknown** (▒), or **Choose** (█).

mod population_heatmap;
mod fitness_chart;
mod species_bar;
mod conservation_gauge;
mod cascade_timeline;
mod strategy_grid;
mod summary_card;

pub use population_heatmap::PopulationHeatmap;
pub use fitness_chart::FitnessChart;
pub use species_bar::SpeciesBar;
pub use conservation_gauge::ConservationGauge;
pub use cascade_timeline::{CascadeTimeline, CascadeEvent};
pub use strategy_grid::StrategyGrid;
pub use summary_card::SummaryCard;

/// The three ternary agent states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryState {
    Avoid,
    Unknown,
    Choose,
}

impl TernaryState {
    /// Returns the block character for this state.
    pub fn symbol(self) -> char {
        match self {
            TernaryState::Avoid => '░',
            TernaryState::Unknown => '▒',
            TernaryState::Choose => '█',
        }
    }
}
