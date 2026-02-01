use crate::team::Position;
use serde::{Deserialize, Serialize};

/// Main tactic structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactic {
    pub formation: Formation,
    pub attacking_mentality: u8,  // 0-100
    pub defensive_height: DefensiveHeight,
    pub passing_style: PassingStyle,
    pub tempo: Tempo,
    pub player_roles: Vec<PlayerRoleAssignment>,
}

impl Default for Tactic {
    fn default() -> Self {
        Self {
            formation: Formation::FourFourTwo,
            attacking_mentality: 50,
            defensive_height: DefensiveHeight::Medium,
            passing_style: PassingStyle::Mixed,
            tempo: Tempo::Medium,
            player_roles: Vec::new(),
        }
    }
}

impl Tactic {
    /// Get tactical style description
    pub fn style_description(&self) -> &str {
        if matches!(self.defensive_height, DefensiveHeight::High) && self.attacking_mentality > 70 {
            "高位逼抢"
        } else if matches!(self.defensive_height, DefensiveHeight::Low)
            && matches!(self.tempo, Tempo::Fast) {
            "防守反击"
        } else if matches!(self.passing_style, PassingStyle::Short) {
            "控球为主"
        } else if matches!(self.passing_style, PassingStyle::Long) {
            "长传冲吊"
        } else {
            "平衡战术"
        }
    }

    /// Get tactical intensity (0-100)
    pub fn intensity(&self) -> u8 {
        let press_intensity = match self.defensive_height {
            DefensiveHeight::High => 30,
            DefensiveHeight::Medium => 20,
            DefensiveHeight::Low => 10,
        };
        let tempo_intensity = match self.tempo {
            Tempo::Fast => 30,
            Tempo::Medium => 20,
            Tempo::Slow => 10,
        };
        let mentality_intensity = self.attacking_mentality / 3;

        (press_intensity + tempo_intensity + mentality_intensity).min(100)
    }

    /// Validate tactic setup
    pub fn validate(&self) -> bool {
        self.player_roles.len() == 11
    }
}

/// Formation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Formation {
    FourFourTwo,
    FourThreeThree,
    FourTwoThreeOne,
    FourOneFourOne,
    FourFiveOne,
    ThreeFiveTwo,
    ThreeFourTwoOne,
    FiveThreeTwo,
    FiveTwoTwoOne,
}

impl Formation {
    /// Get positions for this formation
    pub fn positions(&self) -> Vec<Position> {
        match self {
            Formation::FourFourTwo => vec![
                Position::GK,
                Position::LB, Position::CB, Position::CB, Position::RB,
                Position::LW, Position::CM, Position::CM, Position::RW,
                Position::ST, Position::ST,
            ],
            Formation::FourThreeThree => vec![
                Position::GK,
                Position::LB, Position::CB, Position::CB, Position::RB,
                Position::CM, Position::CM, Position::CM,
                Position::LW, Position::ST, Position::RW,
            ],
            Formation::FourTwoThreeOne => vec![
                Position::GK,
                Position::LB, Position::CB, Position::CB, Position::RB,
                Position::DM, Position::DM,
                Position::LW, Position::AM, Position::RW,
                Position::ST,
            ],
            Formation::FourOneFourOne => vec![
                Position::GK,
                Position::LB, Position::CB, Position::CB, Position::RB,
                Position::DM,
                Position::LW, Position::CM, Position::CM, Position::RW,
                Position::ST,
            ],
            Formation::FourFiveOne => vec![
                Position::GK,
                Position::LB, Position::CB, Position::CB, Position::RB,
                Position::DM, Position::CM, Position::CM, Position::LW, Position::RW,
                Position::ST,
            ],
            Formation::ThreeFiveTwo => vec![
                Position::GK,
                Position::CB, Position::CB, Position::CB,
                Position::WB, Position::DM, Position::CM, Position::DM, Position::WB,
                Position::ST, Position::ST,
            ],
            Formation::ThreeFourTwoOne => vec![
                Position::GK,
                Position::CB, Position::CB, Position::CB,
                Position::LW, Position::CM, Position::CM, Position::RW,
                Position::AM, Position::AM,
                Position::ST,
            ],
            Formation::FiveThreeTwo => vec![
                Position::GK,
                Position::WB, Position::CB, Position::CB, Position::CB, Position::WB,
                Position::DM, Position::CM, Position::DM,
                Position::ST, Position::ST,
            ],
            Formation::FiveTwoTwoOne => vec![
                Position::GK,
                Position::WB, Position::CB, Position::CB, Position::CB, Position::WB,
                Position::DM, Position::DM,
                Position::AM, Position::AM,
                Position::ST,
            ],
        }
    }

