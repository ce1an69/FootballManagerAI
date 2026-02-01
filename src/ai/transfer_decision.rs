use crate::team::{Player, Position};
use crate::data::{Database, TransferMarketRepository, PlayerRepository, TeamRepository};
use std::collections::HashMap;

/// AI transfer decision error types
#[derive(Debug, thiserror::Error)]
pub enum TransferDecisionError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::data::DatabaseError),

    #[error("Team not found: {0}")]
    TeamNotFound(String),

    #[error("Insufficient budget")]
    InsufficientBudget,
}

/// Weakness identified in a team
#[derive(Debug, Clone)]
pub struct TeamWeakness {
    pub position: Position,
    pub severity: WeaknessSeverity,
    pub current_players: Vec<String>, // Player IDs
    pub suggested_quality: u16,       // Target ability level
}

/// Severity of weakness
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeaknessSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// AI transfer target suggestion
#[derive(Debug, Clone)]
pub struct TransferTarget {
    pub player_id: String,
    pub position: Position,
    pub priority: TransferPriority,
    pub max_price: u32,
    pub reason: String,
}

/// Priority of transfer target
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TransferPriority {
    Urgent,
    High,
    Medium,
    Low,
}

/// Evaluate team weaknesses for AI team
pub fn evaluate_weaknesses(
    team_id: &str,
    database: &Database,
) -> Result<Vec<TeamWeakness>, TransferDecisionError> {
    let player_repo = database.player_repo();
    let players = player_repo.get_by_team(team_id)?;

    let mut weaknesses = Vec::new();

    // Group players by position
    let mut position_groups: HashMap<Position, Vec<&Player>> = HashMap::new();
    for player in &players {
        position_groups
            .entry(player.position.clone())
            .or_insert_with(Vec::new)
            .push(player);
    }

    // Evaluate each position group
    for (position, pos_players) in &position_groups {
        let weakness = assess_position_weakness(position, pos_players);
        if let Some(w) = weakness {
            weaknesses.push(w);
        }
    }

    // Sort by severity (critical first)
    weaknesses.sort_by(|a, b| {
        let severity_order = |s: &WeaknessSeverity| match s {
            WeaknessSeverity::Critical => 0,
            WeaknessSeverity::High => 1,
            WeaknessSeverity::Medium => 2,
            WeaknessSeverity::Low => 3,
        };
        severity_order(&a.severity).cmp(&severity_order(&b.severity))
    });

    Ok(weaknesses)
}

/// Assess weakness at a specific position
fn assess_position_weakness(
    position: &Position,
    players: &[&Player],
) -> Option<TeamWeakness> {
    let player_count = players.len();
    let avg_ability: u16 = if player_count > 0 {
        players.iter().map(|p| p.current_ability).sum::<u16>() / player_count as u16
    } else {
        0
    };

    // Determine required player count and quality by position
    let (required_count, min_quality) = match position {
        Position::GK => (2, 100),     // Need 2 GKs
        Position::CB => (4, 110),     // Need 4 CBs
        Position::LB | Position::RB => (2, 105), // Need 2 fullbacks
        Position::WB => (1, 100),     // Need 1 wingback
        Position::DM => (2, 108),     // Need 2 DMs
        Position::CM => (4, 110),     // Need 4 CMs
        Position::AM => (2, 112),     // Need 2 AMs
        Position::LW | Position::RW => (2, 108), // Need 2 wingers
        Position::ST | Position::CF => (3, 115), // Need 3 strikers
    };

    // Calculate weakness severity
    let (severity, suggested_quality) = if player_count == 0 {
        (WeaknessSeverity::Critical, min_quality + 20)
    } else if player_count < required_count / 2 {
        (WeaknessSeverity::High, min_quality + 10)
    } else if player_count < required_count {
        (WeaknessSeverity::Medium, min_quality)
    } else if avg_ability < min_quality - 20 {
        (WeaknessSeverity::High, min_quality + 5)
    } else if avg_ability < min_quality - 10 {
        (WeaknessSeverity::Medium, min_quality)
    } else if avg_ability < min_quality {
        (WeaknessSeverity::Low, min_quality)
    } else {
        return None; // No weakness
    };

    Some(TeamWeakness {
        position: position.clone(),
        severity,
        current_players: players.iter().map(|p| p.id.clone()).collect(),
        suggested_quality,
    })
}

