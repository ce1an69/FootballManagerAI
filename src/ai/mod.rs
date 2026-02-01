// AI module - Data generation and match simulation

mod generator;
mod match_sim;
pub mod progression;
pub mod transfer_decision;

// Re-exports
pub use generator::{generate_player, generate_team, generate_league, generate_name};
pub use match_sim::MatchSimulator;
pub use progression::{update_players_after_match, update_players_during_break, age_players, PlayerUpdate, PlayerUpdateType};
pub use transfer_decision::{
    evaluate_weaknesses, decide_ai_transfer, process_ai_transfers,
    TeamWeakness, WeaknessSeverity, TransferTarget, TransferPriority,
    AITransferAction, AITransferActionType, TransferDecisionError
};
