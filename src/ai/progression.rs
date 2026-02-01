use crate::team::{Player, Position};
use rand::Rng;
use std::collections::HashMap;

/// Update players after a match
///
/// Applies fatigue based on minutes played and potential injuries
pub fn update_players_after_match(
    players: &mut [Player],
    minutes_played: &HashMap<String, u32>,
) -> Vec<PlayerUpdate> {
    let mut updates = Vec::new();
    let mut rng = rand::thread_rng();

    for player in players.iter_mut() {
        let minutes = minutes_played.get(&player.id).copied().unwrap_or(0);

        // Apply fatigue
        if minutes > 0 {
            let fatigue_increase = (minutes as f32 / 90.0 * 20.0) as u8;
            player.fatigue = (player.fatigue + fatigue_increase).min(100);

            updates.push(PlayerUpdate {
                player_id: player.id.clone(),
                update_type: PlayerUpdateType::FatigueIncreased(fatigue_increase),
            });
        }

        // Check for injury (small chance based on minutes)
        if minutes > 0 && player.injury_days.is_none() {
            let injury_chance = (minutes as f32 / 90.0) * 0.02; // 2% chance for full match
            if rng.gen::<f32>() < injury_chance {
                let injury_days = rng.gen_range(1..=21); // 1-21 days
                player.injury_days = Some(injury_days);

                updates.push(PlayerUpdate {
                    player_id: player.id.clone(),
                    update_type: PlayerUpdateType::Injured(injury_days),
                });
            }
        }

        // Small chance of morale change
        if minutes > 0 && rng.gen::<f32>() < 0.1 {
            let morale_change = if rng.gen() { 5 } else { -5 };
            player.morale = (player.morale as i8 + morale_change).clamp(0, 100) as u8;

            updates.push(PlayerUpdate {
                player_id: player.id.clone(),
                update_type: PlayerUpdateType::MoraleChanged(morale_change),
            });
        }
    }

    updates
}

/// Update players during a break period (e.g., between seasons)
///
/// Reduces fatigue and heals injuries over time
pub fn update_players_during_break(
    players: &mut [Player],
    days: u32,
) -> Vec<PlayerUpdate> {
    let mut updates = Vec::new();

    for player in players.iter_mut() {
        // Recover fatigue
        if player.fatigue > 0 {
            let fatigue_recovery = ((days as f32).min(7.0) * 10.0) as u8; // Recover up to 70% fatigue per week
            let new_fatigue = player.fatigue.saturating_sub(fatigue_recovery);
            let recovered = player.fatigue - new_fatigue;
            player.fatigue = new_fatigue;

            if recovered > 0 {
                updates.push(PlayerUpdate {
                    player_id: player.id.clone(),
                    update_type: PlayerUpdateType::FatigueRecovered(recovered),
                });
            }
        }

        // Heal injuries
        if let Some(injury_days) = player.injury_days {
            let new_injury_days = injury_days.saturating_sub(days as u8);
            let healed = injury_days - new_injury_days;
            player.injury_days = if new_injury_days == 0 { None } else { Some(new_injury_days) };

            if healed > 0 {
                updates.push(PlayerUpdate {
                    player_id: player.id.clone(),
                    update_type: PlayerUpdateType::InjuryHealed(healed),
                });
            }
        }

        // Small morale recovery during breaks
        if player.morale < 80 && days > 0 {
            let morale_recovery = ((days as f32).min(14.0) * 0.5) as i8; // Up to 7 morale per 2 weeks
            let new_morale = (player.morale as i8 + morale_recovery).min(100) as u8;
            let change = new_morale as i8 - player.morale as i8;
            player.morale = new_morale;

            if change > 0 {
                updates.push(PlayerUpdate {
                    player_id: player.id.clone(),
                    update_type: PlayerUpdateType::MoraleChanged(change),
                });
            }
        }
    }

    updates
}

