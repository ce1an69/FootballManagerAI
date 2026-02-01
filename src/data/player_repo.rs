use crate::data::{DatabaseError, PlayerRepository};
use crate::team::Player;
use crate::team::Position;
use crate::team::Foot;
use crate::team::PlayerStatus;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of PlayerRepository
pub struct SqlitePlayerRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqlitePlayerRepository {
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

    fn parse_foot(s: &str) -> Foot {
        match s {
            "Left" => Foot::Left,
            "Right" => Foot::Right,
            "Both" => Foot::Both,
            _ => Foot::Right,
        }
    }

    fn parse_status(s: &str) -> PlayerStatus {
        match s {
            "Healthy" => PlayerStatus::Healthy,
            "Injured" => PlayerStatus::Injured,
            "Fatigued" => PlayerStatus::Fatigued,
            "Suspended" => PlayerStatus::Suspended,
            _ => PlayerStatus::Healthy,
        }
    }

    fn parse_positions(s: &str) -> Vec<Position> {
        if s.is_empty() {
            Vec::new()
        } else {
            s.split(',')
                .filter_map(|pos_str| {
                    let trimmed = pos_str.trim();
                    if trimmed.starts_with("Position(") && trimmed.ends_with(')') {
                        let inner = &trimmed[9..trimmed.len()-1];
                        Some(Self::parse_position(inner))
                    } else {
                        Some(Self::parse_position(trimmed))
                    }
                })
                .collect()
        }
    }
}

