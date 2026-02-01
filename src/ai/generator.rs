use crate::team::{Player, Team, League, Position};
use rand::Rng;
use uuid::Uuid;

/// Generate a random player
pub fn generate_player(
    team_id: Option<String>,
    position: Position,
    skill_level: u16,
    age_range: std::ops::Range<u8>,
) -> Player {
    let mut rng = rand::thread_rng();
    let age = rng.gen_range(age_range);

    let mut player = Player::new(
        Uuid::new_v4().to_string(),
        generate_name(),
        position,
    );

    player.team_id = team_id;
    player.age = age;
    player.current_ability = skill_level;
    player.potential_ability = skill_level + rng.gen_range(0..30);

    // Position-specific attributes
    adjust_attributes_for_position(&mut player, skill_level);

    // Calculate market value and wage
    player.market_value = crate::team::calculate_market_value(&player);
    player.wage = crate::team::calculate_wage(&player);

    player
}

/// Generate a random name (simplified - returns "Player XXXX")
pub fn generate_name() -> String {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1000..9999);
    format!("Player {}", number)
}

/// Generate a team with players
pub fn generate_team(id: String, name: String, skill_level: u16) -> Team {
    let mut team = Team::new(id.clone(), name, "league1".to_string(), 10_000_000);

    // Generate squad
    for i in 0..25 {
        let position = match i {
            0 => Position::GK,
            1..=4 => Position::CB,
            5 => Position::LB,
            6 => Position::RB,
            7..=10 => Position::CM,
            11 => Position::LW,
            12 => Position::RW,
            13..=15 => Position::ST,
            _ => Position::CM, // Fill remaining with CM
        };

        let player = generate_player(Some(id.clone()), position, skill_level, 18..35);
        team.add_player(player.id.clone());
    }

    team
}

/// Generate a league with teams
pub fn generate_league(num_teams: usize, skill_level: u16) -> League {
    let mut teams = Vec::new();

    for i in 0..num_teams {
        let team = generate_team(
            format!("team-{}", i),
            format!("Team {}", i + 1),
            skill_level,
        );
        teams.push(team.id.clone());
    }

    let mut league = League::new(
        "league1".to_string(),
        "Premier League".to_string(),
        teams,
    );

    league.generate_schedule();
    league
}

/// Adjust player attributes based on position
fn adjust_attributes_for_position(player: &mut Player, skill_level: u16) {
    let mut rng = rand::thread_rng();
    let variance: u16 = 30;

    match player.position {
        Position::ST => {
            player.finishing = skill_level + rng.gen_range(0..variance);
            player.off_the_ball = skill_level + rng.gen_range(0..variance);
            player.pace = skill_level + rng.gen_range(0..variance);
            player.heading = skill_level + rng.gen_range(0..variance);
        }
        Position::CM => {
            player.passing = skill_level + rng.gen_range(0..variance);
            player.vision = skill_level + rng.gen_range(0..variance);
            player.stamina = skill_level + rng.gen_range(0..variance);
            player.teamwork = skill_level + rng.gen_range(0..variance);
        }
        Position::CB => {
            player.tackling = skill_level + rng.gen_range(0..variance);
            player.marking = skill_level + rng.gen_range(0..variance);
            player.positioning = skill_level + rng.gen_range(0..variance);
            player.strength = skill_level + rng.gen_range(0..variance);
        }
        Position::GK => {
            player.handling = skill_level + rng.gen_range(0..variance);
            player.reflexes = skill_level + rng.gen_range(0..variance);
            player.gk_positioning = skill_level + rng.gen_range(0..variance);
            player.aerial_reach = skill_level + rng.gen_range(0..variance);
        }
        _ => {
            // Generic attributes for other positions
            player.passing = skill_level + rng.gen_range(0..variance);
            player.pace = skill_level + rng.gen_range(0..variance);
            player.stamina = skill_level + rng.gen_range(0..variance);
        }
    }

    // Clamp all attributes to 0-200
    let clamp = |attr: &mut u16| {
        *attr = (*attr).min(200);
    };

    clamp(&mut player.finishing);
    clamp(&mut player.passing);
    clamp(&mut player.tackling);
    clamp(&mut player.pace);
    clamp(&mut player.stamina);
    clamp(&mut player.strength);
    clamp(&mut player.vision);
    clamp(&mut player.positioning);
    clamp(&mut player.handling);
    clamp(&mut player.reflexes);
    clamp(&mut player.gk_positioning);
    clamp(&mut player.off_the_ball);
    clamp(&mut player.heading);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_player() {
        let player = generate_player(Some("team1".to_string()), Position::ST, 100, 20..25);
        assert_eq!(player.position, Position::ST);
        assert!(player.age >= 20 && player.age <= 25);
        assert!(player.finishing > 50);
    }

    #[test]
    fn test_generate_name() {
        let name = generate_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_generate_team() {
        let team = generate_team("team1".to_string(), "Test Team".to_string(), 100);
        assert_eq!(team.name, "Test Team");
        assert_eq!(team.players.len(), 25);
    }

    #[test]
    fn test_generate_league() {
        let league = generate_league(20, 100);
        assert_eq!(league.teams.len(), 20);
        assert_eq!(league.total_rounds, 38); // 20 teams, double round-robin
    }
}
