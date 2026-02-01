use crate::team::{PassingStyle, DefensiveHeight, Tempo, Tactic};

/// Tactical styles that teams can employ
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TacticalStyle {
    /// Possession-based football: short passing, slow tempo
    Possession,
    /// Counter-attacking: deep defensive line, long balls, fast transitions
    CounterAttack,
    /// High pressing: high defensive line, aggressive attacking mentality
    HighPress,
    /// Direct play: long balls, bypassing midfield
    DirectPlay,
    /// Balanced approach
    Balanced,
}

impl std::fmt::Display for TacticalStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TacticalStyle::Possession => write!(f, "控球"),
            TacticalStyle::CounterAttack => write!(f, "防反"),
            TacticalStyle::HighPress => write!(f, "高压"),
            TacticalStyle::DirectPlay => write!(f, "直接"),
            TacticalStyle::Balanced => write!(f, "平衡"),
        }
    }
}

/// Infer the tactical style from a team's tactic
///
/// This analyzes the formation and mentality to determine what style
/// of football the team is trying to play.
pub fn infer_tactical_style(tactic: &Tactic) -> TacticalStyle {
    // High press: high defensive line + high attacking mentality
    let high_press = tactic.defensive_height == DefensiveHeight::High
        && tactic.attacking_mentality > 70;

    // Counter attack: low defensive line + fast tempo
    let counter = tactic.defensive_height == DefensiveHeight::Low
        && tactic.tempo == Tempo::Fast;

    // Possession: short passing + slow/medium tempo
    let possession = tactic.passing_style == PassingStyle::Short
        && (tactic.tempo == Tempo::Slow || tactic.tempo == Tempo::Medium);

    // Direct: long passing style
    let direct = tactic.passing_style == PassingStyle::Long;

    match (high_press, counter, possession, direct) {
        (true, _, _, _) => TacticalStyle::HighPress,
        (_, true, _, _) => TacticalStyle::CounterAttack,
        (_, _, true, _) => TacticalStyle::Possession,
        (_, _, _, true) => TacticalStyle::DirectPlay,
        _ => TacticalStyle::Balanced,
    }
}

