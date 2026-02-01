use crate::data::{DatabaseError, LineupRepository};
use crate::team::{PlayerSlot, Position, PlayerRole, Duty};
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of LineupRepository
pub struct SqliteLineupRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteLineupRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    fn parse_position(s: &str) -> Position {
        match s {
            "GK" => Position::GK,
            "CB" => Position::CB,
            "LB" => Position::LB,
            "RB" => Position::RB,
            "WB" => Position::WB,
            "DM" => Position::DM,
            "CM" => Position::CM,
            "AM" => Position::AM,
            "LW" => Position::LW,
            "RW" => Position::RW,
            "ST" => Position::ST,
            "CF" => Position::CF,
            _ => Position::ST,
        }
    }

    fn parse_role(s: &str) -> PlayerRole {
        match s {
            "Goalkeeper" => PlayerRole::Goalkeeper,
            "FullBack" => PlayerRole::FullBack,
            "WingBack" => PlayerRole::WingBack,
            "CentralDefender" => PlayerRole::CentralDefender,
            "DefensiveMidfielder" => PlayerRole::DefensiveMidfielder,
            "CentralMidfielder" => PlayerRole::CentralMidfielder,
            "WideMidfielder" => PlayerRole::WideMidfielder,
            "Winger" => PlayerRole::Winger,
            "AdvancedForward" => PlayerRole::AdvancedForward,
            "CompleteForward" => PlayerRole::CompleteForward,
            "DeepLyingForward" => PlayerRole::DeepLyingForward,
            "TargetMan" => PlayerRole::TargetMan,
            "Poacher" => PlayerRole::Poacher,
            "FalseNine" => PlayerRole::FalseNine,
            "Trequartista" => PlayerRole::Trequartista,
            "InsideForward" => PlayerRole::InsideForward,
            "WideTargetMan" => PlayerRole::WideTargetMan,
            "BoxToBox" => PlayerRole::BoxToBox,
            "BallWinningMidfielder" => PlayerRole::BallWinningMidfielder,
            "DeepLyingPlaymaker" => PlayerRole::DeepLyingPlaymaker,
            "Regista" => PlayerRole::Regista,
            "AdvancedPlaymaker" => PlayerRole::AdvancedPlaymaker,
            "CompleteWingBack" => PlayerRole::CompleteWingBack,
            "InvertedWingBack" => PlayerRole::InvertedWingBack,
            "BallPlayingDefender" => PlayerRole::BallPlayingDefender,
            "Libero" => PlayerRole::Libero,
            _ => PlayerRole::CentralMidfielder,
        }
    }

    fn parse_duty(s: &str) -> Duty {
        match s {
            "Defend" => Duty::Defend,
            "Support" => Duty::Support,
            "Attack" => Duty::Attack,
            "Stopper" => Duty::Stopper,
            "Cover" => Duty::Cover,
            _ => Duty::Support,
        }
    }
}

impl LineupRepository for SqliteLineupRepository {
    fn save_lineup(&self, team_id: &str, lineup: &[PlayerSlot], is_starting: bool) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        // Clear existing lineup for this team with the same is_starting value
        conn.execute(
            "DELETE FROM lineups WHERE team_id = ? AND is_starting = ?",
            params![team_id, is_starting],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        // Insert new lineup (id is auto-generated)
        for (index, slot) in lineup.iter().enumerate() {
            conn.execute(
                "INSERT INTO lineups (id, team_id, player_id, position, role, duty, is_starting, position_index) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    team_id,
                    slot.player_id,
                    format!("{:?}", slot.position),
                    format!("{:?}", slot.role),
                    format!("{:?}", slot.duty),
                    is_starting,
                    index as i32,
                ],
            ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        }

        Ok(())
    }

