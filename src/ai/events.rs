use crate::game::{Notification, NotificationType, NotificationPriority};
use crate::team::Player;
use rand::Rng;

/// Injury types with duration information
#[derive(Debug, Clone, PartialEq)]
pub enum InjuryType {
    /// Minor injury - 1-3 weeks
    Minor { weeks: u32 },
    /// Moderate injury - 4-8 weeks
    Moderate { weeks: u32 },
    /// Severe injury - 9-26 weeks
    Severe { weeks: u32 },
    /// Career ending injury
    CareerEnding,
}

impl InjuryType {
    /// Get the duration in weeks, if applicable
    pub fn weeks(&self) -> Option<u32> {
        match self {
            InjuryType::Minor { weeks } => Some(*weeks),
            InjuryType::Moderate { weeks } => Some(*weeks),
            InjuryType::Severe { weeks } => Some(*weeks),
            InjuryType::CareerEnding => None,
        }
    }

    /// Create a random minor injury
    pub fn random_minor() -> Self {
        let mut rng = rand::thread_rng();
        let weeks = rng.gen_range(1..=3);
        InjuryType::Minor { weeks }
    }

    /// Create a random moderate injury
    pub fn random_moderate() -> Self {
        let mut rng = rand::thread_rng();
        let weeks = rng.gen_range(4..=8);
        InjuryType::Moderate { weeks }
    }

    /// Create a random severe injury
    pub fn random_severe() -> Self {
        let mut rng = rand::thread_rng();
        let weeks = rng.gen_range(9..=26);
        InjuryType::Severe { weeks }
    }
}

/// Random event types that can occur during the game
#[derive(Debug, Clone, PartialEq)]
pub enum RandomEvent {
    /// Player injury event
    Injury {
        player_id: String,
        player_name: String,
        injury_type: InjuryType,
    },

    /// Transfer offer for a player
    TransferOffer {
        player_id: String,
        player_name: String,
        from_team: String,
        offer_amount: u64,
    },

    /// Media story about the team or a player
    MediaStory {
        headline: String,
        impact: MediaImpact,
    },

    /// Contract expiry notification
    ContractExpiry {
        player_id: String,
        player_name: String,
    },
}

/// Impact level of media stories
#[derive(Debug, Clone, PartialEq)]
pub enum MediaImpact {
    Positive,
    Negative,
    Neutral,
}

