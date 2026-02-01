// AI module - Data generation and match simulation

mod generator;
mod match_sim;

// Re-exports
pub use generator::{generate_player, generate_team, generate_league, generate_name};
pub use match_sim::MatchSimulator;