    fn get_starting_11(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, position, role, duty FROM lineups WHERE team_id = ? AND is_starting = 1 ORDER BY position_index"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let slots = stmt.query_map(params![team_id], |row| {
            Ok(PlayerSlot {
                player_id: row.get("player_id")?,
                position: Self::parse_position(&row.get::<_, String>("position")?),
                role: Self::parse_role(&row.get::<_, String>("role")?),
                duty: Self::parse_duty(&row.get::<_, String>("duty")?),
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(slots)
    }

    fn get_bench(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, position, role, duty FROM lineups WHERE team_id = ? AND is_starting = 0 ORDER BY position_index"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let slots = stmt.query_map(params![team_id], |row| {
            Ok(PlayerSlot {
                player_id: row.get("player_id")?,
                position: Self::parse_position(&row.get::<_, String>("position")?),
                role: Self::parse_role(&row.get::<_, String>("role")?),
                duty: Self::parse_duty(&row.get::<_, String>("duty")?),
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(slots)
    }

    fn clear_lineup(&self, team_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "DELETE FROM lineups WHERE team_id = ?",
            params![team_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;
    use crate::team::PlayerRole;
    use crate::team::Duty;

    fn create_test_lineup() -> Vec<PlayerSlot> {
        vec![
            PlayerSlot {
                player_id: "p1".to_string(),
                position: Position::GK,
                role: PlayerRole::Goalkeeper,
                duty: Duty::Defend,
            },
            PlayerSlot {
                player_id: "p2".to_string(),
                position: Position::CB,
                role: PlayerRole::CentralDefender,
                duty: Duty::Defend,
            },
        ]
    }

    #[test]
    fn test_save_and_get_lineup() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteLineupRepository::new(conn.clone());

        // Create league and team
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create players
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "GK", "", "Right", 185, 80, 5000, 3, 1000000],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p2", "team1", "Player 2", 24, "England", "CB", "", "Right", 188, 82, 8000, 4, 2000000],
        ).unwrap();

        let lineup = create_test_lineup();

        repo.save_lineup("team1", &lineup, true).unwrap();
        let starting_11 = repo.get_starting_11("team1").unwrap();

        assert_eq!(starting_11.len(), 2);
        assert_eq!(starting_11[0].player_id, "p1");
        assert_eq!(starting_11[1].player_id, "p2");
    }

    #[test]
    fn test_bench() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteLineupRepository::new(conn.clone());

        // Create league and team
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create players
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "GK", "", "Right", 185, 80, 5000, 3, 1000000],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p2", "team1", "Player 2", 24, "England", "CB", "", "Right", 188, 82, 8000, 4, 2000000],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p3", "team1", "Player 3", 26, "England", "ST", "", "Right", 182, 78, 15000, 3, 5000000],
        ).unwrap();

        let starting = create_test_lineup();
        let bench = vec![
            PlayerSlot {
                player_id: "p3".to_string(),
                position: Position::ST,
                role: PlayerRole::TargetMan,
                duty: Duty::Attack,
            },
        ];

        repo.save_lineup("team1", &starting, true).unwrap();
        repo.save_lineup("team1", &bench, false).unwrap();

        let starting_11 = repo.get_starting_11("team1").unwrap();
        assert_eq!(starting_11.len(), 2);

        let bench_players = repo.get_bench("team1").unwrap();
        assert_eq!(bench_players.len(), 1);
        assert_eq!(bench_players[0].player_id, "p3");
    }

    #[test]
    fn test_clear_lineup() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteLineupRepository::new(conn.clone());

        // Create league and team
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create players
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "GK", "", "Right", 185, 80, 5000, 3, 1000000],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p2", "team1", "Player 2", 24, "England", "CB", "", "Right", 188, 82, 8000, 4, 2000000],
        ).unwrap();

        let lineup = create_test_lineup();
        repo.save_lineup("team1", &lineup, true).unwrap();

        repo.clear_lineup("team1").unwrap();

        let starting_11 = repo.get_starting_11("team1").unwrap();
        assert_eq!(starting_11.len(), 0);
    }
}