/// Age all players by one year and handle development/decline
pub fn age_players(players: &mut [Player]) -> Vec<PlayerUpdate> {
    let mut updates = Vec::new();

    for player in players.iter_mut() {
        let old_age = player.age;
        player.age += 1;

        // Age-related development/decline
        let development = calculate_age_development(player);

        // Apply ability changes
        if development != 0 {
            // Distribute change across key attributes based on position
            apply_development_to_attributes(player, development);

            updates.push(PlayerUpdate {
                player_id: player.id.clone(),
                update_type: if development > 0 {
                    PlayerUpdateType::Developed(development)
                } else {
                    PlayerUpdateType::Declined(development.unsigned_abs())
                },
            });
        }

        // Potential adjustment
        adjust_potential_based_on_age(player);

        // Check for retirement
        if should_retire(player) {
            updates.push(PlayerUpdate {
                player_id: player.id.clone(),
                update_type: PlayerUpdateType::Retired,
            });
        }

        updates.push(PlayerUpdate {
            player_id: player.id.clone(),
            update_type: PlayerUpdateType::Aged(old_age, player.age),
        });
    }

    updates
}

/// Calculate ability development/decline based on age
fn calculate_age_development(player: &Player) -> i8 {
    match player.age {
        // Young players: Development phase
        16..=20 => {
            let development_chance = player.potential_ability - player.current_ability;
            if development_chance > 0 {
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < 0.3 {
                    // Develop by 1-3 points
                    rng.gen_range(1..=3)
                } else {
                    0
                }
            } else {
                0
            }
        }
        // Peak years: Maintains or slight decline
        21..=28 => {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < 0.1 {
                -1 // Small chance of decline
            } else {
                0
            }
        }
        // Decline phase: Accelerated decline
        29..=32 => {
            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < 0.3 {
                rng.gen_range(-3..=-1)
            } else {
                0
            }
        }
        // Rapid decline
        33..=36 => {
            let mut rng = rand::thread_rng();
            rng.gen_range(-5..=-2)
        }
        // Final years
        _ => {
            let mut rng = rand::thread_rng();
            rng.gen_range(-7..=-3)
        }
    }
}

/// Apply development to position-specific attributes
fn apply_development_to_attributes(player: &mut Player, development: i8) {
    let attributes = get_key_attributes_for_position(&player.position);
    let attr_count = attributes.len();

    for attr in &attributes {
        let current = get_attribute_value(player, attr);
        let change = (development as f32 / attr_count as f32).round() as i16;
        let new_value = (current as i16 + change).clamp(0, 200) as u16;
        set_attribute_value(player, attr, new_value);
    }

    // Recalculate overall ability
    player.current_ability = player.calculate_overall_ability();
}

/// Get key attributes for a position
fn get_key_attributes_for_position(position: &Position) -> Vec<&'static str> {
    match position {
        Position::GK => vec!["handling", "reflexes", "gk_positioning", "aerial_reach"],
        Position::CB | Position::WB => vec!["tackling", "heading", "positioning", "strength"],
        Position::LB | Position::RB => vec!["tackling", "pace", "stamina", "crossing"],
        Position::DM => vec!["tackling", "passing", "positioning", "work_rate"],
        Position::CM => vec!["passing", "vision", "technique", "stamina"],
        Position::AM => vec!["passing", "vision", "dribbling", "technique"],
        Position::LW | Position::RW => vec!["pace", "dribbling", "crossing", "finishing"],
        Position::ST | Position::CF => vec!["finishing", "heading", "pace", "technique"],
    }
}

/// Get attribute value by name
fn get_attribute_value(player: &Player, attr: &str) -> u16 {
    match attr {
        "pace" => player.pace,
        "passing" => player.passing,
        "dribbling" => player.dribbling,
        "heading" => player.heading,
        "finishing" => player.finishing,
        "tackling" => player.tackling,
        "technique" => player.technique,
        "positioning" => player.positioning,
        "vision" => player.vision,
        "work_rate" => player.work_rate,
        "stamina" => player.stamina,
        "strength" => player.strength,
        "aggression" => player.aggression,
        "handling" => player.handling,
        "reflexes" => player.reflexes,
        "aerial_reach" => player.aerial_reach,
        "command_of_area" => player.command_of_area,
        "throwing" => player.throwing,
        "kicking" => player.kicking,
        "gk_positioning" => player.gk_positioning,
        _ => 100,
    }
}

