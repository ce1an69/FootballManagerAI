use crate::team::{Player, Position};

/// Suggestion for a substitution during a match
#[derive(Debug, Clone)]
pub struct SubstitutionSuggestion {
    pub player_out_id: String,
    pub player_out_name: String,
    pub player_in_id: String,
    pub player_in_name: String,
    pub reason: SubstitutionReason,
    pub urgency: SuggestionUrgency,
}

/// Reason for suggesting a substitution
#[derive(Debug, Clone)]
pub enum SubstitutionReason {
    /// Player fitness is critically low
    LowFitness { current: u8 },
    /// Player fatigue is too high
    HighFatigue { current: u8 },
    /// Player is injured
    Injured,
    /// Need to boost attacking play
    TacticalAttacking,
    /// Need to bolster defense
    TacticalDefensive,
}

impl std::fmt::Display for SubstitutionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubstitutionReason::LowFitness { current } => {
                write!(f, "体能过低 ({}%)", current)
            }
            SubstitutionReason::HighFatigue { current } => {
                write!(f, "疲劳过高 ({}%)", current)
            }
            SubstitutionReason::Injured => {
                write!(f, "球员受伤")
            }
            SubstitutionReason::TacticalAttacking => {
                write!(f, "加强进攻")
            }
            SubstitutionReason::TacticalDefensive => {
                write!(f, "加强防守")
            }
        }
    }
}

/// Urgency level of the substitution suggestion
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuggestionUrgency {
    /// Can safely ignore
    Low,
    /// Should consider
    Medium,
    /// Strongly recommended
    High,
    /// Must substitute immediately
    Critical,
}

impl std::fmt::Display for SuggestionUrgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuggestionUrgency::Low => write!(f, "低"),
            SuggestionUrgency::Medium => write!(f, "中"),
            SuggestionUrgency::High => write!(f, "高"),
            SuggestionUrgency::Critical => write!(f, "紧急"),
        }
    }
}

/// Advisor for suggesting substitutions during matches
pub struct SubstitutionAdvisor;