    /// Get formation name
    pub fn name(&self) -> &str {
        match self {
            Formation::FourFourTwo => "4-4-2",
            Formation::FourThreeThree => "4-3-3",
            Formation::FourTwoThreeOne => "4-2-3-1",
            Formation::FourOneFourOne => "4-1-4-1",
            Formation::FourFiveOne => "4-5-1",
            Formation::ThreeFiveTwo => "3-5-2",
            Formation::ThreeFourTwoOne => "3-4-2-1",
            Formation::FiveThreeTwo => "5-3-2",
            Formation::FiveTwoTwoOne => "5-2-2-1",
        }
    }
}

/// Defensive line height
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DefensiveHeight {
    Low,
    Medium,
    High,
}

/// Passing style
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PassingStyle {
    Short,
    Mixed,
    Long,
}

/// Tempo of play
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tempo {
    Slow,
    Medium,
    Fast,
}

/// Player roles (FM-style)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerRole {
    // Forward roles
    AdvancedForward,
    CompleteForward,
    DeepLyingForward,
    TargetMan,
    Poacher,
    FalseNine,
    Trequartista,

    // Midfielder roles
    BoxToBox,
    CentralMidfielder,
    BallWinningMidfielder,
    DefensiveMidfielder,
    DeepLyingPlaymaker,
    Regista,
    AdvancedPlaymaker,
    WideMidfielder,
    Winger,
    InsideForward,
    WideTargetMan,

    // Defender roles
    FullBack,
    WingBack,
    CompleteWingBack,
    InvertedWingBack,
    CentralDefender,
    BallPlayingDefender,
    Libero,
    NoNonsenseDefender,
    WideCentreBack,

    // Goalkeeper roles
    Goalkeeper,
    SweeperKeeper,
}

/// Player duty
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Duty {
    Attack,
    Support,
    Defend,
    Stopper,
    Cover,
}

/// Player role assignment in tactic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRoleAssignment {
    pub position_index: usize,
    pub role: PlayerRole,
    pub duty: Duty,
}

