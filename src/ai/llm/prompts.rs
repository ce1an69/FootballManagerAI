//! Prompt templates for LLM generation
//!
//! This module provides structured prompt templates for various LLM-based
//! generation tasks including player generation, team generation, match
//! commentary, and news events.

/// Prompt templates for LLM generation
pub struct PromptTemplates;

impl PromptTemplates {
    /// Generate a prompt for creating player data
    ///
    /// # Arguments
    /// * `position` - The player's position (e.g., "ST", "MF", "GK")
    /// * `skill_level` - Target skill level (1-200)
    /// * `age` - Player's age
    pub fn generate_player(position: &str, skill_level: u16, age: u8) -> String {
        format!(
            "You are a football manager game data generator. Please generate player data in JSON format:

Position: {}
Skill Level: {} (1-200)
Age: {}

Return JSON format (return JSON only, no other text):
{{
  \"name\": \"Player Name\",
  \"nationality\": \"Nationality\",
  \"attributes\": {{
    \"finishing\": 1-200,
    \"passing\": 1-200,
    \"pace\": 1-200,
    \"stamina\": 1-200,
    \"strength\": 1-200
  }}
}}",
            position, skill_level, age
        )
    }

    /// Generate a prompt for creating team data
    ///
    /// # Arguments
    /// * `league_name` - The league where the team plays
    pub fn generate_team(league_name: &str) -> String {
        format!(
            "Generate a football team JSON data located in {}:

Return JSON format (return JSON only, no other text):
{{
  \"name\": \"Team Name\",
  \"abbreviation\": \"Abbreviation\",
  \"founded_year\": Year,
  \"stadium\": {{
    \"name\": \"Stadium Name\",
    \"capacity\": Capacity
  }}
}}",
            league_name
        )
    }

    /// Generate a prompt for match commentary
    ///
    /// # Arguments
    /// * `event` - The match event to comment on
    /// * `minute` - The minute when the event occurred
    /// * `teams` - The teams playing
    pub fn match_commentary(event: &str, minute: u8, teams: &str) -> String {
        format!(
            "You are a football commentator. Please generate a commentary line for this match event:

Event: {}
Time: {}' minute
Match: {}

Requirements: Short and punchy, 15-30 characters, return commentary only, no other content.",
            event, minute, teams
        )
    }

    /// Generate a prompt for news events
    ///
    /// # Arguments
    /// * `event_type` - The type of event (e.g., "transfer", "injury", "match")
    /// * `team_name` - The team involved in the event
    pub fn generate_news(event_type: &str, team_name: &str) -> String {
        format!(
            "You are a sports journalist. Please generate a news item for this event:

Event Type: {}
Team: {}

Requirements: Short title (10-20 characters) + body (50-100 characters), return news content only, no other text.",
            event_type, team_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_player_prompt() {
        let prompt = PromptTemplates::generate_player("ST", 150, 25);
        assert!(prompt.contains("ST"));
        assert!(prompt.contains("150"));
        assert!(prompt.contains("25"));
        assert!(prompt.contains("Player Name"));
        assert!(prompt.contains("finishing"));
    }

    #[test]
    fn test_generate_team_prompt() {
        let prompt = PromptTemplates::generate_team("Premier League");
        assert!(prompt.contains("Premier League"));
        assert!(prompt.contains("Team Name"));
        assert!(prompt.contains("stadium"));
    }

    #[test]
    fn test_match_commentary_prompt() {
        let prompt = PromptTemplates::match_commentary("Goal", 45, "Arsenal vs Chelsea");
        assert!(prompt.contains("Goal"));
        assert!(prompt.contains("45"));
        assert!(prompt.contains("Arsenal vs Chelsea"));
    }

    #[test]
    fn test_generate_news_prompt() {
        let prompt = PromptTemplates::generate_news("transfer", "Manchester United");
        assert!(prompt.contains("transfer"));
        assert!(prompt.contains("Manchester United"));
    }
}
