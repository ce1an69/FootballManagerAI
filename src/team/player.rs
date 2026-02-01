use serde::{Deserialize, Serialize};

/// Player position on the field
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Position {
    GK,
    CB,
    LB,
    RB,
    WB,
    DM,
    CM,
    AM,
    LW,
    RW,
    ST,
    CF,
}

/// Player's preferred foot
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Foot {
    Left,
    Right,
    Both,
}

/// Player's current status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerStatus {
    Healthy,
    Injured,
    Fatigued,
    Suspended,
}

/// Complete player data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    // Identity
    pub id: String,
    pub team_id: Option<String>,

    // Basic info
    pub name: String,
    pub age: u8,
    pub nationality: String,
    pub position: Position,
    pub second_positions: Vec<Position>,
    pub preferred_foot: Foot,
    pub height: u8,  // cm
    pub weight: u8,  // kg

    // Technical attributes (0-200)
    pub corners: u16,
    pub crossing: u16,
    pub dribbling: u16,
    pub finishing: u16,
    pub heading: u16,
    pub long_shots: u16,
    pub long_throws: u16,
    pub marking: u16,
    pub passing: u16,
    pub penalties: u16,
    pub tackling: u16,
    pub technique: u16,

    // Mental attributes (0-200)
    pub aggression: u16,
    pub anticipation: u16,
    pub bravery: u16,
    pub creativity: u16,
    pub decisions: u16,
    pub concentration: u16,
    pub positioning: u16,
    pub off_the_ball: u16,
    pub work_rate: u16,
    pub pressure: u16,
    pub teamwork: u16,
    pub vision: u16,

    // Physical attributes (0-200)
    pub acceleration: u16,
    pub agility: u16,
    pub balance: u16,
    pub pace: u16,
    pub stamina: u16,
    pub strength: u16,

    // Goalkeeper attributes (0-200)
    pub aerial_reach: u16,
    pub command_of_area: u16,
    pub communication: u16,
    pub eccentricity: u16,
    pub handling: u16,
    pub kicking: u16,
    pub throwing: u16,
    pub reflexes: u16,
    pub rushing_out: u16,
    pub gk_positioning: u16,

    // Hidden attributes
    pub potential_ability: u16,
    pub current_ability: u16,
    pub adaptability: u16,
    pub ambition: u16,
    pub professionalism: u16,
    pub loyalty: u16,
    pub injury_proneness: u16,
    pub controversy: u16,

    // Status
    pub match_fitness: u8,      // 0-100
    pub morale: u8,             // 0-100
    pub status: PlayerStatus,
    pub injury_days: Option<u8>,
    pub fatigue: u8,            // 0-100

    // Contract
    pub wage: u32,              // Weekly wage
    pub contract_years: u8,
    pub market_value: u32,
}

impl Player {
    /// Create a new player with default attributes
    pub fn new(id: String, name: String, position: Position) -> Self {
        Self {
            id,
            team_id: None,
            name,
            age: 16,
            nationality: "Unknown".to_string(),
            position,
            second_positions: Vec::new(),
            preferred_foot: Foot::Right,
            height: 175,
            weight: 70,

            // Default technical attributes
            corners: 50,
            crossing: 50,
            dribbling: 50,
            finishing: 50,
            heading: 50,
            long_shots: 50,
            long_throws: 50,
            marking: 50,
            passing: 50,
            penalties: 50,
            tackling: 50,
            technique: 50,

            // Default mental attributes
            aggression: 50,
            anticipation: 50,
            bravery: 50,
            creativity: 50,
            decisions: 50,
            concentration: 50,
            positioning: 50,
            off_the_ball: 50,
            work_rate: 50,
            pressure: 50,
            teamwork: 50,
            vision: 50,

            // Default physical attributes
            acceleration: 50,
            agility: 50,
            balance: 50,
            pace: 50,
            stamina: 50,
            strength: 50,

            // Default goalkeeper attributes
            aerial_reach: 50,
            command_of_area: 50,
            communication: 50,
            eccentricity: 50,
            handling: 50,
            kicking: 50,
            throwing: 50,
            reflexes: 50,
            rushing_out: 50,
            gk_positioning: 50,

            // Hidden attributes
            potential_ability: 100,
            current_ability: 100,
            adaptability: 50,
            ambition: 50,
            professionalism: 50,
            loyalty: 50,
            injury_proneness: 50,
            controversy: 50,

            // Status
            match_fitness: 100,
            morale: 50,
            status: PlayerStatus::Healthy,
            injury_days: None,
            fatigue: 0,

            // Contract
            wage: 1000,
            contract_years: 3,
            market_value: 50000,
        }
    }

    /// Age the player by one year and update abilities
    pub fn age_player(&mut self) {
        self.age += 1;

        // Young players (16-23) develop
        if self.age <= 23 && self.current_ability < self.potential_ability {
            let growth = ((self.potential_ability - self.current_ability) as f32 * 0.1) as u16;
            self.current_ability = (self.current_ability + growth).min(self.potential_ability);
        }
        // Old players (30+) decline
        else if self.age >= 30 {
            let decline = ((32 - self.age.min(36)) as f32 * 0.5) as u16;
            self.current_ability = self.current_ability.saturating_sub(decline);
        }

        // Update contract
        if self.contract_years > 0 {
            self.contract_years -= 1;
        }
    }

    /// Recover fatigue by given amount
    pub fn recover_fatigue(&mut self, amount: u8) {
        self.fatigue = self.fatigue.saturating_sub(amount);
        if self.fatigue == 0 && self.status == PlayerStatus::Fatigued {
            self.status = PlayerStatus::Healthy;
        }
    }