/// Set attribute value by name
fn set_attribute_value(player: &mut Player, attr: &str, value: u16) {
    match attr {
        "pace" => player.pace = value,
        "passing" => player.passing = value,
        "dribbling" => player.dribbling = value,
        "heading" => player.heading = value,
        "finishing" => player.finishing = value,
        "tackling" => player.tackling = value,
        "technique" => player.technique = value,
        "positioning" => player.positioning = value,
        "vision" => player.vision = value,
        "work_rate" => player.work_rate = value,
        "stamina" => player.stamina = value,
        "strength" => player.strength = value,
        "aggression" => player.aggression = value,
        "handling" => player.handling = value,
        "reflexes" => player.reflexes = value,
        "aerial_reach" => player.aerial_reach = value,
        "command_of_area" => player.command_of_area = value,
        "throwing" => player.throwing = value,
        "kicking" => player.kicking = value,
        "gk_positioning" => player.gk_positioning = value,
        _ => {}
    }
}

/// Adjust potential based on age and performance
fn adjust_potential_based_on_age(player: &mut Player) {
    // Potential decreases as player approaches retirement
    if player.age > 30 {
        let decline = (player.age - 30) as u16 * 2;
        player.potential_ability = player.potential_ability.saturating_sub(decline);
    }
}

/// Check if player should retire
fn should_retire(player: &Player) -> bool {
    if player.age < 33 {
        return false;
    }

    // Retirement probability increases with age
    let retirement_chance = match player.age {
        33..=35 => 0.1,
        36..=38 => 0.3,
        39..=40 => 0.6,
        _ => 0.9,
    };

    // Also consider ability - low ability players retire earlier
    let ability_factor = if player.current_ability < 100 {
        0.2
    } else {
        0.0
    };

    let mut rng = rand::thread_rng();
    rng.gen::<f32>() < (retirement_chance + ability_factor)
}

/// Types of player updates
#[derive(Debug, Clone, PartialEq)]
pub enum PlayerUpdateType {
    Aged(u8, u8), // From age, to age
    Developed(i8), // Ability increased by X
    Declined(u8), // Ability decreased by X
    FatigueIncreased(u8),
    FatigueRecovered(u8),
    Injured(u8), // For N days
    InjuryHealed(u8), // Healed N days
    MoraleChanged(i8),
    Retired,
}

/// Player update event
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerUpdate {
    pub player_id: String,
    pub update_type: PlayerUpdateType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Player;

    #[test]
    fn test_update_fatigue_after_match() {
        let mut player = Player::new("test_player".to_string(), "Test Player".to_string(), Position::CM);
        let mut players = vec![player.clone()];
        let mut minutes = HashMap::new();
        minutes.insert("test_player".to_string(), 90);

        let updates = update_players_after_match(&mut players, &minutes);

        assert!(players[0].fatigue > 0);
        assert!(updates.iter().any(|u| matches!(u.update_type, PlayerUpdateType::FatigueIncreased(_))));
    }

    #[test]
    fn test_recover_fatigue_during_break() {
        let mut player = Player::new("test_player".to_string(), "Test Player".to_string(), Position::CM);
        player.fatigue = 80;
        let mut players = vec![player.clone()];

        update_players_during_break(&mut players, 7);

        assert!(players[0].fatigue < player.fatigue);
    }

    #[test]
    fn test_age_players() {
        let mut player = Player::new("test_player".to_string(), "Test Player".to_string(), Position::CM);
        player.age = 20;
        let mut players = vec![player.clone()];

        let updates = age_players(&mut players);

        assert_eq!(players[0].age, 21);
        assert!(updates.iter().any(|u| matches!(u.update_type, PlayerUpdateType::Aged(20, 21))));
    }

    #[test]
    fn test_injury_chance() {
        let mut player = Player::new("test_player".to_string(), "Test Player".to_string(), Position::CM);
        let mut players = vec![player.clone()];
        let mut minutes = HashMap::new();
        minutes.insert("test_player".to_string(), 90);

        // Run multiple times to see if injuries occur
        let mut injury_count = 0;
        for _ in 0..1000 {
            let mut test_player = player.clone();
            let mut test_players = vec![test_player];
            let updates = update_players_after_match(&mut test_players, &minutes);
            if updates.iter().any(|u| matches!(u.update_type, PlayerUpdateType::Injured(_))) {
                injury_count += 1;
            }
        }

        // Should have some injuries (~2% per match)
        assert!(injury_count > 0 && injury_count < 100);
    }
}
