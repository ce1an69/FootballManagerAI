use crate::game::{GameState, Notification, NotificationType, NotificationPriority};
use uuid::Uuid;

/// NotificationManager - Factory methods for creating game notifications
///
/// This struct provides convenient factory methods for creating different
/// types of notifications with appropriate titles, messages, and priorities.
pub struct NotificationManager;

impl NotificationManager {
    /// Create an injury notification
    ///
    /// # Arguments
    /// * `player_name` - Name of the injured player
    /// * `duration_weeks` - Expected recovery time in weeks
    pub fn notify_injury(player_name: &str, duration_weeks: u8) -> Notification {
        let title = format!("球员受伤: {}", player_name);
        let message = format!("{} 预计缺阵 {} 周", player_name, duration_weeks);

        Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type: NotificationType::Injury,
            priority: NotificationPriority::High,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a transfer offer notification
    ///
    /// # Arguments
    /// * `player_name` - Name of the player offered for
    /// * `from_team` - Name of the team making the offer
    /// * `amount` - Transfer fee offered
    pub fn notify_transfer_offer(player_name: &str, from_team: &str, amount: u32) -> Notification {
        let title = format!("转会报价: {}", player_name);
        let message = format!("{} 出价 {} 购买 {}", from_team, amount, player_name);

        Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type: NotificationType::Transfer,
            priority: NotificationPriority::Normal,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a transfer completed notification
    ///
    /// # Arguments
    /// * `player_name` - Name of the transferred player
    /// * `to_team` - Name of the destination team
    /// * `fee` - Transfer fee
    pub fn notify_transfer_completed(player_name: &str, to_team: &str, fee: u32) -> Notification {
        let title = format!("转会完成: {}", player_name);
        let message = format!("{} 已以 {} 的价格转会至 {}", player_name, fee, to_team);

        Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type: NotificationType::Transfer,
            priority: NotificationPriority::Normal,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a contract expiring notification
    ///
    /// # Arguments
    /// * `player_name` - Name of the player
    /// * `months_left` - Months remaining on contract
    pub fn notify_contract_expiring(player_name: &str, months_left: u8) -> Notification {
        let title = format!("合同即将到期: {}", player_name);
        let message = format!("{} 的合同将在 {} 个月后到期", player_name, months_left);

        let priority = if months_left <= 1 {
            NotificationPriority::Urgent
        } else if months_left <= 3 {
            NotificationPriority::High
        } else {
            NotificationPriority::Normal
        };

        Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type: NotificationType::Contract,
            priority,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a match result notification
    ///
    /// # Arguments
    /// * `result` - Match result description (e.g., "胜 2-1", "负 1-3")
    /// * `opponent` - Name of the opponent team
    pub fn notify_match_result(result: &str, opponent: &str) -> Notification {
        let title = format!("比赛结果: {}", result);
        let message = format!("对阵 {} 的比赛已结束", opponent);

        Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type: NotificationType::Match,
            priority: NotificationPriority::Normal,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create an achievement notification
    ///
    /// # Arguments
    /// * `title` - Achievement title
    /// * `description` - Achievement description
    pub fn notify_achievement(title: &str, description: &str) -> Notification {
        Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: description.to_string(),
            notification_type: NotificationType::Achievement,
            priority: NotificationPriority::High,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a finance notification
    ///
    /// # Arguments
    /// * `title` - Transaction title
    /// * `amount` - Amount (positive for income, negative for expense)
    pub fn notify_finance(title: &str, amount: i64) -> Notification {
        let message = if amount >= 0 {
            format!("收入: +{}", amount)
        } else {
            format!("支出: {}", amount)
        };

        Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message,
            notification_type: NotificationType::Finance,
            priority: NotificationPriority::Normal,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a board confidence notification
    ///
    /// # Arguments
    /// * `level` - Board confidence level (0-100)
    pub fn notify_board_confidence(level: u8) -> Notification {
        let (title, description) = if level >= 80 {
            ("董事会信心极高", "董事会对你的工作非常满意")
        } else if level >= 60 {
            ("董事会信心良好", "董事会认可你的工作")
        } else if level >= 40 {
            ("董事会信心一般", "董事会认为你需要改进")
        } else {
            ("董事会信心低", "董事会对你的工作表示担忧")
        };

        Notification {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            message: description.to_string(),
            notification_type: NotificationType::News,
            priority: if level < 40 {
                NotificationPriority::High
            } else {
                NotificationPriority::Normal
            },
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Create a media story notification
    ///
    /// # Arguments
    /// * `headline` - News headline
    pub fn notify_media_story(headline: &str) -> Notification {
        Notification {
            id: Uuid::new_v4().to_string(),
            title: "媒体报道".to_string(),
            message: headline.to_string(),
            notification_type: NotificationType::News,
            priority: NotificationPriority::Low,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_injury() {
        let notification = NotificationManager::notify_injury("张三", 4);

        assert_eq!(notification.notification_type, NotificationType::Injury);
        assert_eq!(notification.priority, NotificationPriority::High);
        assert!(notification.title.contains("张三"));
        assert!(notification.message.contains("4 周"));
    }

    #[test]
    fn test_notify_transfer_offer() {
        let notification = NotificationManager::notify_transfer_offer("李四", "曼联", 5000000);

        assert_eq!(notification.notification_type, NotificationType::Transfer);
        assert!(notification.title.contains("李四"));
        assert!(notification.message.contains("曼联"));
        assert!(notification.message.contains("5000000"));
    }

    #[test]
    fn test_notify_contract_expiring_urgent() {
        let notification = NotificationManager::notify_contract_expiring("王五", 1);

        assert_eq!(notification.notification_type, NotificationType::Contract);
        assert_eq!(notification.priority, NotificationPriority::Urgent);
    }

    #[test]
    fn test_notify_contract_expiring_normal() {
        let notification = NotificationManager::notify_contract_expiring("王五", 6);

        assert_eq!(notification.notification_type, NotificationType::Contract);
        assert_eq!(notification.priority, NotificationPriority::Normal);
    }

    #[test]
    fn test_notify_achievement() {
        let notification = NotificationManager::notify_achievement(
            "联赛冠军",
            "恭喜你赢得了联赛冠军！"
        );

        assert_eq!(notification.notification_type, NotificationType::Achievement);
        assert_eq!(notification.priority, NotificationPriority::High);
        assert_eq!(notification.title, "联赛冠军");
    }

    #[test]
    fn test_notify_finance_income() {
        let notification = NotificationManager::notify_finance("门票收入", 100000);

        assert_eq!(notification.notification_type, NotificationType::Finance);
        assert!(notification.message.contains("+100000"));
    }

    #[test]
    fn test_notify_finance_expense() {
        let notification = NotificationManager::notify_finance("转会支出", -5000000);

        assert_eq!(notification.notification_type, NotificationType::Finance);
        assert!(notification.message.contains("5000000"));
    }

    #[test]
    fn test_notify_board_confidence_high() {
        let notification = NotificationManager::notify_board_confidence(85);

        assert_eq!(notification.notification_type, NotificationType::News);
        assert!(notification.title.contains("极高"));
    }

    #[test]
    fn test_notify_board_confidence_low() {
        let notification = NotificationManager::notify_board_confidence(30);

        assert_eq!(notification.notification_type, NotificationType::News);
        assert_eq!(notification.priority, NotificationPriority::High);
        assert!(notification.title.contains("低"));
    }
}
