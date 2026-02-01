use crate::team::Position;
use serde::{Deserialize, Serialize};

/// Match result data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub id: String,
    pub league_id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_score: u8,
    pub away_score: u8,
    pub match_mode: MatchMode,
    pub events: Vec<MatchEvent>,
    pub statistics: MatchStatistics,
    pub played_at: u64,
    pub round: u32,
}

/// Match mode (how the match was simulated/watched)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchMode {
    Live,
    Quick,
}

/// Detailed match statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchStatistics {
    // Possession
    pub home_possession: f32,
    pub away_possession: f32,

    // Shots
    pub home_shots: u16,
    pub away_shots: u16,
    pub home_shots_on_target: u16,
    pub away_shots_on_target: u16,

    // Passing
    pub home_passes: u16,
    pub away_passes: u16,
    pub home_pass_accuracy: f32,
    pub away_pass_accuracy: f32,

    // Other
    pub home_corners: u8,
    pub away_corners: u8,
    pub home_fouls: u8,
    pub away_fouls: u8,
    pub home_offsides: u8,
    pub away_offsides: u8,

    // Player ratings
    pub home_player_ratings: Vec<PlayerMatchRating>,
    pub away_player_ratings: Vec<PlayerMatchRating>,
}

/// Player rating for a single match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchRating {
    pub player_id: String,
    pub player_name: String,
    pub position: Position,
    pub rating: f32,           // 6.0 - 10.0
    pub minutes_played: u8,
    pub goals: u8,
    pub assists: u8,
}

impl MatchStatistics {
    /// Calculate home shot accuracy percentage
    pub fn home_shot_accuracy(&self) -> f32 {
        if self.home_shots == 0 {
            0.0
        } else {
            (self.home_shots_on_target as f32 / self.home_shots as f32) * 100.0
        }
    }

    /// Calculate away shot accuracy percentage
    pub fn away_shot_accuracy(&self) -> f32 {
        if self.away_shots == 0 {
            0.0
        } else {
            (self.away_shots_on_target as f32 / self.away_shots as f32) * 100.0
        }
    }

    /// Get man of the match (highest rated player)
    pub fn man_of_the_match(&self) -> Option<&PlayerMatchRating> {
        self.home_player_ratings.iter()
            .chain(self.away_player_ratings.iter())
            .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap())
    }
}

/// Match events (goals, cards, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEvent {
    Goal {
        team: String,
        player_id: String,
        minute: u8,
    },
    OwnGoal {
        team: String,
        player_id: String,
        minute: u8,
    },
    Penalty {
        team: String,
        player_id: String,
        minute: u8,
        scored: bool,
    },
    YellowCard {
        team: String,
        player_id: String,
        minute: u8,
    },
    RedCard {
        team: String,
        player_id: String,
        minute: u8,
    },
    Injury {
        team: String,
        player_id: String,
        minute: u8,
        severity: u8,
    },
    Substitution {
        team: String,
        player_out: String,
        player_in: String,
        minute: u8,
    },
}

impl MatchResult {
    /// Create a new match result
    pub fn new(
        id: String,
        league_id: String,
        home_team_id: String,
        away_team_id: String,
        match_mode: MatchMode,
        round: u32,
    ) -> Self {
        Self {
            id,
            league_id,
            home_team_id,
            away_team_id,
            home_score: 0,
            away_score: 0,
            match_mode,
            events: Vec::new(),
            statistics: MatchStatistics::default(),
            played_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            round,
        }
    }

    /// Check if home team won
    pub fn home_won(&self) -> bool {
        self.home_score > self.away_score
    }

    /// Check if away team won
    pub fn away_won(&self) -> bool {
        self.away_score > self.home_score
    }

    /// Check if match was a draw
    pub fn is_draw(&self) -> bool {
        self.home_score == self.away_score
    }

