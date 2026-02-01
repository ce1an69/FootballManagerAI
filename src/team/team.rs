use crate::team::{player::Player, tactics::Tactic, Position, PlayerRole, Duty};
use serde::{Deserialize, Serialize};

/// Team data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub league_id: String,
    pub budget: u32,
    pub players: Vec<String>,  // Player IDs
    pub starting_11: Vec<PlayerSlot>,
    pub bench: Vec<PlayerSlot>,
    pub tactic: Tactic,
    pub statistics: TeamStatistics,
}

/// Player slot in lineup with position and role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSlot {
    pub player_id: String,
    pub position: Position,
    pub role: PlayerRole,
    pub duty: Duty,
}

/// Team statistics for current season
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamStatistics {
    pub matches_played: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub goals_for: u32,
    pub goals_against: u32,
    pub points: u32,
    pub league_position: Option<u32>,
}

impl Team {
    /// Create a new team
    pub fn new(id: String, name: String, league_id: String, budget: u32) -> Self {
        Self {
            id,
            name,
            league_id,
            budget,
            players: Vec::new(),
            starting_11: Vec::new(),
            bench: Vec::new(),
            tactic: Tactic::default(),
            statistics: TeamStatistics::default(),
        }
    }

    /// Add a player to the team
    pub fn add_player(&mut self, player_id: String) {
        if !self.players.contains(&player_id) {
            self.players.push(player_id);
        }
    }

    /// Remove a player from the team
    pub fn remove_player(&mut self, player_id: &str) {
        self.players.retain(|id| id != player_id);
        self.starting_11.retain(|slot| slot.player_id != player_id);
        self.bench.retain(|slot| slot.player_id != player_id);
    }

    /// Set the starting 11 lineup
    pub fn set_starting_11(&mut self, players: Vec<PlayerSlot>) {
        if players.len() == 11 {
            self.starting_11 = players;
        }
    }

    /// Get overall team strength based on players
    pub fn get_team_strength(&self, players: &[Player]) -> u16 {
        if self.starting_11.is_empty() {
            return 0;
        }

        let mut total_strength = 0;
        let mut count = 0;

        for slot in &self.starting_11 {
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                total_strength += player.get_position_rating(&slot.position);
                count += 1;
            }
        }

