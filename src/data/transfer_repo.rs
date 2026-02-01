use crate::data::{DatabaseError, TransferMarketRepository, MarketListing, PlayerRepository};
use crate::team::Player;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of TransferMarketRepository
pub struct SqliteTransferMarketRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
    player_repo: Arc<dyn PlayerRepository>,
}

impl SqliteTransferMarketRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>, player_repo: Arc<dyn PlayerRepository>) -> Self {
        Self { conn, player_repo }
    }
}

impl TransferMarketRepository for SqliteTransferMarketRepository {
    fn add_to_market(&self, player_id: &str, price: u32) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        // Get current timestamp
        let listed_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        conn.execute(
            "INSERT INTO transfer_market (player_id, asking_price, listed_at) VALUES (?, ?, ?)",
            params![player_id, price, listed_at],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn remove_from_market(&self, player_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "DELETE FROM transfer_market WHERE player_id = ?",
            params![player_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_market_players(&self) -> Result<Vec<Player>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id FROM transfer_market ORDER BY listed_at DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let player_ids = stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut players = Vec::new();
        for player_id in player_ids {
            match self.player_repo.get_by_id(&player_id) {
                Ok(player) => players.push(player),
                Err(DatabaseError::NotFound(_)) => {
                    // Player no longer exists, skip
                    continue;
                },
                Err(e) => return Err(e),
            }
        }

        Ok(players)
    }

    fn get_market_listing(&self, player_id: &str) -> Result<Option<MarketListing>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let listing = conn.query_row(
            "SELECT player_id, asking_price, listed_at, reason FROM transfer_market WHERE player_id = ?",
            params![player_id],
            |row| {
                Ok(MarketListing {
                    player_id: row.get("player_id")?,
                    asking_price: row.get("asking_price")?,
                    listed_at: row.get("listed_at")?,
                    reason: row.get::<_, String>("reason").ok(),
                })
            },
        );

        match listing {
            Ok(l) => Ok(Some(l)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(DatabaseError::QueryError(e.to_string())),
        }
    }

    fn update_price(&self, player_id: &str, new_price: u32) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE transfer_market SET asking_price = ? WHERE player_id = ?",
            params![new_price, player_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;
    use crate::data::PlayerRepository;
    use crate::data::SqlitePlayerRepository;

    #[test]
    fn test_add_and_get_market_listing() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let player_repo = Arc::new(SqlitePlayerRepository::new(conn.clone())) as Arc<dyn PlayerRepository>;
        let repo = SqliteTransferMarketRepository::new(conn.clone(), player_repo);

        // Create league and team
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create player
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "ST", "", "Right", 182, 78, 15000, 3, 5000000],
        ).unwrap();

        // Add to market
        repo.add_to_market("p1", 1000000).unwrap();

        // Get listing
        let listing = repo.get_market_listing("p1").unwrap();
        assert!(listing.is_some());
        assert_eq!(listing.as_ref().unwrap().asking_price, 1000000);
        assert_eq!(listing.as_ref().unwrap().player_id, "p1");
    }

    #[test]
    fn test_remove_from_market() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let player_repo = Arc::new(SqlitePlayerRepository::new(conn.clone())) as Arc<dyn PlayerRepository>;
        let repo = SqliteTransferMarketRepository::new(conn.clone(), player_repo);

        // Create league, team and player
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "ST", "", "Right", 182, 78, 15000, 3, 5000000],
        ).unwrap();

        // Add to market
        repo.add_to_market("p1", 1000000).unwrap();

        // Remove from market
        repo.remove_from_market("p1").unwrap();

        // Verify removal
        let listing = repo.get_market_listing("p1").unwrap();
        assert!(listing.is_none());
    }

    #[test]
    fn test_update_price() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let player_repo = Arc::new(SqlitePlayerRepository::new(conn.clone())) as Arc<dyn PlayerRepository>;
        let repo = SqliteTransferMarketRepository::new(conn.clone(), player_repo);

        // Create league, team and player
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "ST", "", "Right", 182, 78, 15000, 3, 5000000],
        ).unwrap();

        // Add to market
        repo.add_to_market("p1", 1000000).unwrap();

        // Update price
        repo.update_price("p1", 1500000).unwrap();

        // Verify update
        let listing = repo.get_market_listing("p1").unwrap();
        assert!(listing.is_some());
        assert_eq!(listing.as_ref().unwrap().asking_price, 1500000);
    }

    #[test]
    fn test_get_market_players() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let player_repo = Arc::new(SqlitePlayerRepository::new(conn.clone())) as Arc<dyn PlayerRepository>;
        let repo = SqliteTransferMarketRepository::new(conn.clone(), player_repo);

        // Create league, team and players
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p1", "team1", "Player 1", 25, "England", "ST", "", "Right", 182, 78, 15000, 3, 5000000],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, second_positions, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["p2", "team1", "Player 2", 24, "England", "ST", "", "Right", 180, 76, 12000, 2, 3000000],
        ).unwrap();

        // Add both to market
        repo.add_to_market("p1", 1000000).unwrap();
        repo.add_to_market("p2", 2000000).unwrap();

        // Get market players
        let market_players = repo.get_market_players().unwrap();
        assert_eq!(market_players.len(), 2);
    }
}