impl PlayerRepository for SqlitePlayerRepository {
    fn create(&self, player: &Player) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT INTO players (
                id, team_id, name, age, nationality, position, second_positions, preferred_foot,
                height, weight,
                corners, crossing, dribbling, finishing, heading, long_shots, long_throws,
                marking, passing, penalties, tackling, technique,
                aggression, anticipation, bravery, creativity, decisions, concentration,
                positioning, off_the_ball, work_rate, pressure, teamwork, vision,
                acceleration, agility, balance, pace, stamina, strength,
                aerial_reach, command_of_area, communication, eccentricity, handling, kicking,
                throwing, reflexes, rushing_out, gk_positioning,
                potential_ability, current_ability, adaptability, ambition, professionalism,
                loyalty, injury_proneness, controversy, match_fitness, morale, status, injury_days,
                fatigue, wage, contract_years, market_value
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                player.id,
                player.team_id,
                player.name,
                player.age,
                player.nationality,
                format!("{:?}", player.position),
                player.second_positions.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>().join(","),
                format!("{:?}", player.preferred_foot),
                player.height,
                player.weight,
                player.corners,
                player.crossing,
                player.dribbling,
                player.finishing,
                player.heading,
                player.long_shots,
                player.long_throws,
                player.marking,
                player.passing,
                player.penalties,
                player.tackling,
                player.technique,
                player.aggression,
                player.anticipation,
                player.bravery,
                player.creativity,
                player.decisions,
                player.concentration,
                player.positioning,
                player.off_the_ball,
                player.work_rate,
                player.pressure,
                player.teamwork,
                player.vision,
                player.acceleration,
                player.agility,
                player.balance,
                player.pace,
                player.stamina,
                player.strength,
                player.aerial_reach,
                player.command_of_area,
                player.communication,
                player.eccentricity,
                player.handling,
                player.kicking,
                player.throwing,
                player.reflexes,
                player.rushing_out,
                player.gk_positioning,
                player.potential_ability,
                player.current_ability,
                player.adaptability,
                player.ambition,
                player.professionalism,
                player.loyalty,
                player.injury_proneness,
                player.controversy,
                player.match_fitness,
                player.morale,
                format!("{:?}", player.status),
                player.injury_days,
                player.fatigue,
                player.wage,
                player.contract_years,
                player.market_value,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<Player, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let player = conn.query_row(
            "SELECT * FROM players WHERE id = ?",
            params![id],
            |row| {
                let position_str: String = row.get("position")?;
                let second_positions_str: String = row.get("second_positions").unwrap_or_default();

                Ok(Player {
                    id: row.get("id")?,
                    team_id: row.get("team_id")?,
                    name: row.get("name")?,
                    age: row.get("age")?,
                    nationality: row.get("nationality")?,
                    position: Self::parse_position(&position_str),
                    second_positions: Self::parse_positions(&second_positions_str),
                    preferred_foot: Self::parse_foot(&row.get::<_, String>("preferred_foot").unwrap_or_default()),
                    height: row.get("height")?,
                    weight: row.get("weight")?,
                    corners: row.get("corners")?,
                    crossing: row.get("crossing")?,
                    dribbling: row.get("dribbling")?,
                    finishing: row.get("finishing")?,
                    heading: row.get("heading")?,
                    long_shots: row.get("long_shots")?,
                    long_throws: row.get("long_throws")?,
                    marking: row.get("marking")?,
                    passing: row.get("passing")?,
                    penalties: row.get("penalties")?,
                    tackling: row.get("tackling")?,
                    technique: row.get("technique")?,
                    aggression: row.get("aggression")?,
                    anticipation: row.get("anticipation")?,
                    bravery: row.get("bravery")?,
                    creativity: row.get("creativity")?,
                    decisions: row.get("decisions")?,
                    concentration: row.get("concentration")?,
                    positioning: row.get("positioning")?,
                    off_the_ball: row.get("off_the_ball")?,
                    work_rate: row.get("work_rate")?,
                    pressure: row.get("pressure")?,
                    teamwork: row.get("teamwork")?,
                    vision: row.get("vision")?,
                    acceleration: row.get("acceleration")?,
                    agility: row.get("agility")?,
                    balance: row.get("balance")?,
                    pace: row.get("pace")?,
                    stamina: row.get("stamina")?,
                    strength: row.get("strength")?,
                    aerial_reach: row.get("aerial_reach")?,
                    command_of_area: row.get("command_of_area")?,
                    communication: row.get("communication")?,
                    eccentricity: row.get("eccentricity")?,
                    handling: row.get("handling")?,
                    kicking: row.get("kicking")?,
                    throwing: row.get("throwing")?,
                    reflexes: row.get("reflexes")?,
                    rushing_out: row.get("rushing_out")?,
                    gk_positioning: row.get("gk_positioning")?,
                    potential_ability: row.get("potential_ability")?,
                    current_ability: row.get("current_ability")?,
                    adaptability: row.get("adaptability")?,
                    ambition: row.get("ambition")?,
                    professionalism: row.get("professionalism")?,
                    loyalty: row.get("loyalty")?,
                    injury_proneness: row.get("injury_proneness")?,
                    controversy: row.get("controversy")?,
                    match_fitness: row.get("match_fitness")?,
                    morale: row.get("morale")?,
                    status: Self::parse_status(&row.get::<_, String>("status").unwrap_or_default()),
                    injury_days: row.get("injury_days")?,
                    fatigue: row.get("fatigue")?,
                    wage: row.get("wage")?,
                    contract_years: row.get("contract_years")?,
                    market_value: row.get("market_value")?,
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        Ok(player)
    }

    fn get_by_team(&self, team_id: &str) -> Result<Vec<Player>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare("SELECT id FROM players WHERE team_id = ? ORDER BY name")
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let player_ids = stmt.query_map(params![team_id], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut players = Vec::new();
        for player_id in player_ids {
            players.push(self.get_by_id(&player_id)?);
        }

        Ok(players)
    }

    fn update(&self, player: &Player) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE players SET
                team_id = ?, name = ?, age = ?, nationality = ?, position = ?,
                preferred_foot = ?, height = ?, weight = ?,
                corners = ?, crossing = ?, dribbling = ?, finishing = ?, heading = ?,
                long_shots = ?, long_throws = ?, marking = ?, passing = ?, penalties = ?,
                tackling = ?, technique = ?, aggression = ?, anticipation = ?, bravery = ?,
                creativity = ?, decisions = ?, concentration = ?, positioning = ?,
                off_the_ball = ?, work_rate = ?, pressure = ?, teamwork = ?, vision = ?,
                acceleration = ?, agility = ?, balance = ?, pace = ?, stamina = ?,
                strength = ?, aerial_reach = ?, command_of_area = ?, communication = ?,
                eccentricity = ?, handling = ?, kicking = ?, throwing = ?, reflexes = ?,
                rushing_out = ?, gk_positioning = ?, potential_ability = ?, current_ability = ?,
                adaptability = ?, ambition = ?, professionalism = ?, loyalty = ?,
                injury_proneness = ?, controversy = ?, match_fitness = ?, morale = ?,
                status = ?, injury_days = ?, fatigue = ?, wage = ?, contract_years = ?, market_value = ?,
                second_positions = ?
            WHERE id = ?",
            params![
                player.team_id,
                player.name,
                player.age,
                player.nationality,
                format!("{:?}", player.position),
                format!("{:?}", player.preferred_foot),
                player.height,
                player.weight,
                player.corners,
                player.crossing,
                player.dribbling,
                player.finishing,
                player.heading,
                player.long_shots,
                player.long_throws,
                player.marking,
                player.passing,
                player.penalties,
                player.tackling,
                player.technique,
                player.aggression,
                player.anticipation,
                player.bravery,
                player.creativity,
                player.decisions,
                player.concentration,
                player.positioning,
                player.off_the_ball,
                player.work_rate,
                player.pressure,
                player.teamwork,
                player.vision,
                player.acceleration,
                player.agility,
                player.balance,
                player.pace,
                player.stamina,
                player.strength,
                player.aerial_reach,
                player.command_of_area,
                player.communication,
                player.eccentricity,
                player.handling,
                player.kicking,
                player.throwing,
                player.reflexes,
                player.rushing_out,
                player.gk_positioning,
                player.potential_ability,
                player.current_ability,
                player.adaptability,
                player.ambition,
                player.professionalism,
                player.loyalty,
                player.injury_proneness,
                player.controversy,
                player.match_fitness,
                player.morale,
                format!("{:?}", player.status),
                player.injury_days,
                player.fatigue,
                player.wage,
                player.contract_years,
                player.market_value,
                player.second_positions.iter().map(|p| format!("{:?}", p)).collect::<Vec<_>>().join(","),
                player.id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn delete(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute("DELETE FROM players WHERE id = ?", params![id])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_free_agents(&self) -> Result<Vec<Player>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare("SELECT id FROM players WHERE team_id IS NULL ORDER BY name")
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let player_ids = stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut players = Vec::new();
        for player_id in player_ids {
            players.push(self.get_by_id(&player_id)?);
        }

        Ok(players)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    fn create_test_player(id: &str, name: &str) -> Player {
        let mut player = Player::new(
            id.to_string(),
            name.to_string(),
            Position::ST,
        );
        player.team_id = Some("team1".to_string());
        player
    }

    #[test]
    fn test_create_and_get_player() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqlitePlayerRepository::new(conn.clone());

        // Create league first (required by foreign key constraint)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create team
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let player = create_test_player("player1", "Test Player");

        repo.create(&player).unwrap();
        let retrieved = repo.get_by_id("player1").unwrap();

        assert_eq!(retrieved.id, "player1");
        assert_eq!(retrieved.name, "Test Player");
        assert_eq!(retrieved.team_id, Some("team1".to_string()));
    }

    #[test]
    fn test_get_by_team() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqlitePlayerRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create team
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let player1 = create_test_player("player1", "Player 1");
        let player2 = create_test_player("player2", "Player 2");

        repo.create(&player1).unwrap();
        repo.create(&player2).unwrap();

        let players = repo.get_by_team("team1").unwrap();
        assert_eq!(players.len(), 2);
    }

    #[test]
    fn test_update_player() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqlitePlayerRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create team
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let mut player = create_test_player("player1", "Test Player");
        repo.create(&player).unwrap();

        player.name = "Updated Player".to_string();
        player.age = 25;
        repo.update(&player).unwrap();

        let retrieved = repo.get_by_id("player1").unwrap();
        assert_eq!(retrieved.name, "Updated Player");
        assert_eq!(retrieved.age, 25);
    }

    #[test]
    fn test_delete_player() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqlitePlayerRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create team
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let player = create_test_player("player1", "Test Player");
        repo.create(&player).unwrap();

        repo.delete("player1").unwrap();

        let result = repo.get_by_id("player1");
        assert!(matches!(result, Err(DatabaseError::NotFound(_))));
    }

    #[test]
    fn test_get_free_agents() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqlitePlayerRepository::new(conn.clone());

        // Create league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create team
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create player with team
        let player1 = create_test_player("player1", "Player 1");
        repo.create(&player1).unwrap();

        // Create free agent
        let player2 = Player::new("player2".to_string(), "Free Agent".to_string(), Position::ST);
        // team_id is None by default for free agent
        repo.create(&player2).unwrap();

        let free_agents = repo.get_free_agents().unwrap();
        assert_eq!(free_agents.len(), 1);
        assert_eq!(free_agents[0].id, "player2");
    }
}