/// Calculate tactical modifier for match simulation
///
/// Returns a modifier between -0.25 and +0.25 that represents
/// the tactical advantage/disadvantage.
///
/// # Tactical Rock-Paper-Scissors
/// - HighPress beats Possession (disrupts build-up)
/// - Possession beats CounterAttack (controls ball, denies transitions)
/// - CounterAttack beats HighPress (exploits space behind high line)
/// - DirectPlay beats HighPress (bypasses press with long balls)
/// - Possession beats DirectPlay (controls possession)
///
/// # Arguments
/// * `home_style` - The home team's tactical style
/// * `away_style` - The away team's tactical style
///
/// # Returns
/// * Positive value (up to +0.25) means home team has tactical advantage
/// * Negative value (down to -0.25) means away team has tactical advantage
/// * Zero means styles are evenly matched or similar
///
/// # Examples
/// ```
/// use football_manager_ai::ai::tactical::{TacticalStyle, calculate_tactical_modifier};
///
/// // HighPress beats Possession (+25% advantage)
/// let modifier = calculate_tactical_modifier(&TacticalStyle::HighPress, &TacticalStyle::Possession);
/// assert!(modifier > 0.0); // Home team advantage
///
/// // Possession loses to CounterAttack (-25% disadvantage)
/// let modifier = calculate_tactical_modifier(&TacticalStyle::Possession, &TacticalStyle::CounterAttack);
/// assert!(modifier < 0.0); // Away team advantage
/// ```
pub fn calculate_tactical_modifier(
    home_style: &TacticalStyle,
    away_style: &TacticalStyle,
) -> f64 {
    use TacticalStyle::*;

    const ADVANTAGE: f64 = 0.25;
    const DISADVANTAGE: f64 = -0.25;
    const NEUTRAL: f64 = 0.0;

    let base_mod = match (home_style, away_style) {
        // Home team advantages (counter relationships)
        (HighPress, Possession) => ADVANTAGE,
        (Possession, CounterAttack) => ADVANTAGE,
        (CounterAttack, HighPress) => ADVANTAGE,
        (DirectPlay, HighPress) => ADVANTAGE,
        (Possession, DirectPlay) => ADVANTAGE,

        // Home team disadvantages (being countered)
        (Possession, HighPress) => DISADVANTAGE,
        (CounterAttack, Possession) => DISADVANTAGE,
        (HighPress, CounterAttack) => DISADVANTAGE,
        (HighPress, DirectPlay) => DISADVANTAGE,
        (DirectPlay, Possession) => DISADVANTAGE,

        // Same style or both balanced = no advantage
        _ => NEUTRAL,
    };

    base_mod
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_possession_style() {
        let tactic = Tactic {
            formation: crate::team::Formation::FourFourTwo,
            attacking_mentality: 60,
            defensive_height: DefensiveHeight::Medium,
            passing_style: PassingStyle::Short,
            tempo: Tempo::Slow,
            player_roles: vec![],
        };

        let style = infer_tactical_style(&tactic);
        assert_eq!(style, TacticalStyle::Possession);
    }

    #[test]
    fn test_infer_high_press_style() {
        let tactic = Tactic {
            formation: crate::team::Formation::FourFourTwo,
            attacking_mentality: 80,
            defensive_height: DefensiveHeight::High,
            passing_style: PassingStyle::Mixed,
            tempo: Tempo::Fast,
            player_roles: vec![],
        };

        let style = infer_tactical_style(&tactic);
        assert_eq!(style, TacticalStyle::HighPress);
    }

    #[test]
    fn test_infer_counter_attack_style() {
        let tactic = Tactic {
            formation: crate::team::Formation::FourFourTwo,
            attacking_mentality: 50,
            defensive_height: DefensiveHeight::Low,
            passing_style: PassingStyle::Mixed,
            tempo: Tempo::Fast,
            player_roles: vec![],
        };

        let style = infer_tactical_style(&tactic);
        assert_eq!(style, TacticalStyle::CounterAttack);
    }

    #[test]
    fn test_infer_direct_play_style() {
        let tactic = Tactic {
            formation: crate::team::Formation::FourFourTwo,
            attacking_mentality: 60,
            defensive_height: DefensiveHeight::Medium,
            passing_style: PassingStyle::Long,
            tempo: Tempo::Medium,
            player_roles: vec![],
        };

        let style = infer_tactical_style(&tactic);
        assert_eq!(style, TacticalStyle::DirectPlay);
    }

    #[test]
    fn test_tactical_counter_high_press_vs_possession() {
        // HighPress should beat Possession
        let modifier = calculate_tactical_modifier(&TacticalStyle::HighPress, &TacticalStyle::Possession);
        assert!(modifier > 0.0);
        assert_eq!(modifier, 0.25);
    }

    #[test]
    fn test_tactical_counter_possession_vs_counter_attack() {
        // Possession should beat CounterAttack
        let modifier = calculate_tactical_modifier(&TacticalStyle::Possession, &TacticalStyle::CounterAttack);
        assert!(modifier > 0.0);
        assert_eq!(modifier, 0.25);
    }

    #[test]
    fn test_tactical_counter_counter_attack_vs_high_press() {
        // CounterAttack should beat HighPress
        let modifier = calculate_tactical_modifier(&TacticalStyle::CounterAttack, &TacticalStyle::HighPress);
        assert!(modifier > 0.0);
        assert_eq!(modifier, 0.25);
    }

    #[test]
    fn test_tactical_disadvantage() {
        // Possession should lose to HighPress
        let modifier = calculate_tactical_modifier(&TacticalStyle::Possession, &TacticalStyle::HighPress);
        assert!(modifier < 0.0);
        assert_eq!(modifier, -0.25);
    }

    #[test]
    fn test_tactical_same_style() {
        // Same style should be neutral
        let modifier = calculate_tactical_modifier(&TacticalStyle::Possession, &TacticalStyle::Possession);
        assert_eq!(modifier, 0.0);
    }

    #[test]
    fn test_tactical_balanced_neutral() {
        // Balanced against anything should be neutral
        let modifier1 = calculate_tactical_modifier(&TacticalStyle::Balanced, &TacticalStyle::Possession);
        let modifier2 = calculate_tactical_modifier(&TacticalStyle::HighPress, &TacticalStyle::Balanced);
        assert_eq!(modifier1, 0.0);
        assert_eq!(modifier2, 0.0);
    }
}
