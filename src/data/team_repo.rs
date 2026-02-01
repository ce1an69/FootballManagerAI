use crate::data::{DatabaseError, TeamRepository};
use crate::team::{Team, TeamStatistics};
use crate::team::Tactic;
use rusqlite::{params, Row};
use std::sync::{Arc, RwLock};

/// SQLite implementation of TeamRepository
pub struct SqliteTeamRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteTeamRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    fn parse_formation(s: &str) -> crate::team::Formation {
        match s {
            "4-4-2" => crate::team::Formation::FourFourTwo,
            "4-3-3" => crate::team::Formation::FourThreeThree,
            "4-2-3-1" => crate::team::Formation::FourTwoThreeOne,
            "4-1-4-1" => crate::team::Formation::FourOneFourOne,
            "4-5-1" => crate::team::Formation::FourFiveOne,
            "3-5-2" => crate::team::Formation::ThreeFiveTwo,
            "3-4-2-1" => crate::team::Formation::ThreeFourTwoOne,
            "5-3-2" => crate::team::Formation::FiveThreeTwo,
            "5-2-2-1" => crate::team::Formation::FiveTwoTwoOne,
            _ => crate::team::Formation::FourFourTwo,
        }
    }

    fn parse_defensive_height(s: &str) -> crate::team::DefensiveHeight {
        match s {
            "Low" => crate::team::DefensiveHeight::Low,
            "Medium" => crate::team::DefensiveHeight::Medium,
            "High" => crate::team::DefensiveHeight::High,
            _ => crate::team::DefensiveHeight::Medium,
        }
    }

    fn parse_passing_style(s: &str) -> crate::team::PassingStyle {
        match s {
            "Short" => crate::team::PassingStyle::Short,
            "Mixed" => crate::team::PassingStyle::Mixed,
            "Long" => crate::team::PassingStyle::Long,
            _ => crate::team::PassingStyle::Mixed,
        }
    }

    fn parse_tempo(s: &str) -> crate::team::Tempo {
        match s {
            "Slow" => crate::team::Tempo::Slow,
            "Medium" => crate::team::Tempo::Medium,
            "Fast" => crate::team::Tempo::Fast,
            _ => crate::team::Tempo::Medium,
        }
    }
}