        if count == 0 {
            0
        } else {
            total_strength / count
        }
    }

    /// Calculate attack strength
    pub fn calculate_attack_strength(&self, players: &[Player]) -> u16 {
        let attacking_players = self.starting_11.iter()
            .filter(|slot| {
                matches!(slot.position, Position::ST | Position::CF | Position::LW | Position::RW | Position::AM)
            });

        let mut total = 0;
        let mut count = 0;

        for slot in attacking_players {
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                // Weight key attacking attributes
                let attack_rating = (
                    player.finishing as u32 * 3 +
                    player.off_the_ball as u32 * 2 +
                    player.pace as u32 * 2 +
                    player.dribbling as u32 +
                    player.heading as u32
                ) / 9;

                total += attack_rating as u16;
                count += 1;
            }
        }

        if count == 0 { 0 } else { total / count }
    }

    /// Calculate defense strength
    pub fn calculate_defense_strength(&self, players: &[Player]) -> u16 {
        let defending_players = self.starting_11.iter()
            .filter(|slot| {
                matches!(slot.position, Position::GK | Position::CB | Position::LB | Position::RB | Position::WB | Position::DM)
            });

        let mut total = 0;
        let mut count = 0;

        for slot in defending_players {
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                let defense_rating = if player.is_gk() {
                    // GK attributes
                    (player.handling + player.reflexes + player.gk_positioning +
                     player.aerial_reach + player.command_of_area) / 5
                } else {
                    // Outfield defender attributes
                    (player.tackling * 3 +
                     player.marking * 3 +
                     player.positioning * 2 +
                     player.strength +
                     player.heading) / 10
                };

                total += defense_rating;
                count += 1;
            }
        }

        if count == 0 { 0 } else { total / count }
    }

    /// Calculate midfield strength
    pub fn calculate_midfield_strength(&self, players: &[Player]) -> u16 {
        let midfield_players = self.starting_11.iter()
            .filter(|slot| {
                matches!(slot.position, Position::CM | Position::DM | Position::AM | Position::LW | Position::RW)
            });

        let mut total = 0;
        let mut count = 0;

        for slot in midfield_players {
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                let midfield_rating = (
                    player.passing as u32 * 3 +
                    player.vision as u32 * 2 +
                    player.stamina as u32 * 2 +
                    player.teamwork as u32 +
                    player.work_rate as u32
                ) / 9;

                total += midfield_rating as u16;
                count += 1;
            }
        }

        if count == 0 { 0 } else { total / count }
    }

    /// Update statistics after a match
    pub fn update_statistics(&mut self, won: bool, drawn: bool, goals_for: u32, goals_against: u32) {
        self.statistics.matches_played += 1;
        self.statistics.goals_for += goals_for;
        self.statistics.goals_against += goals_against;

        if won {
            self.statistics.wins += 1;
            self.statistics.points += 3;
        } else if drawn {
            self.statistics.draws += 1;
            self.statistics.points += 1;
        } else {
            self.statistics.losses += 1;
        }
    }

    /// Calculate effective strength considering player conditions
    pub fn calculate_effective_attack(&self, players: &[Player]) -> u16 {
        let base_strength = self.calculate_attack_strength(players);
        let morale_modifier = if self.starting_11.is_empty() {
            100
        } else {
            let mut total_morale = 0;
            let mut count = 0;
            for slot in &self.starting_11 {
                if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                    total_morale += player.morale;
                    count += 1;
                }
            }
            if count == 0 { 100 } else { (total_morale / count) as u16 }
        };

        (base_strength * morale_modifier) / 100
    }

    /// Calculate effective defense considering player conditions
    pub fn calculate_effective_defense(&self, players: &[Player]) -> u16 {
        let base_strength = self.calculate_defense_strength(players);
        let fatigue_modifier = if self.starting_11.is_empty() {
            100
        } else {
            let mut avg_fatigue = 0;
            let mut count = 0;
            for slot in &self.starting_11 {
                if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                    avg_fatigue += player.fatigue;
                    count += 1;
                }
            }
            if count == 0 { 100 } else { 100 - (avg_fatigue / count) as u16 }
        };

        (base_strength * fatigue_modifier.max(50)) / 100
    }

    /// Calculate effective midfield considering player conditions
    pub fn calculate_effective_midfield(&self, players: &[Player]) -> u16 {
        let base_strength = self.calculate_midfield_strength(players);

        // Consider fitness
        let fitness_modifier = if self.starting_11.is_empty() {
            100
        } else {
            let mut total_fitness = 0;
            let mut count = 0;
            for slot in &self.starting_11 {
                if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                    total_fitness += player.match_fitness;
                    count += 1;
                }
            }
            if count == 0 { 100 } else { (total_fitness / count) as u16 }
        };

        (base_strength * fitness_modifier) / 100
    }
}

impl TeamStatistics {
    /// Get goal difference
    pub fn goal_difference(&self) -> i32 {
        (self.goals_for as i32) - (self.goals_against as i32)
    }

    /// Get win rate (percentage)
    pub fn win_rate(&self) -> f32 {
        if self.matches_played == 0 {
            0.0
        } else {
            (self.wins as f32 / self.matches_played as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::tactics::{Formation, Duty};

    #[test]
    fn test_team_creation() {
        let team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        assert_eq!(team.name, "Test");
        assert_eq!(team.budget, 1000000);
        assert!(team.players.is_empty());
    }

    #[test]
    fn test_add_player() {
        let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        team.add_player("player1".to_string());
        assert!(team.players.contains(&"player1".to_string()));
        assert_eq!(team.players.len(), 1);
    }

    #[test]
    fn test_remove_player() {
        let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        team.add_player("player1".to_string());
        team.add_player("player2".to_string());
        team.remove_player("player1");
        assert!(!team.players.contains(&"player1".to_string()));
        assert!(team.players.contains(&"player2".to_string()));
    }

    #[test]
    fn test_update_statistics() {
        let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        team.update_statistics(true, false, 2, 1);
        assert_eq!(team.statistics.matches_played, 1);
        assert_eq!(team.statistics.wins, 1);
        assert_eq!(team.statistics.goals_for, 2);
        assert_eq!(team.statistics.goals_against, 1);
        assert_eq!(team.statistics.points, 3);
    }

    #[test]
    fn test_draw_statistics() {
        let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        team.update_statistics(false, true, 1, 1);
        assert_eq!(team.statistics.draws, 1);
        assert_eq!(team.statistics.points, 1);
    }

    #[test]
    fn test_loss_statistics() {
        let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
        team.update_statistics(false, false, 0, 1);
        assert_eq!(team.statistics.losses, 1);
        assert_eq!(team.statistics.points, 0);
    }
}
