//! Team generator using LLM
//!
//! This module provides functionality to generate team data using LLM APIs.

use crate::ai::llm::{LLMClient, Result, LlmError, PromptTemplates, Message};
use crate::team::Team;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StadiumData {
    name: String,
    capacity: u32,
}

#[derive(Debug, Deserialize)]
struct LlmTeamData {
    name: String,
    abbreviation: String,
    founded_year: u32,
    stadium: StadiumData,
}

/// Team generator using LLM
pub struct TeamGenerator {
    client: LLMClient,
}

impl TeamGenerator {
    /// Create a new team generator
    pub fn new(client: LLMClient) -> Self {
        Self { client }
    }

    /// Generate a team in the given league
    pub async fn generate(&self, league_name: &str, team_id: String) -> Result<Team> {
        let prompt = PromptTemplates::generate_team(league_name);

        let response = self
            .client
            .chat(&[
                Message::system("You are a football manager game data generator."),
                Message::user(&prompt),
            ])
            .await?;

        self.parse_response(&response, team_id, league_name)
    }

    /// Parse the LLM response into a Team
    fn parse_response(&self, json: &str, team_id: String, league_id: &str) -> Result<Team> {
        let json = extract_json(json)?;

        let data: LlmTeamData = serde_json::from_str(json).map_err(LlmError::JsonParse)?;

        let team = Team::new(
            team_id,
            data.name,
            league_id.to_string(),
            10_000_000, // Default budget
        );

        // TODO: Extend Team struct to store more information
        // team.abbreviation = data.abbreviation;
        // team.founded_year = data.founded_year;
        // team.stadium_name = data.stadium.name;
        // team.stadium_capacity = data.stadium.capacity;

        Ok(team)
    }
}

/// Extract JSON from text, removing markdown code blocks if present
fn extract_json(text: &str) -> Result<&str> {
    let text = text.trim();

    if text.starts_with("```json") {
        let start = 7;
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else if text.starts_with("```") {
        let start = 3;
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else {
        Ok(text)
    }
}