impl TeamRepository for SqliteTeamRepository {
    fn create(&self, team: &Team) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        // Insert team
        conn.execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                team.id,
                team.name,
                team.league_id,
                team.budget,
                team.tactic.formation.name(),
                team.tactic.attacking_mentality,
                format!("{:?}", team.tactic.defensive_height),
                format!("{:?}", team.tactic.passing_style),
                format!("{:?}", team.tactic.tempo),
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        // Create empty statistics (if not exists)
        let _ = conn.execute(
            "INSERT OR IGNORE INTO team_statistics (team_id, matches_played, wins, draws, losses, goals_for, goals_against, points)
             VALUES (?, 0, 0, 0, 0, 0, 0, 0)",
            params![team.id],
        );

        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<Team, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let team = conn.query_row(
            "SELECT * FROM teams WHERE id = ?",
            params![id],
            |row| {
                let id: String = row.get("id")?;
                let name: String = row.get("name")?;
                let league_id: String = row.get("league_id")?;
                let budget: u32 = row.get("budget")?;

                // Load tactic
                let formation_str: String = row.get("formation")?;
                let attacking_mentality: u8 = row.get("attacking_mentality")?;
                let defensive_height_str: String = row.get("defensive_height")?;
                let passing_style_str: String = row.get("passing_style")?;
                let tempo_str: String = row.get("tempo")?;

                let tactic = Tactic {
                    formation: Self::parse_formation(&formation_str),
                    attacking_mentality,
                    defensive_height: Self::parse_defensive_height(&defensive_height_str),
                    passing_style: Self::parse_passing_style(&passing_style_str),
                    tempo: Self::parse_tempo(&tempo_str),
                    player_roles: Vec::new(),
                };

                let players: Vec<String> = Vec::new(); // Will be loaded from players table
                let starting_11 = Vec::new();
                let bench = Vec::new();
                let statistics = TeamStatistics::default();

                Ok(Team {
                    id,
                    name,
                    league_id,
                    budget,
                    players,
                    starting_11,
                    bench,
                    tactic,
                    statistics,
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        Ok(team)
    }

    fn get_all(&self) -> Result<Vec<Team>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare("SELECT id FROM teams ORDER BY name")
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let team_ids = stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut teams = Vec::new();
        for team_id in team_ids {
            teams.push(self.get_by_id(&team_id)?);
        }

        Ok(teams)
    }

    fn update(&self, team: &Team) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE teams SET name = ?, budget = ?, formation = ?, attacking_mentality = ?,
             defensive_height = ?, passing_style = ?, tempo = ? WHERE id = ?",
            params![
                team.name,
                team.budget,
                team.tactic.formation.name(),
                team.tactic.attacking_mentality,
                format!("{:?}", team.tactic.defensive_height),
                format!("{:?}", team.tactic.passing_style),
                format!("{:?}", team.tactic.tempo),
                team.id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute("DELETE FROM teams WHERE id = ?", params![id])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_league(&self, league_id: &str) -> Result<Vec<Team>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare("SELECT id FROM teams WHERE league_id = ? ORDER BY name")
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let team_ids = stmt.query_map(params![league_id], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut teams = Vec::new();
        for team_id in team_ids {
            teams.push(self.get_by_id(&team_id)?);
        }

        Ok(teams)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    #[test]
    fn test_create_and_get_team() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamRepository::new(conn.clone());

        // Create league first (required by foreign key constraint)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        let team = Team::new("team1".to_string(), "Test Team".to_string(), "league1".to_string(), 1000000);

        repo.create(&team).unwrap();
        let retrieved = repo.get_by_id("team1").unwrap();

        assert_eq!(retrieved.id, "team1");
        assert_eq!(retrieved.name, "Test Team");
        assert_eq!(retrieved.budget, 1000000);
    }

    #[test]
    fn test_get_all_teams() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        let team1 = Team::new("team1".to_string(), "Team A".to_string(), "league1".to_string(), 1000000);
        let team2 = Team::new("team2".to_string(), "Team B".to_string(), "league1".to_string(), 2000000);

        repo.create(&team1).unwrap();
        repo.create(&team2).unwrap();

        let teams = repo.get_all().unwrap();
        assert_eq!(teams.len(), 2);
    }

    #[test]
    fn test_update_team() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        let mut team = Team::new("team1".to_string(), "Test Team".to_string(), "league1".to_string(), 1000000);
        repo.create(&team).unwrap();

        team.budget = 2000000;
        team.name = "Updated Team".to_string();
        repo.update(&team).unwrap();

        let retrieved = repo.get_by_id("team1").unwrap();
        assert_eq!(retrieved.budget, 2000000);
        assert_eq!(retrieved.name, "Updated Team");
    }

    #[test]
    fn test_delete_team() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        let team = Team::new("team1".to_string(), "Test Team".to_string(), "league1".to_string(), 1000000);
        repo.create(&team).unwrap();

        repo.delete("team1").unwrap();

        let result = repo.get_by_id("team1");
        assert!(matches!(result, Err(DatabaseError::NotFound(_))));
    }

    #[test]
    fn test_get_by_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamRepository::new(conn.clone());

        // Create leagues first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league2", "League 2", 0, 38],
        ).unwrap();

        let team1 = Team::new("team1".to_string(), "Team A".to_string(), "league1".to_string(), 1000000);
        let team2 = Team::new("team2".to_string(), "Team B".to_string(), "league1".to_string(), 2000000);
        let team3 = Team::new("team3".to_string(), "Team C".to_string(), "league2".to_string(), 3000000);

        repo.create(&team1).unwrap();
        repo.create(&team2).unwrap();
        repo.create(&team3).unwrap();

        let league1_teams = repo.get_by_league("league1").unwrap();
        assert_eq!(league1_teams.len(), 2);

        let league2_teams = repo.get_by_league("league2").unwrap();
        assert_eq!(league2_teams.len(), 1);
    }
}