/// Decide AI transfer targets based on weaknesses
pub fn decide_ai_transfer(
    team_id: &str,
    database: &Database,
) -> Result<Vec<TransferTarget>, TransferDecisionError> {
    // Get team budget
    let team = match database.team_repo().get_by_id(team_id) {
        Ok(t) => t,
        Err(crate::data::DatabaseError::NotFound(_)) => {
            return Err(TransferDecisionError::TeamNotFound(team_id.to_string()))
        }
        Err(e) => return Err(TransferDecisionError::DatabaseError(e)),
    };

    // Evaluate weaknesses
    let weaknesses = evaluate_weaknesses(team_id, database)?;

    if weaknesses.is_empty() {
        return Ok(Vec::new()); // No transfers needed
    }

    // Get transfer market
    let market_repo = database.transfer_market_repo();
    let market_players = market_repo.get_market_players()?;

    let mut targets = Vec::new();

    // For each weakness, find suitable players on market
    for weakness in weaknesses.iter().take(3) { // Limit to top 3 weaknesses
        let budget_per_player = (team.budget / weaknesses.len() as u32).min(20_000_000);

        // Find players on market for this position
        let suitable_players: Vec<_> = market_players.iter()
            .filter(|p| p.position == weakness.position)
            .filter(|p| p.current_ability >= weakness.suggested_quality.saturating_sub(10))
            .filter(|p| p.market_value <= budget_per_player)
            .take(3) // Take top 3 candidates
            .collect();

        for player in suitable_players {
            let priority = match weakness.severity {
                WeaknessSeverity::Critical => TransferPriority::Urgent,
                WeaknessSeverity::High => TransferPriority::High,
                WeaknessSeverity::Medium => TransferPriority::Medium,
                WeaknessSeverity::Low => TransferPriority::Low,
            };

            targets.push(TransferTarget {
                player_id: player.id.clone(),
                position: player.position.clone(),
                priority,
                max_price: budget_per_player,
                reason: format!(
                    "Needed for {} position (current quality: {})",
                    format!("{:?}", weakness.position),
                    weakness.suggested_quality
                ),
            });
        }
    }

    // Sort by priority
    targets.sort_by(|a, b| {
        let priority_order = |p: &TransferPriority| match p {
            TransferPriority::Urgent => 0,
            TransferPriority::High => 1,
            TransferPriority::Medium => 2,
            TransferPriority::Low => 3,
        };
        priority_order(&a.priority).cmp(&priority_order(&b.priority))
    });

    Ok(targets)
}

/// Process AI transfers for all AI teams
pub fn process_ai_transfers(
    database: &Database,
    league_id: &str,
) -> Result<Vec<AITransferAction>, TransferDecisionError> {
    let team_repo = database.team_repo();
    let teams = team_repo.get_all()?;

    // Filter teams in this league (excluding player's team if needed)
    let ai_teams: Vec<_> = teams.into_iter()
        .filter(|t| &t.league_id == league_id)
        .collect();

    let mut actions = Vec::new();

    // Process transfers for each AI team
    for team in ai_teams {
        // Decide transfers
        let targets = decide_ai_transfer(&team.id, database)?;

        // Execute transfers (simplified - just attempt to buy)
        for target in targets.iter().take(2) { // Limit to 2 transfers per team
            match execute_ai_transfer(&team.id, target, database) {
                Ok(action) => {
                    actions.push(action);
                }
                Err(_) => {
                    // Transfer failed, skip
                    continue;
                }
            }
        }

        // Also consider selling players if budget is low
        if team.budget < 5_000_000 {
            let sold = consider_selling_players(&team.id, database)?;
            actions.extend(sold);
        }
    }

    Ok(actions)
}