impl RandomEvent {
    /// Convert the random event to a notification
    pub fn to_notification(&self) -> Notification {
        match self {
            RandomEvent::Injury { player_id: _, player_name, injury_type } => {
                let (title, message, priority) = match injury_type {
                    InjuryType::Minor { weeks } => (
                        "Minor Injury Reported".to_string(),
                        format!("{} has suffered a minor injury and will be out for {} week(s).", player_name, weeks),
                        NotificationPriority::Normal,
                    ),
                    InjuryType::Moderate { weeks } => (
                        "Injury Update".to_string(),
                        format!("{} has suffered a moderate injury and will be out for {} weeks.", player_name, weeks),
                        NotificationPriority::High,
                    ),
                    InjuryType::Severe { weeks } => (
                        "Severe Injury Alert".to_string(),
                        format!("{} has suffered a severe injury and will be out for {} weeks.", player_name, weeks),
                        NotificationPriority::Urgent,
                    ),
                    InjuryType::CareerEnding => (
                        "Tragic Injury News".to_string(),
                        format!("{} has suffered a career-ending injury. Our thoughts are with them.", player_name),
                        NotificationPriority::Urgent,
                    ),
                };

                Notification {
                    id: uuid::Uuid::new_v4().to_string(),
                    title,
                    message,
                    notification_type: NotificationType::Injury,
                    priority,
                    read: false,
                    created_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }

            RandomEvent::TransferOffer { player_id: _, player_name, from_team, offer_amount } => {
                Notification {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Transfer Offer Received".to_string(),
                    message: format!(
                        "{} has made a ${} offer for {}. Transfer window is open.",
                        from_team,
                        offer_amount,
                        player_name
                    ),
                    notification_type: NotificationType::Transfer,
                    priority: NotificationPriority::High,
                    read: false,
                    created_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }

            RandomEvent::MediaStory { headline, impact } => {
                let priority = match impact {
                    MediaImpact::Positive => NotificationPriority::Normal,
                    MediaImpact::Negative => NotificationPriority::High,
                    MediaImpact::Neutral => NotificationPriority::Low,
                };

                Notification {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: "Media Report".to_string(),
                    message: headline.clone(),
                    notification_type: NotificationType::News,
                    priority,
                    read: false,
                    created_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }

            RandomEvent::ContractExpiry { player_id: _, player_name } => {
                Notification {
                    id: uuid::Uuid::new_v4().to_string(),
                    title: " Contract Expiring Soon".to_string(),
                    message: format!(
                        "{}'s contract is expiring soon. Consider renewing or they may leave on a free transfer.",
                        player_name
                    ),
                    notification_type: NotificationType::Contract,
                    priority: NotificationPriority::High,
                    read: false,
                    created_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                }
            }
        }
    }
}

/// Generate an injury event for a player
/// 5% probability per match
/// Distribution: 10% career-ending, 20% severe, 30% moderate, 40% minor
pub fn generate_injury_event(player: &Player) -> Option<RandomEvent> {
    let mut rng = rand::thread_rng();

    // 5% probability of injury
    if rng.gen_range(0..100) >= 5 {
        return None;
    }

    // Determine injury type based on distribution
    let roll = rng.gen_range(0..100);
    let injury_type = if roll < 10 {
        // 10% career-ending
        InjuryType::CareerEnding
    } else if roll < 30 {
        // 20% severe
        InjuryType::random_severe()
    } else if roll < 60 {
        // 30% moderate
        InjuryType::random_moderate()
    } else {
        // 40% minor
        InjuryType::random_minor()
    };

    Some(RandomEvent::Injury {
        player_id: player.id.clone(),
        player_name: player.name.clone(),
        injury_type,
    })
}

/// Generate a transfer offer for a player
/// 2% probability per week
/// Only for players with current_ability >= 100
/// Offer 80-120% of market value
pub fn generate_transfer_offer(player: &Player) -> Option<RandomEvent> {
    let mut rng = rand::thread_rng();

    // Only for good players
    if player.current_ability < 100 {
        return None;
    }

    // 2% probability per week
    if rng.gen_range(0..100) >= 2 {
        return None;
    }

    // Generate offer amount: 80-120% of market value
    let multiplier = rng.gen_range(80..=120) as f64 / 100.0;
    let offer_amount = (player.market_value as f64 * multiplier) as u64;

    // Random big team names
    let big_teams = vec![
        "Real Madrid",
        "Barcelona",
        "Manchester City",
        "Manchester United",
        "Liverpool",
        "Bayern Munich",
        "PSG",
        "Juventus",
        "AC Milan",
        "Inter Milan",
        "Chelsea",
        "Arsenal",
        "Tottenham Hotspur",
        "Borussia Dortmund",
        "Atletico Madrid",
    ];
    let from_team = big_teams[rng.gen_range(0..big_teams.len())].to_string();

    Some(RandomEvent::TransferOffer {
        player_id: player.id.clone(),
        player_name: player.name.clone(),
        from_team,
        offer_amount,
    })
}

/// Generate a media story about a team
/// 10% probability per week
/// Random headline from templates with random impact
pub fn generate_media_story(team_name: &str) -> Option<RandomEvent> {
    let mut rng = rand::thread_rng();

    // 10% probability per week
    if rng.gen_range(0..100) >= 10 {
        return None;
    }

    // Determine impact (positive, negative, or neutral)
    let impact_roll = rng.gen_range(0..100);
    let impact = if impact_roll < 40 {
        MediaImpact::Positive
    } else if impact_roll < 70 {
        MediaImpact::Negative
    } else {
        MediaImpact::Neutral
    };

    // Generate headline based on impact
    let headline = match impact {
        MediaImpact::Positive => {
            let positive_headlines = vec![
                format!("{} shines in brilliant team performance!", team_name),
                format!("{}'s title charge gains momentum", team_name),
                format!("{} fans dreaming of glory after latest win", team_name),
                format!("Manager praised for {}'s tactical masterclass", team_name),
                format!("{} stars impress in training camp", team_name),
                format!("{}'s youth system bearing fruit", team_name),
            ];
            positive_headlines[rng.gen_range(0..positive_headlines.len())].clone()
        }
        MediaImpact::Negative => {
            let negative_headlines = vec![
                format!("{} suffers shock defeat", team_name),
                format!("Crisis looms at {} after poor run", team_name),
                format!("{} board considering manager's future", team_name),
                format!("Fan protests outside {} stadium", team_name),
                format!("{} star linked with sensational exit", team_name),
                format!("Injury crisis hits {} hard", team_name),
            ];
            negative_headlines[rng.gen_range(0..negative_headlines.len())].clone()
        }
        MediaImpact::Neutral => {
            let neutral_headlines = vec![
                format!("{} draws in entertaining match", team_name),
                format!("{} prepares for busy fixture schedule", team_name),
                format!("{} captain gives exclusive interview", team_name),
                format!("{} academy expansion announced", team_name),
                format!("{} announces ticket price changes", team_name),
                format!("{} scheduled for midweek friendly", team_name),
            ];
            neutral_headlines[rng.gen_range(0..neutral_headlines.len())].clone()
        }
    };

    Some(RandomEvent::MediaStory { headline, impact })
}

/// Generate a random event for a player/team
/// Randomly selects one of the event generators
pub fn generate_random_event(player: &Player, team_name: &str) -> Option<RandomEvent> {
    let mut rng = rand::thread_rng();

    // Weights: injury (40), transfer (30), media (30)
    let roll = rng.gen_range(0..100);

    if roll < 40 {
        generate_injury_event(player)
    } else if roll < 70 {
        generate_transfer_offer(player)
    } else {
        generate_media_story(team_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Position;

    #[test]
    fn test_injury_type_weeks() {
        let minor = InjuryType::Minor { weeks: 2 };
        assert_eq!(minor.weeks(), Some(2));

        let moderate = InjuryType::Moderate { weeks: 6 };
        assert_eq!(moderate.weeks(), Some(6));

        let severe = InjuryType::Severe { weeks: 15 };
        assert_eq!(severe.weeks(), Some(15));

        let career_ending = InjuryType::CareerEnding;
        assert_eq!(career_ending.weeks(), None);
    }

    #[test]
    fn test_random_injury_creation() {
        let minor = InjuryType::random_minor();
        match minor {
            InjuryType::Minor { weeks } => {
                assert!(weeks >= 1 && weeks <= 3);
            }
            _ => panic!("Expected Minor injury"),
        }

        let moderate = InjuryType::random_moderate();
        match moderate {
            InjuryType::Moderate { weeks } => {
                assert!(weeks >= 4 && weeks <= 8);
            }
            _ => panic!("Expected Moderate injury"),
        }

        let severe = InjuryType::random_severe();
        match severe {
            InjuryType::Severe { weeks } => {
                assert!(weeks >= 9 && weeks <= 26);
            }
            _ => panic!("Expected Severe injury"),
        }
    }

    #[test]
    fn test_injury_to_notification() {
        let event = RandomEvent::Injury {
            player_id: "player123".to_string(),
            player_name: "John Doe".to_string(),
            injury_type: InjuryType::Minor { weeks: 2 },
        };

        let notification = event.to_notification();

        assert_eq!(notification.notification_type, NotificationType::Injury);
        assert_eq!(notification.priority, NotificationPriority::Normal);
        assert!(notification.message.contains("John Doe"));
        assert!(notification.message.contains("2 week"));
        assert!(!notification.read);
    }

    #[test]
    fn test_transfer_offer_to_notification() {
        let event = RandomEvent::TransferOffer {
            player_id: "player123".to_string(),
            player_name: "Jane Smith".to_string(),
            from_team: "Rival FC".to_string(),
            offer_amount: 5_000_000,
        };

        let notification = event.to_notification();

        assert_eq!(notification.notification_type, NotificationType::Transfer);
        assert_eq!(notification.priority, NotificationPriority::High);
        assert!(notification.message.contains("Jane Smith"));
        assert!(notification.message.contains("Rival FC"));
        assert!(notification.message.contains("5000000"));
    }

    #[test]
    fn test_media_story_to_notification() {
        let event = RandomEvent::MediaStory {
            headline: "Team in title race!".to_string(),
            impact: MediaImpact::Positive,
        };

        let notification = event.to_notification();

        assert_eq!(notification.notification_type, NotificationType::News);
        assert_eq!(notification.priority, NotificationPriority::Normal);
        assert_eq!(notification.message, "Team in title race!");
    }

    #[test]
    fn test_contract_expiry_to_notification() {
        let event = RandomEvent::ContractExpiry {
            player_id: "player123".to_string(),
            player_name: "Alex Johnson".to_string(),
        };

        let notification = event.to_notification();

        assert_eq!(notification.notification_type, NotificationType::Contract);
        assert_eq!(notification.priority, NotificationPriority::High);
        assert!(notification.message.contains("Alex Johnson"));
        assert!(notification.message.contains("contract"));
    }

    #[test]
    fn test_severe_injury_priority() {
        let severe = RandomEvent::Injury {
            player_id: "player123".to_string(),
            player_name: "Star Player".to_string(),
            injury_type: InjuryType::Severe { weeks: 20 },
        };

        let notification = severe.to_notification();
        assert_eq!(notification.priority, NotificationPriority::Urgent);
    }

    #[test]
    fn test_career_ending_injury() {
        let career_ending = RandomEvent::Injury {
            player_id: "player123".to_string(),
            player_name: "Veteran Player".to_string(),
            injury_type: InjuryType::CareerEnding,
        };

        let notification = career_ending.to_notification();
        assert_eq!(notification.priority, NotificationPriority::Urgent);
        assert!(notification.message.contains("career-ending"));
    }

    // Tests for event generators

    #[test]
    fn test_generate_injury_event_probability() {
        let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);

        // Run many times to check probability distribution
        let mut injury_count = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if let Some(event) = generate_injury_event(&player) {
                injury_count += 1;
                match event {
                    RandomEvent::Injury { player_id, .. } => {
                        assert_eq!(player_id, "player1");
                    }
                    _ => panic!("Expected Injury event"),
                }
            }
        }

        // Should be approximately 5% (allow 3-7% range for randomness)
        let percentage = (injury_count as f64 / iterations as f64) * 100.0;
        assert!(percentage >= 3.0 && percentage <= 7.0);
    }

    #[test]
    fn test_generate_injury_event_distribution() {
        let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);

        let mut career_ending = 0;
        let mut severe = 0;
        let mut moderate = 0;
        let mut minor = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            // Force injury by checking all random events
            if let Some(RandomEvent::Injury { injury_type, .. }) = generate_random_event(&player, "Test Team") {
                match injury_type {
                    InjuryType::CareerEnding => career_ending += 1,
                    InjuryType::Severe { .. } => severe += 1,
                    InjuryType::Moderate { .. } => moderate += 1,
                    InjuryType::Minor { .. } => minor += 1,
                }
            }
        }

        let total = career_ending + severe + moderate + minor;

        // Check distribution (allow reasonable tolerance for randomness)
        // Expected: 10% career-ending, 20% severe, 30% moderate, 40% minor
        let career_ending_pct = (career_ending as f64 / total as f64) * 100.0;
        let severe_pct = (severe as f64 / total as f64) * 100.0;
        let moderate_pct = (moderate as f64 / total as f64) * 100.0;
        let minor_pct = (minor as f64 / total as f64) * 100.0;

        // Wider tolerance for randomness
        assert!(career_ending_pct >= 4.0 && career_ending_pct <= 16.0);
        assert!(severe_pct >= 14.0 && severe_pct <= 26.0);
        assert!(moderate_pct >= 24.0 && moderate_pct <= 36.0);
        assert!(minor_pct >= 34.0 && minor_pct <= 46.0);
    }

    #[test]
    fn test_generate_transfer_offer_low_ability() {
        // Player with ability < 100 should never get offers
        let mut player = Player::new("player1".to_string(), "Weak Player".to_string(), Position::ST);
        player.current_ability = 50;
        player.market_value = 10000;

        // Try many times
        for _ in 0..1000 {
            assert!(generate_transfer_offer(&player).is_none());
        }
    }

    #[test]
    fn test_generate_transfer_offer_high_ability() {
        // Player with ability >= 100 can get offers
        let mut player = Player::new("player1".to_string(), "Star Player".to_string(), Position::ST);
        player.current_ability = 150;
        player.market_value = 10_000_000;

        // Check that offers can be generated
        let mut got_offer = false;
        for _ in 0..1000 {
            if let Some(event) = generate_transfer_offer(&player) {
                got_offer = true;
                match event {
                    RandomEvent::TransferOffer { player_id, player_name, from_team, offer_amount } => {
                        assert_eq!(player_id, "player1");
                        assert_eq!(player_name, "Star Player");
                        assert!(!from_team.is_empty());

                        // Check offer is within 80-120% of market value
                        let min_offer = player.market_value as f64 * 0.8;
                        let max_offer = player.market_value as f64 * 1.2;
                        assert!(offer_amount as f64 >= min_offer);
                        assert!(offer_amount as f64 <= max_offer);
                    }
                    _ => panic!("Expected TransferOffer event"),
                }
                break;
            }
        }

        assert!(got_offer, "Should get at least one offer in 1000 attempts");
    }

    #[test]
    fn test_generate_transfer_offer_probability() {
        let mut player = Player::new("player1".to_string(), "Star Player".to_string(), Position::ST);
        player.current_ability = 150;
        player.market_value = 5_000_000;

        let mut offer_count = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if generate_transfer_offer(&player).is_some() {
                offer_count += 1;
            }
        }

        // Should be approximately 2% (allow 1-3% range for randomness)
        let percentage = (offer_count as f64 / iterations as f64) * 100.0;
        assert!(percentage >= 1.0 && percentage <= 3.0);
    }

