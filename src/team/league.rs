use serde::{Deserialize, Serialize};

/// League data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct League {
    pub id: String,
    pub name: String,
    pub current_round: u32,
    pub total_rounds: u32,
    pub teams: Vec<String>,  // Team IDs
    pub schedule: MatchSchedule,
}

impl League {
    /// Create a new league
    pub fn new(id: String, name: String, teams: Vec<String>) -> Self {
        let team_count = teams.len();
        let total_rounds = if team_count > 1 {
            (team_count as u32 - 1) * 2  // Double round-robin
        } else {
            0
        };

        Self {
            id,
            name,
            current_round: 0,
            total_rounds,
            teams,
            schedule: MatchSchedule { rounds: Vec::new() },
        }
    }

    /// Generate double round-robin schedule
    pub fn generate_schedule(&mut self) -> &MatchSchedule {
        if self.teams.len() < 2 {
            return &self.schedule;
        }

        let mut rounds: Vec<Round> = Vec::new();
        let mut team_ids = self.teams.clone();

        // If odd number of teams, add a dummy team for bye
        let has_bye = team_ids.len() % 2 != 0;
        if has_bye {
            team_ids.push("BYE".to_string());
        }

        let n = team_ids.len();
        let num_rounds = n - 1;

        for round_num in 0..num_rounds {
            let mut matches = Vec::new();
            let half = n / 2;

            for i in 0..half {
                let home = &team_ids[i];
                let away = &team_ids[n - 1 - i];

                // Skip if one team is bye
                if home != "BYE" && away != "BYE" {
                    matches.push(ScheduledMatch {
                        id: format!("match_{}_{}", round_num, i),
                        home_team_id: home.clone(),
                        away_team_id: away.clone(),
                        played: false,
                    });
                }
            }

            rounds.push(Round {
                round_number: (round_num + 1) as u32,
                matches,
            });

            // Rotate teams (keep first team fixed)
            let last = team_ids.pop().unwrap();
            team_ids.insert(1, last);
        }

        // Second half of season (reverse fixtures)
        let mut second_half = rounds.clone();
        for round in &mut second_half {
            for scheduled_match in &mut round.matches {
                std::mem::swap(&mut scheduled_match.home_team_id, &mut scheduled_match.away_team_id);
                scheduled_match.id = format!("{}_rev", scheduled_match.id);
            }
            round.round_number += num_rounds as u32;
        }

        rounds.extend(second_half);
        self.schedule.rounds = rounds;
        &self.schedule
    }

    /// Get matches for current round
    pub fn get_current_round_matches(&self) -> Option<&Round> {
        self.schedule.rounds.get(self.current_round as usize)
    }

    /// Get matches for specific round
    pub fn get_round_matches(&self, round: u32) -> Option<&Round> {
        self.schedule.rounds.get(round as usize)
    }

    /// Advance to next round
    pub fn advance_round(&mut self) -> bool {
        if self.current_round < self.total_rounds {
            self.current_round += 1;
            true
        } else {
            false
        }
    }

    /// Reset for new season
    pub fn new_season(&mut self) {
        self.current_round = 0;
        for round in &mut self.schedule.rounds {
            for scheduled_match in &mut round.matches {
                scheduled_match.played = false;
            }
        }
    }
}

/// Match schedule containing all rounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSchedule {
    pub rounds: Vec<Round>,
}

/// Round containing multiple matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub round_number: u32,
    pub matches: Vec<ScheduledMatch>,
}

/// Scheduled match between two teams
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMatch {
    pub id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub played: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_league_creation() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
            "team3".to_string(),
            "team4".to_string(),
        ];
        let league = League::new("1".to_string(), "Test League".to_string(), teams);
        assert_eq!(league.name, "Test League");
        assert_eq!(league.teams.len(), 4);
        assert_eq!(league.total_rounds, 6); // (4-1) * 2
    }

    #[test]
    fn test_schedule_generation() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
            "team3".to_string(),
            "team4".to_string(),
        ];
        let mut league = League::new("1".to_string(), "Test League".to_string(), teams);
        league.generate_schedule();

        assert_eq!(league.schedule.rounds.len(), 6);

        // Each round should have 2 matches
        for round in &league.schedule.rounds {
            assert_eq!(round.matches.len(), 2);
        }
    }

    #[test]
    fn test_odd_team_schedule() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
            "team3".to_string(),
        ];
        let mut league = League::new("1".to_string(), "Test League".to_string(), teams);
        league.generate_schedule();

        assert_eq!(league.total_rounds, 4);

        // Some rounds should have 1 match (due to bye)
        let total_matches: usize = league.schedule.rounds.iter()
            .map(|r| r.matches.len())
            .sum();
        assert!(total_matches > 0);
    }

    #[test]
    fn test_advance_round() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
        ];
        let mut league = League::new("1".to_string(), "Test League".to_string(), teams);

        assert_eq!(league.current_round, 0);
        assert!(league.advance_round());
        assert_eq!(league.current_round, 1);
    }

    #[test]
    fn test_get_current_round() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
        ];
        let mut league = League::new("1".to_string(), "Test League".to_string(), teams);
        league.generate_schedule();

        let round = league.get_current_round_matches();
        assert!(round.is_some());
        assert_eq!(round.unwrap().round_number, 1);
    }

    #[test]
    fn test_new_season() {
        let teams = vec![
            "team1".to_string(),
            "team2".to_string(),
        ];
        let mut league = League::new("1".to_string(), "Test League".to_string(), teams);
        league.generate_schedule();

        // Mark some matches as played
        if let Some(round) = league.schedule.rounds.get_mut(0) {
            for scheduled_match in &mut round.matches {
                scheduled_match.played = true;
            }
        }

        league.current_round = 3;
        league.new_season();

        assert_eq!(league.current_round, 0);
        for round in &league.schedule.rounds {
            for scheduled_match in &round.matches {
                assert!(!scheduled_match.played);
            }
        }
    }
}
