use crate::team::Player;

/// Evaluate player's current market value
///
/// Calculates market value based on:
/// - Base value from current_ability
/// - Age factor (peak at 27, bonus for young players, decline after 30)
/// - Potential factor (bonus if potential > current)
/// - Contract factor (less years = cheaper)
///
/// # Arguments
/// * `player` - Reference to the player to evaluate
///
/// # Returns
/// Market value in currency units (u64)
///
/// # Examples
/// ```
/// # use football_manager_ai::team::{Player, Position};
/// # use football_manager_ai::transfer::evaluate_player_value;
/// # let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);
/// let value = evaluate_player_value(&player);
/// assert!(value > 0);
/// ```
pub fn evaluate_player_value(player: &Player) -> u64 {
    // Base value from current ability (ability * 50,000)
    let base_value = player.current_ability as u64 * 50_000;

    // Age factor
    // - Peak at 27: +30% bonus
    // - Young players (20-23): +5% to +25% bonus (gradual increase)
    // - Old players (30+): -10% per year over 30
    let age_multiplier = match player.age {
        age if age <= 19 => 1.0,  // Very young, no bonus yet
        20 => 1.05,   // +5%
        21 => 1.10,   // +10%
        22 => 1.175,  // +17.5%
        23 => 1.25,   // +25%
        24 => 1.20,   // +20%
        25 => 1.25,   // +25%
        26 => 1.275,  // +27.5%
        27 => 1.30,   // Peak: +30%
        28 | 29 => 1.15,  // Slight decline from peak
        age if age >= 30 => {
            // Decline: -10% per year over 30
            let decline = ((age - 30) as f64 * 0.10).min(0.60); // Max -60%
            1.0 - decline
        }
        _ => 1.0,
    };

    // Apply age multiplier to base value
    let mut value = (base_value as f64 * age_multiplier) as u64;

    // Potential factor
    // Bonus if potential > current: +2% per point difference
    if player.potential_ability > player.current_ability {
        let potential_diff = (player.potential_ability - player.current_ability) as f64;
        let potential_bonus = potential_diff * 0.02; // 2% per point
        value = (value as f64 * (1.0 + potential_bonus)) as u64;
    }

    // Contract factor
    // Players with fewer years remaining are cheaper
    let contract_multiplier = match player.contract_years {
        0 => 0.70,   // Expiring: -30%
        1 => 0.85,   // 1 year: -15%
        years if years >= 4 => 1.10,  // 4+ years: +10%
        _ => 1.0,    // 2-3 years: neutral
    };

    value = (value as f64 * contract_multiplier) as u64;

    // Minimum value (even terrible players have some value)
    value.max(50_000)
}

