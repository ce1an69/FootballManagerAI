use crate::team::{Team, Position};
use crate::game::GameState;
use rand::Rng;

/// Random event types
#[derive(Debug, Clone, PartialEq)]
pub enum RandomEventType {
    Injury,
    TransferOffer,
    MediaStory,
    Scandal,
    Achievement,
    BoardConfidence,
    FanSupport,
}

/// Injury types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InjuryType {
    Muscle,
    Knee,
    Ankle,
    Hamstring,
    Groin,
    Concussion,
    Broken,
}

/// Random game event
#[derive(Debug, Clone)]
pub struct RandomEvent {
    pub id: String,
    pub event_type: RandomEventType,
    pub title: String,
    pub description: String,
    pub affected_team_id: Option<String>,
    pub affected_player_id: Option<String>,
    pub severity: EventSeverity,
    pub impact: EventImpact,
}

/// Event severity
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Event impact on game
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventImpact {
    Positive,
    Negative,
    Neutral,
}

/// Event generator
pub struct EventGenerator {
    rng: rand::rngs::ThreadRng,
}

impl EventGenerator {
    /// Create new event generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate random event based on probability
    pub fn generate_random_event(
        &mut self,
        game_state: &GameState,
    ) -> Option<RandomEvent> {
        // Only generate events occasionally (5% chance per call)
        if self.rng.gen_range(0..=100) > 5 {
            return None;
        }

        // Choose event type based on weights
        let event_type = self.choose_event_type();

        match event_type {
            RandomEventType::Injury => self.generate_injury_event(game_state),
            RandomEventType::TransferOffer => self.generate_transfer_offer_event(game_state),
            RandomEventType::MediaStory => self.generate_media_story_event(game_state),
            RandomEventType::Achievement => self.generate_achievement_event(game_state),
            RandomEventType::BoardConfidence => self.generate_board_confidence_event(game_state),
            _ => None,
        }
    }

    /// Choose event type based on weighted probability
    fn choose_event_type(&mut self) -> RandomEventType {
        let roll = self.rng.gen_range(0..=100);

        match roll {
            0..=15 => RandomEventType::Injury,          // 15%
            16..=30 => RandomEventType::TransferOffer,  // 15%
            31..=55 => RandomEventType::MediaStory,     // 25%
            56..=70 => RandomEventType::Achievement,    // 15%
            71..=85 => RandomEventType::BoardConfidence, // 15%
            86..=100 => RandomEventType::FanSupport,    // 15%
            _ => RandomEventType::MediaStory,
        }
    }