impl SubstitutionAdvisor {
    /// Analyze match state and generate substitution suggestions
    ///
    /// # Arguments
    /// * `minute` - Current match minute
    /// * `players_on_pitch` - Players currently on the pitch
    /// * `players_on_bench` - Available substitute players
    /// * `score_diff` - Home score minus away score (positive = home leading)
    /// * `subs_remaining` - Number of substitutions remaining
    ///
    /// # Returns
    /// A vector of substitution suggestions, sorted by urgency (highest first)
    ///
    /// # Examples
    /// ```no_run
    /// use football_manager_ai::ai::substitution::SubstitutionAdvisor;
    ///
    /// let suggestions = SubstitutionAdvisor::analyze(
    ///     75,
    ///     &home_starting_11,
    ///     &home_subs,
    ///     1, // Home team leading by 1 goal
    ///     3, // 3 subs remaining
    /// );
    /// ```
    pub fn analyze(
        minute: u8,
        players_on_pitch: &[Player],
        players_on_bench: &[Player],
        score_diff: i8,
        subs_remaining: u8,
    ) -> Vec<SubstitutionSuggestion> {
        let mut suggestions = Vec::new();

        // Don't generate suggestions in first half or if no subs left
        if subs_remaining == 0 || minute < 45 {
            return suggestions;
        }

        // 1. Check for injured players (highest priority)
        for player in players_on_pitch {
            if player.match_fitness == 0 {
                // Player is injured, must substitute
                if let Some(replacement) = Self::find_replacement(
                    player,
                    players_on_bench,
                    &SubstitutionReason::Injured,
                ) {
                    suggestions.push(SubstitutionSuggestion {
                        player_out_id: player.id.clone(),
                        player_out_name: player.name.clone(),
                        player_in_id: replacement.id.clone(),
                        player_in_name: replacement.name.clone(),
                        reason: SubstitutionReason::Injured,
                        urgency: SuggestionUrgency::Critical,
                    });
                }
            }
        }

        // 2. Check for critically low fitness (< 35%)
        for player in players_on_pitch {
            if player.match_fitness < 35 && player.match_fitness > 0 {
                if let Some(replacement) = Self::find_replacement(
                    player,
                    players_on_bench,
                    &SubstitutionReason::LowFitness { current: player.match_fitness },
                ) {
                    // Avoid duplicate suggestions
                    if !suggestions.iter().any(|s| s.player_out_id == player.id) {
                        suggestions.push(SubstitutionSuggestion {
                            player_out_id: player.id.clone(),
                            player_out_name: player.name.clone(),
                            player_in_id: replacement.id.clone(),
                            player_in_name: replacement.name.clone(),
                            reason: SubstitutionReason::LowFitness { current: player.match_fitness },
                            urgency: SuggestionUrgency::High,
                        });
                    }
                }
            }
        }

        // 3. Check for high fatigue (> 75%) but acceptable fitness
        for player in players_on_pitch {
            if player.fatigue > 75 && player.match_fitness >= 35 {
                if let Some(replacement) = Self::find_replacement(
                    player,
                    players_on_bench,
                    &SubstitutionReason::HighFatigue { current: player.fatigue },
                ) {
                    if !suggestions.iter().any(|s| s.player_out_id == player.id) {
                        suggestions.push(SubstitutionSuggestion {
                            player_out_id: player.id.clone(),
                            player_out_name: player.name.clone(),
                            player_in_id: replacement.id.clone(),
                            player_in_name: replacement.name.clone(),
                            reason: SubstitutionReason::HighFatigue { current: player.fatigue },
                            urgency: SuggestionUrgency::Medium,
                        });
                    }
                }
            }
        }

        // 4. Tactical adjustment suggestions (60+ minutes, at least 2 subs remaining)
        if minute >= 60 && subs_remaining >= 2 && suggestions.is_empty() {
            if score_diff < 0 {
                // Behind - suggest attacking changes
                Self::suggest_tactical_substitution(
                    players_on_pitch,
                    players_on_bench,
                    true, // attacking
                    &mut suggestions,
                );
            } else if score_diff == 0 && minute >= 75 {
                // Draw late in match - consider defensive changes
                Self::suggest_tactical_substitution(
                    players_on_pitch,
                    players_on_bench,
                    false, // defensive
                    &mut suggestions,
                );
            }
        }

        // Sort by urgency (highest first)
        suggestions.sort_by(|a, b| b.urgency.cmp(&a.urgency));

        // Limit to at most 3 suggestions
        suggestions.truncate(3);

        suggestions
    }