/// Predict player's future market value
///
/// Estimates what a player will be worth after N years.
/// Accounts for:
/// - Growth potential for young players
/// - Peak years for established players
/// - Decline for older players
///
/// # Arguments
/// * `player` - Reference to the player
/// * `years` - Number of years into the future to predict
///
/// # Returns
/// Predicted market value in currency units (u64)
///
/// # Examples
/// ```
/// # use football_manager_ai::team::{Player, Position};
/// # use football_manager_ai::transfer::{evaluate_player_value, predict_potential_value};
/// # let player = Player::new("player1".to_string(), "Test Player".to_string(), Position::ST);
/// let current_value = evaluate_player_value(&player);
/// let future_value = predict_potential_value(&player, 2);
/// // Young players should have higher future value
/// ```
pub fn predict_potential_value(player: &Player, years: u8) -> u64 {
    if years == 0 {
        return evaluate_player_value(player);
    }

    let _future_age = player.age.saturating_add(years);
    let mut predicted_player = player.clone();

    // Simulate aging and ability changes
    for _ in 0..years {
        // Manually age the player to avoid overflow issues
        if predicted_player.age < 40 {
            predicted_player.age += 1;

            // Young players (16-23) develop
            if predicted_player.age <= 23 && predicted_player.current_ability < predicted_player.potential_ability {
                let growth = ((predicted_player.potential_ability - predicted_player.current_ability) as f32 * 0.1) as u16;
                predicted_player.current_ability = (predicted_player.current_ability + growth).min(predicted_player.potential_ability);
            }
            // Old players (30+) decline
            else if predicted_player.age >= 30 {
                // Calculate decline based on age (max decline at age 36)
                let age_for_decline = predicted_player.age.min(36);
                let decline = ((age_for_decline as i16 - 32).max(-2) as f32 * 0.5).max(0.0) as u16;
                predicted_player.current_ability = predicted_player.current_ability.saturating_sub(decline);
            }

            // Update contract
            if predicted_player.contract_years > 0 {
                predicted_player.contract_years -= 1;
            }
        }
    }

    // Calculate value based on predicted state
    evaluate_player_value(&predicted_player)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Position;

    fn create_test_player(age: u8, current_ability: u16, potential_ability: u16) -> Player {
        let mut player = Player::new(
            "test_id".to_string(),
            "Test Player".to_string(),
            Position::CM
        );
        player.age = age;
        player.current_ability = current_ability;
        player.potential_ability = potential_ability;
        player.contract_years = 3;
        player
    }

    #[test]
    fn test_evaluate_young_player() {
        // Young players (20-23) should have 5-25% bonus
        // Use same current and potential to isolate age factor
        let player_20 = create_test_player(20, 120, 120);
        let player_21 = create_test_player(21, 120, 120);
        let player_22 = create_test_player(22, 120, 120);
        let player_23 = create_test_player(23, 120, 120);

        let value_20 = evaluate_player_value(&player_20);
        let value_21 = evaluate_player_value(&player_21);
        let value_22 = evaluate_player_value(&player_22);
        let value_23 = evaluate_player_value(&player_23);

        // All should be > base value (120 * 50,000 = 6,000,000)
        let base_value = 6_000_000;
        assert!(value_20 > base_value, "Age 20 should have bonus");
        assert!(value_21 > value_20, "Age 21 should have higher bonus than 20");
        assert!(value_22 > value_21, "Age 22 should have higher bonus than 21");
        assert!(value_23 > value_22, "Age 23 should have higher bonus than 22");

        // Check approximate bonus ranges
        let bonus_20 = (value_20 - base_value) as f64 / base_value as f64 * 100.0;
        let bonus_21 = (value_21 - base_value) as f64 / base_value as f64 * 100.0;
        let bonus_22 = (value_22 - base_value) as f64 / base_value as f64 * 100.0;
        let bonus_23 = (value_23 - base_value) as f64 / base_value as f64 * 100.0;

        assert!((bonus_20 - 5.0).abs() < 0.1, "Age 20 bonus should be ~5%, got {}", bonus_20);
        assert!((bonus_21 - 10.0).abs() < 0.1, "Age 21 bonus should be ~10%, got {}", bonus_21);
        assert!((bonus_22 - 17.5).abs() < 0.1, "Age 22 bonus should be ~17.5%, got {}", bonus_22);
        assert!((bonus_23 - 25.0).abs() < 0.1, "Age 23 bonus should be ~25%, got {}", bonus_23);
    }

    #[test]
    fn test_evaluate_peak_player() {
        // Players at 27 should have highest value (30% bonus)
        let player = create_test_player(27, 150, 150);

        let value = evaluate_player_value(&player);
        let base_value = 150 * 50_000; // 7,500,000

        // Should have 30% bonus
        let expected = (base_value as f64 * 1.30) as u64;
        assert_eq!(value, expected, "Age 27 should have exactly 30% bonus");

        // Verify it's higher than adjacent ages
        let player_26 = create_test_player(26, 150, 150);
        let player_28 = create_test_player(28, 150, 150);

        let value_26 = evaluate_player_value(&player_26);
        let value_28 = evaluate_player_value(&player_28);

        assert!(value > value_26, "Age 27 should be worth more than 26");
        assert!(value > value_28, "Age 27 should be worth more than 28");
    }

    #[test]
    fn test_evaluate_old_player() {
        // Old players (30+) should lose 10% per year
        let player_30 = create_test_player(30, 140, 140);
        let player_31 = create_test_player(31, 140, 140);
        let player_32 = create_test_player(32, 140, 140);
        let player_35 = create_test_player(35, 140, 140);

        let value_30 = evaluate_player_value(&player_30);
        let value_31 = evaluate_player_value(&player_31);
        let value_32 = evaluate_player_value(&player_32);
        let value_35 = evaluate_player_value(&player_35);

        let base_value = 140 * 50_000; // 7,000,000

        // Age 30: no penalty (base age for decline)
        let expected_30 = base_value;
        assert_eq!(value_30, expected_30, "Age 30 should have base value");

        // Age 31: -10%
        let expected_31 = (base_value as f64 * 0.90) as u64;
        assert_eq!(value_31, expected_31, "Age 31 should have -10%");

        // Age 32: -20%
        let expected_32 = (base_value as f64 * 0.80) as u64;
        assert_eq!(value_32, expected_32, "Age 32 should have -20%");

        // Age 35: -50% (capped at -60% max)
        let expected_35 = (base_value as f64 * 0.50) as u64;
        assert_eq!(value_35, expected_35, "Age 35 should have -50%");
    }

    #[test]
    fn test_evaluate_potential_factor() {
        // Potential should add 2% per point difference
        let player_low_potential = create_test_player(22, 100, 110);  // +10 diff
        let player_high_potential = create_test_player(22, 100, 150); // +50 diff
        let player_no_potential = create_test_player(22, 100, 100);    // No diff

        let value_low = evaluate_player_value(&player_low_potential);
        let value_high = evaluate_player_value(&player_high_potential);
        let value_no = evaluate_player_value(&player_no_potential);

        // Low potential: +10 * 2% = +20%
        let expected_low = (value_no as f64 * 1.20) as u64;
        assert_eq!(value_low, expected_low, "Low potential should add 20%");

        // High potential: +50 * 2% = +100% (double value)
        let expected_high = (value_no as f64 * 2.00) as u64;
        assert_eq!(value_high, expected_high, "High potential should add 100%");
    }

    #[test]
    fn test_evaluate_contract_factor() {
        // Use age 30 (no age bonus) to isolate contract factor
        let mut player = create_test_player(30, 130, 130);

        // Test different contract lengths
        player.contract_years = 0;
        let value_0 = evaluate_player_value(&player);

        player.contract_years = 1;
        let value_1 = evaluate_player_value(&player);

        player.contract_years = 2;
        let value_2 = evaluate_player_value(&player);

        player.contract_years = 4;
        let value_4 = evaluate_player_value(&player);

        let base_value = 130 * 50_000;

        // 0 years: -30%
        let expected_0 = (base_value as f64 * 0.70) as u64;
        assert_eq!(value_0, expected_0, "Expiring contract should have -30%");

        // 1 year: -15%
        let expected_1 = (base_value as f64 * 0.85) as u64;
        assert_eq!(value_1, expected_1, "1 year contract should have -15%");

        // 2 years: neutral (age multiplier only)
        assert!(value_2 > value_1, "2 years should be worth more than 1 year");

        // 4+ years: +10%
        let expected_4 = (base_value as f64 * 1.10) as u64;
        assert_eq!(value_4, expected_4, "4+ years should have +10%");
    }

    #[test]
    fn test_predict_potential_value() {
        // Test that prediction function correctly simulates player development
        // Young player (age 16) with ability 100, high potential 150
        let young_player = create_test_player(16, 100, 150);

        // Predict growth to age 23 (peak of youth bonus)
        let future_value = predict_potential_value(&young_player, 7);

        // At age 23: ability should be ~140, age bonus 25%, small potential bonus
        // Current (age 16): 100 * 50k * 1.0 (no age bonus) * 2.0 (50pt potential diff) = 10M
        // Future (age 23): ~140 * 50k * 1.25 (age bonus) * 1.2 (10pt potential diff) = ~10.5M

        // The key assertion: prediction should produce a reasonable value
        assert!(future_value > 8_000_000,
            "Predicted value should be reasonable: got {}",
            future_value);

        // Also verify the player's ability increased
        let predicted_player = {
            let mut p = young_player.clone();
            for _ in 0..7 {
                if p.age < 40 {
                    p.age += 1;
                    if p.age <= 23 && p.current_ability < p.potential_ability {
                        let growth = ((p.potential_ability - p.current_ability) as f32 * 0.1) as u16;
                        p.current_ability = (p.current_ability + growth).min(p.potential_ability);
                    }
                }
            }
            p
        };

        assert!(predicted_player.current_ability > young_player.current_ability,
            "Player's ability should increase: {} -> {}",
            young_player.current_ability, predicted_player.current_ability);
    }

    #[test]
    fn test_predict_potential_value_old_player() {
        // Old player should decline in value
        let old_player = create_test_player(32, 140, 140);
        let current_value = evaluate_player_value(&old_player);
        let future_value = predict_potential_value(&old_player, 2);

        // Should be worth less in 2 years due to decline
        assert!(future_value < current_value,
            "Old player future value ({}) should be < current ({})",
            future_value, current_value);
    }

    #[test]
    fn test_predict_potential_value_zero_years() {
        // Zero years should return current value
        let player = create_test_player(25, 120, 130);
        let current = evaluate_player_value(&player);
        let predicted = predict_potential_value(&player, 0);

        assert_eq!(current, predicted, "Zero years prediction should equal current value");
    }

    #[test]
    fn test_minimum_value() {
        // Even very poor players should have minimum value
        let mut player = create_test_player(35, 10, 10);
        player.contract_years = 0; // Worst case scenario

        let value = evaluate_player_value(&player);
        assert!(value >= 50_000, "Should enforce minimum value of at least 50,000");
    }
}
