use crate::game::{Notification, NotificationType, NotificationPriority};
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
            RandomEvent::Injury { player_id, player_name, injury_type } => {
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

            RandomEvent::TransferOffer { player_id, player_name, from_team, offer_amount } => {
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

            RandomEvent::ContractExpiry { player_id, player_name } => {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
