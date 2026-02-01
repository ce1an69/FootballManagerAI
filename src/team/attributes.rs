use crate::team::player::Player;

/// Calculate player's market value
pub fn calculate_market_value(player: &Player) -> u32 {
    let base_value = (player.current_ability as u64) * (player.current_ability as u64) * 100;

    // Age modifiers
    let age_modifier = match player.age {
        16..=21 => 150,  // Young players have higher potential value
        22..=25 => 120,  // Prime approaching
        26..=29 => 100,  // Prime years
        30..=32 => 70,   // Declining
        _ => 40,         // Old
    };

    // Potential modifier (if young and high potential)
    let potential_modifier = if player.age <= 23 && player.potential_ability > player.current_ability {
        let potential_diff = player.potential_ability - player.current_ability;
        100 + (potential_diff as u64 * 2)
    } else {
        100
    };

    // Position modifiers (some positions are more valuable)
    let position_modifier = match player.position {
        crate::team::Position::ST | crate::team::Position::CF => 120,
        crate::team::Position::AM => 110,
        crate::team::Position::CM => 100,
        crate::team::Position::LW | crate::team::Position::RW => 105,
        crate::team::Position::CB => 100,
        crate::team::Position::GK => 90,
        _ => 95,
    };

    // Morale and fitness modifiers
    let condition_modifier = {
        let morale_factor = player.morale as u64;
        let fitness_factor = player.match_fitness as u64;
        (morale_factor + fitness_factor) / 2
    };

    // Calculate final value with overflow protection
    let value = base_value
        .saturating_mul(age_modifier as u64)
        .saturating_mul(potential_modifier as u64)
        .saturating_mul(position_modifier as u64)
        .saturating_mul(condition_modifier)
        / (100 * 100 * 100 * 100);

    // Minimum value and convert to u32
    value.max(10_000) as u32
}

/// Calculate player's weekly wage
pub fn calculate_wage(player: &Player) -> u32 {
    let base_wage = (player.current_ability as u32) * 50;

    // Age modifiers
    let age_modifier = match player.age {
        16..=20 => 50,   // Young players earn less
        21..=25 => 80,
        26..=30 => 100,  // Prime earners
        31..=35 => 90,
        _ => 70,
    };

    // Potential bonus for young players
    let potential_bonus = if player.age <= 22 && player.potential_ability > player.current_ability {
        ((player.potential_ability - player.current_ability) as u32) * 20
    } else {
        0
    };

    let wage = (base_wage * age_modifier) / 100 + potential_bonus;

    // Minimum wage
    wage.max(500)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::{Position, Player};

    #[test]
    fn test_market_value_calculation() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        player.current_ability = 120;
        player.age = 25;

        let value = calculate_market_value(&player);
        assert!(value > 0);
        assert!(value >= 10_000);
    }

    #[test]
    fn test_wage_calculation() {
        let mut player = Player::new("1".to_string(), "Test".to_string(), Position::CM);
        player.current_ability = 100;
        player.age = 26;

        let wage = calculate_wage(&player);
        assert!(wage > 0);
        assert!(wage >= 500);
    }

    #[test]
    fn test_young_player_high_value() {
        let mut young_player = Player::new("1".to_string(), "Young".to_string(), Position::ST);
        young_player.current_ability = 100;
        young_player.potential_ability = 160;
        young_player.age = 18;

        let mut old_player = Player::new("2".to_string(), "Old".to_string(), Position::ST);
        old_player.current_ability = 100;
        old_player.potential_ability = 100;
        old_player.age = 32;

        let young_value = calculate_market_value(&young_player);
        let old_value = calculate_market_value(&old_player);

        assert!(young_value > old_value);
    }

    #[test]
    fn test_prime_age_higher_wage() {
        let mut prime_player = Player::new("1".to_string(), "Prime".to_string(), Position::CM);
        prime_player.current_ability = 120;
        prime_player.age = 27;

        let mut young_player = Player::new("2".to_string(), "Young".to_string(), Position::CM);
        young_player.current_ability = 120;
        young_player.age = 19;

        let prime_wage = calculate_wage(&prime_player);
        let young_wage = calculate_wage(&young_player);

        assert!(prime_wage > young_wage);
    }

    #[test]
    fn test_position_value_modifier() {
        let mut striker = Player::new("1".to_string(), "ST".to_string(), Position::ST);
        striker.current_ability = 100;
        striker.age = 25;

        let mut goalkeeper = Player::new("2".to_string(), "GK".to_string(), Position::GK);
        goalkeeper.current_ability = 100;
        goalkeeper.age = 25;

        let striker_value = calculate_market_value(&striker);
        let gk_value = calculate_market_value(&goalkeeper);

        assert!(striker_value > gk_value);
    }
}