impl PlayerRole {
    /// Get required attributes for this role
    pub fn get_required_attributes(&self) -> Vec<&'static str> {
        match self {
            // Forward roles
            PlayerRole::AdvancedForward => vec!["pace", "finishing", "off_the_ball", "work_rate"],
            PlayerRole::CompleteForward => vec!["finishing", "heading", "technique", "strength", "vision"],
            PlayerRole::DeepLyingForward => vec!["passing", "vision", "technique", "off_the_ball"],
            PlayerRole::TargetMan => vec!["heading", "strength", "bravery", "off_the_ball"],
            PlayerRole::Poacher => vec!["finishing", "off_the_ball", "pace", "anticipation"],
            PlayerRole::FalseNine => vec!["passing", "vision", "technique", "off_the_ball", "creativity"],
            PlayerRole::Trequartista => vec!["technique", "vision", "creativity", "passing", "dribbling"],

            // Midfielder roles
            PlayerRole::BoxToBox => vec!["stamina", "work_rate", "passing", "decisions", "teamwork"],
            PlayerRole::CentralMidfielder => vec!["passing", "teamwork", "decisions", "stamina"],
            PlayerRole::BallWinningMidfielder => vec!["tackling", "work_rate", "stamina", "strength", "aggression"],
            PlayerRole::DefensiveMidfielder => vec!["tackling", "positioning", "decisions", "work_rate"],
            PlayerRole::DeepLyingPlaymaker => vec!["passing", "vision", "technique", "decisions"],
            PlayerRole::Regista => vec!["passing", "vision", "technique", "creativity"],
            PlayerRole::AdvancedPlaymaker => vec!["passing", "vision", "technique", "creativity", "decisions"],
            PlayerRole::WideMidfielder => vec!["passing", "teamwork", "stamina", "work_rate"],
            PlayerRole::Winger => vec!["pace", "dribbling", "crossing", "off_the_ball", "technique"],
            PlayerRole::InsideForward => vec!["dribbling", "finishing", "off_the_ball", "pace"],
            PlayerRole::WideTargetMan => vec!["heading", "strength", "off_the_ball", "crossing"],

            // Defender roles
            PlayerRole::FullBack => vec!["stamina", "work_rate", "tackling", "positioning"],
            PlayerRole::WingBack => vec!["stamina", "pace", "crossing", "work_rate", "tackling"],
            PlayerRole::CompleteWingBack => vec!["pace", "stamina", "crossing", "dribbling", "work_rate", "tackling"],
            PlayerRole::InvertedWingBack => vec!["passing", "vision", "technique", "dribbling"],
            PlayerRole::CentralDefender => vec!["tackling", "marking", "positioning", "heading", "strength"],
            PlayerRole::BallPlayingDefender => vec!["passing", "technique", "tackling", "positioning", "vision"],
            PlayerRole::Libero => vec!["positioning", "passing", "vision", "technique", "tackling"],
            PlayerRole::NoNonsenseDefender => vec!["tackling", "marking", "bravery", "strength", "positioning"],
            PlayerRole::WideCentreBack => vec!["tackling", "marking", "positioning", "strength"],

            // Goalkeeper roles
            PlayerRole::Goalkeeper => vec!["handling", "reflexes", "positioning", "aerial_reach"],
            PlayerRole::SweeperKeeper => vec!["reflexes", "positioning", "pace", "throwing", "kicking"],
        }
    }

    /// Get role description
    pub fn get_description(&self) -> &str {
        match self {
            PlayerRole::AdvancedForward => "攻击性强，善于跑位的前锋",
            PlayerRole::CompleteForward => "全面的前锋，能传能射",
            PlayerRole::DeepLyingForward => "回撤较深的前锋",
            PlayerRole::TargetMan => "高中锋，善于争顶",
            PlayerRole::Poacher => "禁区射手，善于把握机会",
            PlayerRole::FalseNine => "伪九号，经常回撤中场",
            PlayerRole::Trequartista => "自由人，组织核心",

            PlayerRole::BoxToBox => "全能中场，攻防兼备",
            PlayerRole::CentralMidfielder => "标准中场",
            PlayerRole::BallWinningMidfielder => "抢球手，善于抢断",
            PlayerRole::DefensiveMidfielder => "防守型中场",
            PlayerRole::DeepLyingPlaymaker => "深度组织核心",
            PlayerRole::Regista => "拖后组织核心，更前卫",
            PlayerRole::AdvancedPlaymaker => "前场组织核心",
            PlayerRole::WideMidfielder => "边路中场",
            PlayerRole::Winger => "边锋，善于突破传中",
            PlayerRole::InsideForward => "内锋，切入内线射门",
            PlayerRole::WideTargetMan => "边路高中锋",

            PlayerRole::FullBack => "标准边后卫",
            PlayerRole::WingBack => "翼卫，攻守兼备",
            PlayerRole::CompleteWingBack => "全能翼卫",
            PlayerRole::InvertedWingBack => "内切翼卫",
            PlayerRole::CentralDefender => "标准中后卫",
            PlayerRole::BallPlayingDefender => "出球中后卫",
            PlayerRole::Libero => "自由人，拖后中卫",
            PlayerRole::NoNonsenseDefender => "简练中卫，专注防守",
            PlayerRole::WideCentreBack => "三中卫体系中的边中卫",

            PlayerRole::Goalkeeper => "标准门将",
            PlayerRole::SweeperKeeper => "清道夫门将",
        }
    }

    /// Check if role is suitable for position
    pub fn is_suitable_for_position(&self, position: &Position) -> bool {
        match self {
            PlayerRole::AdvancedForward | PlayerRole::CompleteForward |
            PlayerRole::DeepLyingForward | PlayerRole::TargetMan |
            PlayerRole::Poacher | PlayerRole::FalseNine | PlayerRole::Trequartista => {
                matches!(position, Position::ST | Position::CF)
            }

            PlayerRole::BoxToBox | PlayerRole::CentralMidfielder |
            PlayerRole::BallWinningMidfielder | PlayerRole::DefensiveMidfielder |
            PlayerRole::DeepLyingPlaymaker | PlayerRole::Regista |
            PlayerRole::AdvancedPlaymaker => {
                matches!(position, Position::CM | Position::DM | Position::AM)
            }

            PlayerRole::WideMidfielder | PlayerRole::Winger |
            PlayerRole::InsideForward | PlayerRole::WideTargetMan => {
                matches!(position, Position::LW | Position::RW)
            }

            PlayerRole::FullBack | PlayerRole::WingBack | PlayerRole::CompleteWingBack |
            PlayerRole::InvertedWingBack => {
                matches!(position, Position::LB | Position::RB | Position::WB)
            }

            PlayerRole::CentralDefender | PlayerRole::BallPlayingDefender |
            PlayerRole::Libero | PlayerRole::NoNonsenseDefender |
            PlayerRole::WideCentreBack => {
                matches!(position, Position::CB)
            }

            PlayerRole::Goalkeeper | PlayerRole::SweeperKeeper => {
                matches!(position, Position::GK)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formation_positions() {
        let positions = Formation::FourFourTwo.positions();
        assert_eq!(positions.len(), 11);
        assert_eq!(positions[0], Position::GK);
    }

    #[test]
    fn test_formation_names() {
        assert_eq!(Formation::FourFourTwo.name(), "4-4-2");
        assert_eq!(Formation::FourThreeThree.name(), "4-3-3");
    }

    #[test]
    fn test_tactic_description() {
        let tactic = Tactic::default();
        assert!(tactic.style_description().len() > 0);
    }

    #[test]
    fn test_tactic_intensity() {
        let tactic = Tactic::default();
        assert!(tactic.intensity() <= 100);
    }

    #[test]
    fn test_role_required_attributes() {
        let attrs = PlayerRole::AdvancedForward.get_required_attributes();
        assert!(attrs.contains(&"pace"));
        assert!(attrs.contains(&"finishing"));
    }

    #[test]
    fn test_role_suitable_for_position() {
        assert!(PlayerRole::AdvancedForward.is_suitable_for_position(&Position::ST));
        assert!(!PlayerRole::AdvancedForward.is_suitable_for_position(&Position::GK));
    }
}