    /// Generate injury event
    fn generate_injury_event(&mut self, game_state: &GameState) -> Option<RandomEvent> {
        // Get player team
        let team = game_state.get_player_team()?;

        // Get player IDs from the team
        let player_ids = &team.players;

        if player_ids.is_empty() {
            return None;
        }

        // Select random player ID
        let player_id = &player_ids[self.rng.gen_range(0..player_ids.len())];

        // Generate random injury type
        let injury_types = [
            InjuryType::Muscle,
            InjuryType::Knee,
            InjuryType::Ankle,
            InjuryType::Hamstring,
            InjuryType::Groin,
            InjuryType::Concussion,
            InjuryType::Broken,
        ];
        let injury_type = injury_types[self.rng.gen_range(0..injury_types.len())];

        // Duration based on severity
        let (duration, severity) = match injury_type {
            InjuryType::Broken => (90 + self.rng.gen_range(0..=90), EventSeverity::Critical),
            InjuryType::Knee => (60 + self.rng.gen_range(0..=60), EventSeverity::High),
            InjuryType::Concussion => (14 + self.rng.gen_range(0..=28), EventSeverity::Medium),
            _ => (7 + self.rng.gen_range(0..=21), EventSeverity::Medium),
        };

        Some(RandomEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: RandomEventType::Injury,
            title: format!("Injury Reported"),
            description: format!(
                "A player has suffered a {} injury and will be out for {} days",
                format!("{:?}", injury_type).to_lowercase(),
                duration
            ),
            affected_team_id: Some(game_state.player_team_id.clone()),
            affected_player_id: Some(player_id.clone()),
            severity,
            impact: EventImpact::Negative,
        })
    }

    /// Generate transfer offer event
    fn generate_transfer_offer_event(&mut self, game_state: &GameState) -> Option<RandomEvent> {
        // Get a random team in the league
        let teams: Vec<_> = game_state.teams.values().collect();
        if teams.len() < 2 {
            return None;
        }

        let target_team = teams[self.rng.gen_range(0..teams.len())];
        let offering_teams: Vec<_> = teams.iter()
            .filter(|t| t.id != target_team.id)
            .take(3)
            .collect();

        if offering_teams.is_empty() {
            return None;
        }

        let offering_team = offering_teams[self.rng.gen_range(0..offering_teams.len())];

        // Get a player ID from target team
        let player_ids = &target_team.players;
        if player_ids.is_empty() {
            return None;
        }

        let player_id = &player_ids[self.rng.gen_range(0..player_ids.len())];
        let offer_amount = 1000000 + self.rng.gen_range(0..=5000000);

        Some(RandomEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: RandomEventType::TransferOffer,
            title: format!("Transfer Offer Received"),
            description: format!(
                "{} has made a transfer offer of ${} for one of your players",
                offering_team.name,
                offer_amount
            ),
            affected_team_id: Some(target_team.id.clone()),
            affected_player_id: Some(player_id.clone()),
            severity: EventSeverity::Medium,
            impact: EventImpact::Neutral,
        })
    }

    /// Generate media story event
    fn generate_media_story_event(&mut self, game_state: &GameState) -> Option<RandomEvent> {
        let team_name = game_state.get_player_team()
            .map(|t| t.name.as_str())
            .unwrap_or("your club");

        let stories = [
            ("Transfer Rumor", format!("Rumors circulate about a big money move involving {}", team_name)),
            ("Manager Praise", format!("Media praises {}'s recent tactical decisions", team_name)),
            ("Fan Excitement", format!("Fans excited about {}'s recent form", team_name)),
            ("Title Race", format!("{} enters title race discussion", team_name)),
            ("Young Star", format!("Media highlights {} as a rising star to watch", team_name)),
        ];

        let (title, description) = stories[self.rng.gen_range(0..stories.len())].clone();

        Some(RandomEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: RandomEventType::MediaStory,
            title: title.to_string(),
            description,
            affected_team_id: Some(game_state.player_team_id.clone()),
            affected_player_id: None,
            severity: EventSeverity::Low,
            impact: EventImpact::Neutral,
        })
    }

    /// Generate achievement event
    fn generate_achievement_event(&mut self, game_state: &GameState) -> Option<RandomEvent> {
        // Check for achievements based on game state
        if game_state.current_date.month == 5 && game_state.current_date.day == 31 {
            // Season end achievement
            Some(RandomEvent {
                id: uuid::Uuid::new_v4().to_string(),
                event_type: RandomEventType::Achievement,
                title: "Season Complete!".to_string(),
                description: format!(
                    "Season {} has come to an end. {} finished the season.",
                    game_state.current_date.season_string(),
                    game_state.get_player_team().map(|t| t.name.as_str()).unwrap_or("Your team")
                ),
                affected_team_id: Some(game_state.player_team_id.clone()),
                affected_player_id: None,
                severity: EventSeverity::Medium,
                impact: EventImpact::Positive,
            })
        } else {
            None
        }
    }

    /// Generate board confidence event
    fn generate_board_confidence_event(&mut self, game_state: &GameState) -> Option<RandomEvent> {
        let team = game_state.get_player_team()?;

        // Determine confidence based on recent form (simulated)
        let confidence_level = if self.rng.gen_bool(0.5) {
            "High"
        } else {
            "Low"
        };

        let (severity, impact) = if confidence_level == "High" {
            (EventSeverity::Low, EventImpact::Positive)
        } else {
            (EventSeverity::Medium, EventImpact::Negative)
        };

        Some(RandomEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: RandomEventType::BoardConfidence,
            title: format!("Board Confidence {}", confidence_level),
            description: format!(
                "The {} board expresses {} confidence in your management.",
                team.name,
                confidence_level.to_lowercase()
            ),
            affected_team_id: Some(game_state.player_team_id.clone()),
            affected_player_id: None,
            severity,
            impact,
        })
    }

    /// Check if event should trigger based on game date
    pub fn should_generate_event(&mut self, game_state: &GameState) -> bool {
        // Higher chance of events during transfer windows
        let base_chance = if game_state.current_date.is_transfer_window() {
            15
        } else {
            5
        };

        self.rng.gen_range(0..=100) < base_chance
    }
}

impl Default for EventGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_generator_creation() {
        let mut gen = EventGenerator::new();
        // Just verify it creates successfully and can generate event types
        let event_type = gen.choose_event_type();
        // Verify it's a valid event type
        match event_type {
            RandomEventType::Injury |
            RandomEventType::TransferOffer |
            RandomEventType::MediaStory |
            RandomEventType::Scandal |
            RandomEventType::Achievement |
            RandomEventType::BoardConfidence |
            RandomEventType::FanSupport => {}, // All valid
        }
    }

    #[test]
    fn test_should_generate_event() {
        let mut gen = EventGenerator::new();
        let game_state = create_test_state();

        // Test probability
        let mut count = 0;
        for _ in 0..1000 {
            if gen.should_generate_event(&game_state) {
                count += 1;
            }
        }
        // Should have some events (around 5% or 15% depending on transfer window)
        assert!(count > 0 && count < 500);
    }

    fn create_test_state() -> GameState {
        use crate::team::{League, Team};
        use crate::game::GameDate;

        let league = League::new("test".to_string(), "Test".to_string(), vec![]);
        let team = Team::new("team1".to_string(), "Test Team".to_string(), "test".to_string(), 1000000);

        let mut state = GameState::new("team1".to_string(), league, vec![team], 2026);
        state.current_date = GameDate { year: 2026, month: 9, day: 15 }; // During transfer window
        state
    }
}