    /// Find a suitable replacement player from the bench
    fn find_replacement<'a>(
        player_out: &Player,
        bench: &'a [Player],
        _reason: &SubstitutionReason,
    ) -> Option<&'a Player> {
        let position = &player_out.position;

        // First try: same position with good fitness
        let same_position = bench
            .iter()
            .filter(|p| p.position == *position)
            .filter(|p| p.match_fitness > 50)
            .max_by_key(|p| p.match_fitness);

        if same_position.is_some() {
            return same_position;
        }

        // Second try: compatible position with good fitness
        let compatible = bench
            .iter()
            .filter(|p| p.match_fitness > 50)
            .filter(|p| Self::positions_compatible(position, &p.position))
            .max_by_key(|p| p.match_fitness);

        compatible
    }

    /// Check if two positions are compatible for substitution
    fn positions_compatible(pos1: &Position, pos2: &Position) -> bool {
        use crate::team::Position;

        match (pos1, pos2) {
            // Exact match
            (a, b) if a == b => true,

            // Wide defenders can play left or right
            (Position::LB, Position::RB) | (Position::RB, Position::LB) => true,
            (Position::LW, Position::RW) | (Position::RW, Position::LW) => true,

            // Central defenders can swap
            (Position::CB, Position::CB) => true,

            // Wing backs are versatile
            (Position::WB, Position::LB) | (Position::LB, Position::WB) => true,
            (Position::WB, Position::RB) | (Position::RB, Position::WB) => true,

            // Midfielders can often swap
            (Position::CM, Position::DM) | (Position::DM, Position::CM) => true,
            (Position::CM, Position::AM) | (Position::AM, Position::CM) => true,

            // Wide midfielders and wingers
            (Position::LW, Position::AM) | (Position::AM, Position::LW) => true,
            (Position::RW, Position::AM) | (Position::AM, Position::RW) => true,

            // Forwards
            (Position::ST, Position::CF) | (Position::CF, Position::ST) => true,

            // Otherwise not compatible
            _ => false,
        }
    }

    /// Suggest a tactical substitution
    fn suggest_tactical_substitution(
        players_on_pitch: &[Player],
        players_on_bench: &[Player],
        attacking: bool,
        suggestions: &mut Vec<SubstitutionSuggestion>,
    ) {
        use crate::team::Position;

        // Define target positions based on tactical need
        let target_positions = if attacking {
            vec![
                Position::ST,
                Position::CF,
                Position::AM,
                Position::LW,
                Position::RW,
            ]
        } else {
            vec![
                Position::DM,
                Position::CB,
                Position::RB,
                Position::LB,
                Position::CM,
            ]
        };

        // Find the best bench player in target positions
        for target_pos in target_positions {
            let best_bench = players_on_bench
                .iter()
                .filter(|p| p.position == target_pos)
                .filter(|p| p.match_fitness > 60)
                .max_by_key(|p| p.current_ability);

            if let Some(bench_player) = best_bench {
                // Find a player on pitch in that position to replace
                let player_to_sub = players_on_pitch
                    .iter()
                    .find(|p| p.position == target_pos);

                if let Some(player_out) = player_to_sub {
                    suggestions.push(SubstitutionSuggestion {
                        player_out_id: player_out.id.clone(),
                        player_out_name: player_out.name.clone(),
                        player_in_id: bench_player.id.clone(),
                        player_in_name: bench_player.name.clone(),
                        reason: if attacking {
                            SubstitutionReason::TacticalAttacking
                        } else {
                            SubstitutionReason::TacticalDefensive
                        },
                        urgency: SuggestionUrgency::Low,
                    });
                    return; // Only suggest one tactical sub
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Position;

    fn create_test_player(
        id: &str,
        name: &str,
        position: Position,
        match_fitness: u8,
        fatigue: u8,
    ) -> Player {
        Player {
            id: id.to_string(),
            team_id: None,
            name: name.to_string(),
            age: 25,
            nationality: "Test".to_string(),
            position: position.clone(),
            second_positions: vec![],
            preferred_foot: crate::team::Foot::Right,
            height: 180,
            weight: 75,
            corners: 100,
            crossing: 100,
            dribbling: 100,
            finishing: 100,
            heading: 100,
            long_shots: 100,
            long_throws: 100,
            marking: 100,
            passing: 100,
            penalties: 100,
            tackling: 100,
            technique: 100,
            aggression: 100,
            anticipation: 100,
            bravery: 100,
            creativity: 100,
            decisions: 100,
            concentration: 100,
            positioning: 100,
            off_the_ball: 100,
            work_rate: 100,
            pressure: 100,
            teamwork: 100,
            vision: 100,
            acceleration: 100,
            agility: 100,
            balance: 100,
            pace: 100,
            stamina: 100,
            strength: 100,
            aerial_reach: 100,
            command_of_area: 50,
            communication: 100,
            eccentricity: 50,
            handling: 50,
            kicking: 50,
            throwing: 50,
            reflexes: 50,
            rushing_out: 50,
            gk_positioning: 50,
            potential_ability: 150,
            current_ability: 150,
            adaptability: 100,
            ambition: 100,
            professionalism: 100,
            loyalty: 100,
            injury_proneness: 50,
            controversy: 50,
            match_fitness,
            morale: 50,
            status: crate::team::PlayerStatus::Healthy,
            injury_days: None,
            fatigue,
            wage: 10000,
            contract_years: 3,
            market_value: 5000000,
        }
    }

    #[test]
    fn test_injury_substitution() {
        let injured_player = create_test_player("1", "Injured ST", Position::ST, 0, 50);
        let bench_player = create_test_player("2", "Bench ST", Position::ST, 90, 10);

        let suggestions = SubstitutionAdvisor::analyze(
            60,
            &[injured_player.clone()],
            &[bench_player.clone()],
            0,
            3,
        );

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].urgency, SuggestionUrgency::Critical);
        assert_eq!(suggestions[0].player_out_id, "1");
        assert_eq!(suggestions[0].player_in_id, "2");
        assert!(matches!(suggestions[0].reason, SubstitutionReason::Injured));
    }

    #[test]
    fn test_low_fitness_substitution() {
        let tired_player = create_test_player("1", "Tired CM", Position::CM, 30, 60);
        let bench_player = create_test_player("2", "Fresh CM", Position::CM, 85, 20);

        let suggestions = SubstitutionAdvisor::analyze(
            70,
            &[tired_player.clone()],
            &[bench_player.clone()],
            0,
            3,
        );

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].urgency, SuggestionUrgency::High);
        assert!(matches!(
            suggestions[0].reason,
            SubstitutionReason::LowFitness { current: 30 }
        ));
    }

    #[test]
    fn test_high_fatigue_substitution() {
        let fatigued_player = create_test_player("1", "Fatigued LB", Position::LB, 50, 80);
        let bench_player = create_test_player("2", "Fresh LB", Position::LB, 90, 15);

        let suggestions = SubstitutionAdvisor::analyze(
            75,
            &[fatigued_player.clone()],
            &[bench_player.clone()],
            0,
            3,
        );

        assert_eq!(suggestions.len(), 1);
        assert_eq!(suggestions[0].urgency, SuggestionUrgency::Medium);
    }

    #[test]
    fn test_no_suggestions_first_half() {
        let player = create_test_player("1", "Player", Position::ST, 90, 20);
        let bench = create_test_player("2", "Sub", Position::ST, 85, 20);

        let suggestions = SubstitutionAdvisor::analyze(30, &[player], &[bench], 0, 3);

        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_no_suggestions_no_subs_left() {
        let player = create_test_player("1", "Player", Position::ST, 90, 20);
        let bench = create_test_player("2", "Sub", Position::ST, 85, 20);

        let suggestions = SubstitutionAdvisor::analyze(60, &[player], &[bench], 0, 0);

        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_suggestions_sorted_by_urgency() {
        let injured = create_test_player("1", "Injured", Position::ST, 0, 50);
        let low_fitness = create_test_player("2", "Low Fitness", Position::CM, 30, 60);
        let fatigued = create_test_player("3", "Fatigued", Position::LB, 50, 80);

        let bench_st = create_test_player("10", "Bench ST", Position::ST, 90, 10);
        let bench_cm = create_test_player("11", "Bench CM", Position::CM, 85, 20);
        let bench_lb = create_test_player("12", "Bench LB", Position::LB, 90, 15);

        let suggestions = SubstitutionAdvisor::analyze(
            70,
            &[injured, low_fitness, fatigued],
            &[bench_st, bench_cm, bench_lb],
            0,
            3,
        );

        assert_eq!(suggestions.len(), 3);
        assert_eq!(suggestions[0].urgency, SuggestionUrgency::Critical); // Injured
        assert_eq!(suggestions[1].urgency, SuggestionUrgency::High); // Low fitness
        assert_eq!(suggestions[2].urgency, SuggestionUrgency::Medium); // Fatigued
    }

    #[test]
    fn test_max_three_suggestions() {
        // Create 6 players needing substitution
        let players: Vec<Player> = (0..6)
            .map(|i| create_test_player(&i.to_string(), &format!("P{}", i), Position::CM, 30, 60))
            .collect();

        let bench: Vec<Player> = (0..6)
            .map(|i| {
                create_test_player(
                    &format!("b{}", i),
                    &format!("B{}", i),
                    Position::CM,
                    90,
                    10,
                )
            })
            .collect();

        let suggestions = SubstitutionAdvisor::analyze(70, &players, &bench, 0, 3);

        assert!(suggestions.len() <= 3);
    }
}