    /// Injure the player for given days
    pub fn injure(&mut self, days: u8) {
        self.status = PlayerStatus::Injured;
        self.injury_days = Some(days);
        self.match_fitness = self.match_fitness.saturating_sub(20);
    }

    /// Heal the player
    pub fn heal(&mut self) {
        self.status = PlayerStatus::Healthy;
        self.injury_days = None;
    }

    /// Calculate overall ability (simplified average of key attributes)
    pub fn calculate_overall_ability(&self) -> u16 {
        if self.is_gk() {
            // GK attributes
            let gk_attrs = [
                self.handling, self.reflexes, self.gk_positioning,
                self.aerial_reach, self.command_of_area, self.communication,
                self.kicking, self.throwing, self.rushing_out,
            ];
            gk_attrs.iter().sum::<u16>() / gk_attrs.len() as u16
        } else {
            // Outfield player - mix of technical, mental, physical
            let tech_attrs = [
                self.passing, self.dribbling, self.finishing,
                self.tackling, self.heading, self.technique,
            ];
            let mental_attrs = [
                self.decisions, self.positioning, self.off_the_ball,
                self.teamwork, self.work_rate, self.vision,
            ];
            let physical_attrs = [
                self.pace, self.strength, self.stamina,
                self.acceleration, self.agility,
            ];

            let tech_avg: u16 = tech_attrs.iter().sum::<u16>() / tech_attrs.len() as u16;
            let mental_avg: u16 = mental_attrs.iter().sum::<u16>() / mental_attrs.len() as u16;
            let physical_avg: u16 = physical_attrs.iter().sum::<u16>() / physical_attrs.len() as u16;

            (tech_avg + mental_avg + physical_avg) / 3
        }
    }

    /// Get player rating for specific position
    pub fn get_position_rating(&self, position: &Position) -> u16 {
        // If it's the player's natural position
        if &self.position == position {
            return self.calculate_overall_ability();
        }

        // If can play second position
        if self.second_positions.contains(position) {
            return (self.calculate_overall_ability() * 90) / 100;
        }

        // Check position compatibility and apply penalty
        let compatible = match (&self.position, position) {
            // GK is only compatible with GK
            (Position::GK, Position::GK) | (Position::GK, _) => false,

            // CB can play DM
            (Position::CB, Position::DM) => true,

            // LB/RB/WB are somewhat compatible
            (Position::LB, Position::RB | Position::WB) |
            (Position::RB, Position::LB | Position::WB) |
            (Position::WB, Position::LB | Position::RB) => true,

            // CM/DM are compatible
            (Position::CM, Position::DM) |
            (Position::DM, Position::CM) => true,

            // AM/CM are compatible
            (Position::AM, Position::CM) |
            (Position::CM, Position::AM) => true,

            // LW/RW are compatible
            (Position::LW, Position::RW) |
            (Position::RW, Position::LW) => true,

            // ST/CF are compatible
            (Position::ST, Position::CF) |
            (Position::CF, Position::ST) => true,

            // LW/RW can play ST/CF
            (Position::LW, Position::ST | Position::CF) |
            (Position::RW, Position::ST | Position::CF) => true,

            _ => false,
        };

        if compatible {
            (self.calculate_overall_ability() * 75) / 100
        } else {
            (self.calculate_overall_ability() * 50) / 100
        }
    }

    /// Check if player is a goalkeeper
    pub fn is_gk(&self) -> bool {
        self.position == Position::GK
    }

    /// Check if player can play given position
    pub fn can_play_position(&self, position: &Position) -> bool {
        &self.position == position || self.second_positions.contains(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        assert_eq!(player.name, "Test");
        assert_eq!(player.position, Position::ST);
        assert_eq!(player.age, 16);
    }

    #[test]
    fn test_age_player() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        player.age_player();
        assert_eq!(player.age, 17);
    }

    #[test]
    fn test_fatigue_recovery() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        player.fatigue = 50;
        player.recover_fatigue(30);
        assert_eq!(player.fatigue, 20);
    }

    #[test]
    fn test_injury() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        player.injure(10);
        assert_eq!(player.status, PlayerStatus::Injured);
        assert_eq!(player.injury_days, Some(10));
    }

    #[test]
    fn test_heal() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        player.injure(10);
        player.heal();
        assert_eq!(player.status, PlayerStatus::Healthy);
        assert_eq!(player.injury_days, None);
    }

    #[test]
    fn test_is_gk() {
        let gk = Player::new("1".to_string(), "GK".to_string(), Position::GK);
        assert!(gk.is_gk());

        let st = Player::new("2".to_string(), "ST".to_string(), Position::ST);
        assert!(!st.is_gk());
    }

    #[test]
    fn test_position_rating() {
        let player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        let rating = player.get_position_rating(&Position::ST);
        assert!(rating > 0 && rating <= 200);

        // Second position should have lower rating
        let rating_second = player.get_position_rating(&Position::CF);
        assert!(rating_second <= rating);

        // Incompatible position should have much lower rating
        let rating_gk = player.get_position_rating(&Position::GK);
        assert!(rating_gk < rating);
    }

    #[test]
    fn test_serialization() {
        let player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        let serialized = serde_json::to_string(&player).unwrap();
        let deserialized: Player = serde_json::from_str(&serialized).unwrap();
        assert_eq!(player.id, deserialized.id);
        assert_eq!(player.name, deserialized.name);
    }
}
