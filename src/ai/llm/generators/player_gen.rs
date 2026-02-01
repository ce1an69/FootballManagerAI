//! Player generator using LLM
//!
//! This module provides functionality to generate player data using LLM APIs.

use crate::ai::llm::{LLMClient, Result, LlmError, PromptTemplates};
use crate::team::{Player, Position};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PlayerAttributes {
    finishing: u16,
    passing: u16,
    pace: u16,
    stamina: u16,
    strength: u16,
}

#[derive(Debug, Deserialize)]
struct LlmPlayerData {
    name: String,
    nationality: String,
    attributes: PlayerAttributes,
}

/// Player generator using LLM
pub struct PlayerGenerator {
    client: LLMClient,
}

impl PlayerGenerator {
    /// Create a new player generator
    pub fn new(client: LLMClient) -> Self {
        Self { client }
    }

    /// Generate a player with the given specifications
    pub async fn generate(&self, position: Position, skill: u16, age: u8) -> Result<Player> {
        let position_str = format!("{:?}", position);
        let prompt = PromptTemplates::generate_player(&position_str, skill, age);

        let response = self
            .client
            .chat(&[
                crate::ai::llm::Message::system("You are a football manager game data generator."),
                crate::ai::llm::Message::user(&prompt),
            ])
            .await?;

        self.parse_response(&response, position)
    }

    /// Parse the LLM response into a Player
    fn parse_response(&self, json: &str, position: Position) -> Result<Player> {
        // Try to extract JSON (remove possible markdown code blocks)
        let json = extract_json(json)?;

        let data: LlmPlayerData = serde_json::from_str(json).map_err(LlmError::JsonParse)?;

        let mut player = Player::new(
            uuid::Uuid::new_v4().to_string(),
            data.name,
            position,
        );

        // Set LLM-generated attributes
        player.finishing = data.attributes.finishing;
        player.passing = data.attributes.passing;
        player.pace = data.attributes.pace;
        player.stamina = data.attributes.stamina;
        player.strength = data.attributes.strength;

        Ok(player)
    }
}

/// Extract JSON from text, removing markdown code blocks if present
fn extract_json(text: &str) -> Result<&str> {
    let text = text.trim();

    // Remove markdown code blocks
    if text.starts_with("```json") {
        let start = 7; // "```json".len()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_code_block() {
        let input = r#"```json
{"name": "Test"}
```"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result.trim(), r#"{"name": "Test"}"#);
    }

    #[test]
    fn test_extract_json_plain() {
        let input = r#"{"name": "Test"}"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result, r#"{"name": "Test"}"#);
    }
}