    /// Get winning team ID (None if draw)
    pub fn winner(&self) -> Option<&String> {
        if self.home_won() {
            Some(&self.home_team_id)
        } else if self.away_won() {
            Some(&self.away_team_id)
        } else {
            None
        }
    }

    /// Add goal event
    pub fn add_goal(&mut self, team: String, player_id: String, minute: u8) {
        if team == self.home_team_id {
            self.home_score += 1;
        } else {
            self.away_score += 1;
        }
        self.events.push(MatchEvent::Goal {
            team,
            player_id,
            minute,
        });
    }

    /// Add own goal
    pub fn add_own_goal(&mut self, team: String, player_id: String, minute: u8) {
        // Own goal counts for opposite team
        if team == self.home_team_id {
            self.away_score += 1;
        } else {
            self.home_score += 1;
        }
        self.events.push(MatchEvent::OwnGoal {
            team,
            player_id,
            minute,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_creation() {
        let result = MatchResult::new(
            "1".to_string(),
            "league1".to_string(),
            "team1".to_string(),
            "team2".to_string(),
            MatchMode::Quick,
            1,
        );

        assert_eq!(result.home_team_id, "team1");
        assert_eq!(result.away_score, 0);
        assert!(!result.played_at > 0);
    }

    #[test]
    fn test_add_goal() {
        let mut result = MatchResult::new(
            "1".to_string(),
            "league1".to_string(),
            "team1".to_string(),
            "team2".to_string(),
            MatchMode::Quick,
            1,
        );

        result.add_goal("team1".to_string(), "player1".to_string(), 45);
        assert_eq!(result.home_score, 1);
        assert_eq!(result.events.len(), 1);
    }

    #[test]
    fn test_winner() {
        let mut result = MatchResult::new(
            "1".to_string(),
            "league1".to_string(),
            "team1".to_string(),
            "team2".to_string(),
            MatchMode::Quick,
            1,
        );

        result.add_goal("team1".to_string(), "player1".to_string(), 45);
        assert_eq!(result.winner(), Some(&result.home_team_id));
        assert!(result.home_won());
        assert!(!result.away_won());
        assert!(!result.is_draw());
    }

    #[test]
    fn test_draw() {
        let mut result = MatchResult::new(
            "1".to_string(),
            "league1".to_string(),
            "team1".to_string(),
            "team2".to_string(),
            MatchMode::Quick,
            1,
        );

        result.add_goal("team1".to_string(), "player1".to_string(), 45);
        result.add_goal("team2".to_string(), "player2".to_string(), 60);
        assert!(result.is_draw());
        assert_eq!(result.winner(), None);
    }

    #[test]
    fn test_own_goal() {
        let mut result = MatchResult::new(
            "1".to_string(),
            "league1".to_string(),
            "team1".to_string(),
            "team2".to_string(),
            MatchMode::Quick,
            1,
        );

        result.add_own_goal("team1".to_string(), "player1".to_string(), 45);
        assert_eq!(result.away_score, 1);
        assert_eq!(result.home_score, 0);
    }

    #[test]
    fn test_shot_accuracy() {
        let mut stats = MatchStatistics::default();
        stats.home_shots = 10;
        stats.home_shots_on_target = 4;

        assert!((stats.home_shot_accuracy() - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_man_of_the_match() {
        let mut stats = MatchStatistics::default();
        stats.home_player_ratings.push(PlayerMatchRating {
            player_id: "1".to_string(),
            player_name: "Player 1".to_string(),
            position: Position::ST,
            rating: 7.5,
            minutes_played: 90,
            goals: 1,
            assists: 0,
        });
        stats.home_player_ratings.push(PlayerMatchRating {
            player_id: "2".to_string(),
            player_name: "Player 2".to_string(),
            position: Position::CM,
            rating: 8.5,
            minutes_played: 90,
            goals: 0,
            assists: 2,
        });

        let motm = stats.man_of_the_match();
        assert!(motm.is_some());
        assert_eq!(motm.unwrap().player_id, "2");
    }
}