    #[test]
    fn test_generate_media_story_probability() {
        let team_name = "Test Team";

        let mut story_count = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if generate_media_story(team_name).is_some() {
                story_count += 1;
            }
        }

        // Should be approximately 10% (allow 8-12% range for randomness)
        let percentage = (story_count as f64 / iterations as f64) * 100.0;
        assert!(percentage >= 8.0 && percentage <= 12.0);
    }

    #[test]
    fn test_generate_media_story_content() {
        let team_name = "Awesome FC";

        // Generate many stories and check they're valid
        for _ in 0..1000 {
            if let Some(event) = generate_media_story(team_name) {
                match event {
                    RandomEvent::MediaStory { headline, impact } => {
                        assert!(headline.contains("Awesome FC") || headline.contains("Awesome"));
                        assert!(!headline.is_empty());

                        // Check impact is one of the three types
                        assert!(
                            impact == MediaImpact::Positive
                                || impact == MediaImpact::Negative
                                || impact == MediaImpact::Neutral
                        );
                    }
                    _ => panic!("Expected MediaStory event"),
                }
            }
        }
    }

    #[test]
    fn test_generate_media_story_impact_distribution() {
        let team_name = "Test Team";

        let mut positive = 0;
        let mut negative = 0;
        let mut neutral = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if let Some(RandomEvent::MediaStory { impact, .. }) = generate_media_story(team_name) {
                match impact {
                    MediaImpact::Positive => positive += 1,
                    MediaImpact::Negative => negative += 1,
                    MediaImpact::Neutral => neutral += 1,
                }
            }
        }

        let total = positive + negative + neutral;
        assert!(total > 0, "Should have generated some stories");

        // Check distribution (allow 25-45% tolerance for randomness)
        // Expected: 40% positive, 30% negative, 30% neutral
        let positive_pct = (positive as f64 / total as f64) * 100.0;
        let negative_pct = (negative as f64 / total as f64) * 100.0;
        let neutral_pct = (neutral as f64 / total as f64) * 100.0;

        assert!(positive_pct >= 30.0 && positive_pct <= 50.0);
        assert!(negative_pct >= 20.0 && negative_pct <= 40.0);
        assert!(neutral_pct >= 20.0 && neutral_pct <= 40.0);
    }

    #[test]
    fn test_generate_random_event_types() {
        let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);
        let team_name = "Test Team";

        let mut injury = 0;
        let mut transfer = 0;
        let mut media = 0;
        let iterations = 10000;

        for _ in 0..iterations {
            if let Some(event) = generate_random_event(&player, team_name) {
                match event {
                    RandomEvent::Injury { .. } => injury += 1,
                    RandomEvent::TransferOffer { .. } => transfer += 1,
                    RandomEvent::MediaStory { .. } => media += 1,
                    _ => {}
                }
            }
        }

        let total = injury + transfer + media;
        assert!(total > 0, "Should have generated some events");

        // Check distribution is roughly balanced (allow wider range due to probability differences)
        // Transfer offers are rarer (2% probability vs 5% injury vs 10% media)
        let injury_pct = (injury as f64 / total as f64) * 100.0;
        let transfer_pct = (transfer as f64 / total as f64) * 100.0;
        let media_pct = (media as f64 / total as f64) * 100.0;

        // More lenient ranges since transfer offers are much rarer
        assert!(injury_pct >= 10.0 && injury_pct <= 60.0);
        assert!(transfer_pct >= 0.0 && transfer_pct <= 30.0);
        assert!(media_pct >= 30.0 && media_pct <= 80.0);
    }

    #[test]
    fn test_injury_weeks_in_range() {
        let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);

        // Generate injuries and check weeks are in valid ranges
        for _ in 0..1000 {
            if let Some(RandomEvent::Injury { injury_type, .. }) = generate_random_event(&player, "Test") {
                match injury_type {
                    InjuryType::Minor { weeks } => {
                        assert!(weeks >= 1 && weeks <= 3);
                    }
                    InjuryType::Moderate { weeks } => {
                        assert!(weeks >= 4 && weeks <= 8);
                    }
                    InjuryType::Severe { weeks } => {
                        assert!(weeks >= 9 && weeks <= 26);
                    }
                    InjuryType::CareerEnding => {
                        // No weeks to check
                    }
                }
            }
        }
    }

    #[test]
    fn test_transfer_offer_big_teams() {
        let mut player = Player::new("player1".to_string(), "Star".to_string(), Position::ST);
        player.current_ability = 150;
        player.market_value = 10_000_000;

        // Known big teams
        let big_teams = vec![
            "Real Madrid", "Barcelona", "Manchester City", "Manchester United",
            "Liverpool", "Bayern Munich", "PSG", "Juventus",
            "AC Milan", "Inter Milan", "Chelsea", "Arsenal",
            "Tottenham Hotspur", "Borussia Dortmund", "Atletico Madrid",
        ];

        let mut found_offer = false;
        for _ in 0..1000 {
            if let Some(RandomEvent::TransferOffer { from_team, .. }) = generate_transfer_offer(&player) {
                assert!(big_teams.contains(&from_team.as_str()));
                found_offer = true;
                break;
            }
        }

        assert!(found_offer);
    }
}
