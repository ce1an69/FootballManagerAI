use serde::{Deserialize, Serialize};

/// Player statistics for a single season
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSeasonStats {
    pub player_id: String,
    pub season: String,          // e.g., "2026-27"
    pub team_id: String,

    // Appearance data
    pub appearances: u32,        // Total appearances
    pub starts: u32,             // Started matches
    pub minutes_played: u32,     // Minutes played

    // Attack data
    pub goals: u32,              // Goals scored
    pub assists: u32,            // Assists
    pub shots: u32,              // Shots taken
    pub shots_on_target: u32,    // Shots on target

    // Discipline data
    pub yellow_cards: u32,       // Yellow cards
    pub red_cards: u32,          // Red cards

    // Ratings
    pub total_rating: f32,       // Total rating points
    pub average_rating: f32,     // Average rating

    // Best performances
    pub man_of_the_match: u32,   // Man of the match awards
}

impl PlayerSeasonStats {
    /// Create new season stats for a player
    pub fn new(player_id: String, season: String, team_id: String) -> Self {
        Self {
            player_id,
            season,
            team_id,
            ..Default::default()
        }
    }

    /// Record match data
    pub fn record_match(
        &mut self,
        minutes: u32,
        started: bool,
        goals: u32,
        assists: u32,
        yellow: bool,
        red: bool,
        rating: f32,
    ) {
        self.appearances += 1;
        if started {
            self.starts += 1;
        }
        self.minutes_played += minutes;
        self.goals += goals;
        self.assists += assists;
        if yellow {
            self.yellow_cards += 1;
        }
        if red {
            self.red_cards += 1;
        }
        self.total_rating += rating;
        self.average_rating = self.total_rating / self.appearances as f32;
    }

    /// Calculate goals per 90 minutes
    pub fn goals_per_90(&self) -> f32 {
        if self.minutes_played == 0 {
            0.0
        } else {
            (self.goals as f32 / self.minutes_played as f32) * 90.0
        }
    }

    /// Calculate assists per 90 minutes
    pub fn assists_per_90(&self) -> f32 {
        if self.minutes_played == 0 {
            0.0
        } else {
            (self.assists as f32 / self.minutes_played as f32) * 90.0
        }
    }

    /// Calculate shot accuracy
    pub fn shot_accuracy(&self) -> f32 {
        if self.shots == 0 {
            0.0
        } else {
            (self.shots_on_target as f32 / self.shots as f32) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_stats_creation() {
        let stats = PlayerSeasonStats::new(
            "1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        assert_eq!(stats.player_id, "1");
        assert_eq!(stats.season, "2026-27");
        assert_eq!(stats.appearances, 0);
    }

    #[test]
    fn test_record_match() {
        let mut stats = PlayerSeasonStats::new(
            "1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        stats.record_match(90, true, 2, 1, false, false, 8.5);

        assert_eq!(stats.appearances, 1);
        assert_eq!(stats.starts, 1);
        assert_eq!(stats.minutes_played, 90);
        assert_eq!(stats.goals, 2);
        assert_eq!(stats.assists, 1);
        assert!((stats.average_rating - 8.5).abs() < 0.01);
    }

    #[test]
    fn test_goals_per_90() {
        let mut stats = PlayerSeasonStats::new(
            "1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        stats.record_match(90, true, 2, 0, false, false, 7.0);
        stats.record_match(45, false, 1, 0, false, false, 7.0);

        assert_eq!(stats.goals, 3);
        assert_eq!(stats.minutes_played, 135);
        assert!((stats.goals_per_90() - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_shot_accuracy() {
        let mut stats = PlayerSeasonStats::new(
            "1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        stats.shots = 10;
        stats.shots_on_target = 4;

        assert!((stats.shot_accuracy() - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_yellow_and_red_cards() {
        let mut stats = PlayerSeasonStats::new(
            "1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        stats.record_match(90, true, 0, 0, true, false, 7.0);
        stats.record_match(90, true, 0, 0, false, true, 6.0);

        assert_eq!(stats.yellow_cards, 1);
        assert_eq!(stats.red_cards, 1);
    }
}
