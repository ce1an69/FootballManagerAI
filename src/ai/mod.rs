// AI module - Data generation and match simulation

mod generator;
pub mod match_sim;
pub mod tactical;
pub mod substitution;
pub mod progression;
pub mod transfer_decision;
pub mod events;

// Re-exports
pub use generator::{generate_player, generate_team, generate_league, generate_name};
pub use match_sim::MatchSimulator;
pub use tactical::{TacticalStyle, infer_tactical_style, calculate_tactical_modifier};
pub use substitution::{
    SubstitutionAdvisor, SubstitutionSuggestion, SubstitutionReason, SuggestionUrgency
};
pub use progression::{update_players_after_match, update_players_during_break, age_players, PlayerUpdate, PlayerUpdateType};
pub use transfer_decision::{
    evaluate_weaknesses, decide_ai_transfer, process_ai_transfers,
    TeamWeakness, WeaknessSeverity, TransferTarget, TransferPriority,
    AITransferAction, AITransferActionType, TransferDecisionError
};
pub use events::{
    RandomEvent, InjuryType, MediaImpact,
    generate_injury_event, generate_transfer_offer, generate_media_story, generate_random_event
};