/// Execute an AI transfer
fn execute_ai_transfer(
    team_id: &str,
    target: &TransferTarget,
    database: &Database,
) -> Result<AITransferAction, TransferDecisionError> {
    let team = match database.team_repo().get_by_id(team_id) {
        Ok(t) => t,
        Err(crate::data::DatabaseError::NotFound(_)) => {
            return Err(TransferDecisionError::TeamNotFound(team_id.to_string()))
        }
        Err(e) => return Err(TransferDecisionError::DatabaseError(e)),
    };

    // Check budget
    if team.budget < target.max_price {
        return Err(TransferDecisionError::InsufficientBudget);
    }

    // Get player from market
    let market_repo = database.transfer_market_repo();
    let market_players = market_repo.get_market_players()?;
    let player = market_players.iter()
        .find(|p| p.id == target.player_id)
        .ok_or(TransferDecisionError::TeamNotFound(target.player_id.clone()))?;

    // Remove from market
    market_repo.remove_from_market(&target.player_id)?;

    // Update player's team
    let _player_repo = database.player_repo();
    // Note: Would need update method on player_repo

    Ok(AITransferAction {
        team_id: team_id.to_string(),
        player_id: target.player_id.clone(),
        action_type: AITransferActionType::Bought(player.market_value),
        reason: target.reason.clone(),
    })
}

/// Consider selling players to raise funds
fn consider_selling_players(
    team_id: &str,
    database: &Database,
) -> Result<Vec<AITransferAction>, TransferDecisionError> {
    let player_repo = database.player_repo();
    let players = player_repo.get_by_team(team_id)?;

    let mut actions = Vec::new();

    // Find players to sell:
    // 1. Old/declining players (>32)
    // 2. Low ability players (<80)
    // 3. Surplus players at a position
    let sell_candidates: Vec<_> = players.iter()
        .filter(|p| p.age > 32 || p.current_ability < 80)
        .filter(|p| p.market_value > 1_000_000) // Only sell if worth something
        .take(3) // Max 3 players to sell
        .collect();

    for player in sell_candidates {
        // Add to transfer market
        let market_repo = database.transfer_market_repo();
        market_repo.add_to_market(&player.id, player.market_value)?;

        actions.push(AITransferAction {
            team_id: team_id.to_string(),
            player_id: player.id.clone(),
            action_type: AITransferActionType::Listed(player.market_value),
            reason: format!(
                "Listed for sale - {} age {}",
                if player.age > 32 { "older player" } else { "below quality threshold" },
                player.age
            ),
        });
    }

    Ok(actions)
}

/// AI transfer action taken
#[derive(Debug, Clone)]
pub struct AITransferAction {
    pub team_id: String,
    pub player_id: String,
    pub action_type: AITransferActionType,
    pub reason: String,
}

/// Type of AI transfer action
#[derive(Debug, Clone, PartialEq)]
pub enum AITransferActionType {
    Bought(u32),  // Price paid
    Listed(u32),  // Asking price
    Sold(u32),    // Sale price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_position_weakness() {
        // Test with empty position (critical weakness)
        let players: Vec<&Player> = vec![];
        let weakness = assess_position_weakness(&Position::GK, &players);
        assert!(weakness.is_some());
        assert_eq!(weakness.unwrap().severity, WeaknessSeverity::Critical);
    }

    #[test]
    fn test_weakness_severity_ordering() {
        assert!(WeaknessSeverity::Critical < WeaknessSeverity::High);
        assert!(WeaknessSeverity::High < WeaknessSeverity::Medium);
        assert!(WeaknessSeverity::Medium < WeaknessSeverity::Low);
    }

    #[test]
    fn test_transfer_priority_ordering() {
        assert!(TransferPriority::Urgent < TransferPriority::High);
        assert!(TransferPriority::High < TransferPriority::Medium);
        assert!(TransferPriority::Medium < TransferPriority::Low);
    }
}
