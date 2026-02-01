//! Game data generator using LLM
//!
//! This module provides unified game data generation coordinating
//! player, team, and league generation.

use crate::ai::llm::{LLMClient, Result, PlayerGenerator, TeamGenerator};
use crate::team::{Team, League, Position};

/// Unified game data generator using LLM
pub struct GameDataGenerator {
    client: LLMClient,
    player_gen: PlayerGenerator,
    team_gen: TeamGenerator,
}

impl GameDataGenerator {
    /// Create a new game data generator
    pub fn new(client: LLMClient) -> Self {
        let player_gen = PlayerGenerator::new(client.clone());
        let team_gen = TeamGenerator::new(client.clone());

        Self {
            client,
            player_gen,
            team_gen,
        }
    }

    /// Generate a league
    pub async fn generate_league(&self) -> Result<League> {
        let league_name = "Premier League".to_string();
        let mut teams = Vec::new();

        for i in 0..20 {
            teams.push(format!("team-{}", i));
        }

        let mut league = League::new("league1".to_string(), league_name, teams);

        league.generate_schedule();
        Ok(league)
    }

    /// Generate teams for a league
    pub async fn generate_teams(&self, league: &League, count: usize) -> Result<Vec<Team>> {
        let mut teams = Vec::new();

        for i in 0..count {
            let team_id = format!("team-{}", i);
            let team = self.team_gen.generate(&league.name, team_id).await?;
            teams.push(team);
        }

        Ok(teams)
    }

    /// Generate players for a team
    pub async fn generate_team_players(&self, _team: &Team) -> Result<Vec<crate::team::Player>> {
        let mut players = Vec::new();

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
                _ => Position::CM,
            };

            let player = self.player_gen.generate(position, 100, 25).await?;
            players.push(player);
        }

        Ok(players)
    }
}
