use crate::data::DatabaseError;
use crate::transfer::{TransferHistoryEntry, TransferHistoryType};
use rusqlite::params;

/// Transfer history repository trait
pub trait TransferHistoryRepository {
    fn create(&self, entry: &TransferHistoryEntry) -> Result<(), DatabaseError>;
    fn get_by_player(&self, player_id: &str) -> Result<Vec<TransferHistoryEntry>, DatabaseError>;
    fn get_by_team(&self, team_id: &str) -> Result<Vec<TransferHistoryEntry>, DatabaseError>;
    fn get_recent(&self, limit: usize) -> Result<Vec<TransferHistoryEntry>, DatabaseError>;
    fn calculate_spending(&self, team_id: &str) -> Result<u32, DatabaseError>;
    fn calculate_income(&self, team_id: &str) -> Result<u32, DatabaseError>;
}

/// SQLite implementation of transfer history repository
pub struct SqliteTransferHistoryRepository {
    conn: std::sync::Arc<std::sync::RwLock<rusqlite::Connection>>,
}

impl SqliteTransferHistoryRepository {
    pub fn new(conn: std::sync::Arc<std::sync::RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    /// Parse transfer type from string
    fn parse_transfer_type(s: &str) -> Result<TransferHistoryType, DatabaseError> {
        match s {
            "Purchase" => Ok(TransferHistoryType::Purchase),
            "Sale" => Ok(TransferHistoryType::Sale),
            "Loan" => Ok(TransferHistoryType::Loan),
            "Release" => Ok(TransferHistoryType::Release),
            "FreeTransfer" => Ok(TransferHistoryType::FreeTransfer),
            "Exchange" => Ok(TransferHistoryType::Exchange),
            _ => Err(DatabaseError::QueryError(format!("Unknown transfer type: {}", s))),
        }
    }

    /// Serialize transfer type to string
    fn serialize_transfer_type(transfer_type: &TransferHistoryType) -> &'static str {
        match transfer_type {
            TransferHistoryType::Purchase => "Purchase",
            TransferHistoryType::Sale => "Sale",
            TransferHistoryType::Loan => "Loan",
            TransferHistoryType::Release => "Release",
            TransferHistoryType::FreeTransfer => "FreeTransfer",
            TransferHistoryType::Exchange => "Exchange",
        }
    }
}

impl TransferHistoryRepository for SqliteTransferHistoryRepository {
    fn create(&self, entry: &TransferHistoryEntry) -> Result<(), DatabaseError> {
        self.conn.write().unwrap().execute(
            "INSERT INTO transfer_history (id, player_id, player_name, from_team_id, to_team_id, transfer_type, fee, date)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                entry.id,
                entry.player_id,
                entry.player_name,
                entry.from_team_id,
                entry.to_team_id,
                Self::serialize_transfer_type(&entry.transfer_type),
                entry.fee,
                entry.date,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_player(&self, player_id: &str) -> Result<Vec<TransferHistoryEntry>, DatabaseError> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, player_id, player_name, from_team_id, to_team_id, transfer_type, fee, date
             FROM transfer_history
             WHERE player_id = ?1
             ORDER BY date DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let entries = stmt.query_map(params![player_id], |row| {
            Ok(TransferHistoryEntry {
                id: row.get(0)?,
                player_id: row.get(1)?,
                player_name: row.get(2)?,
                from_team_id: row.get(3)?,
                to_team_id: row.get(4)?,
                transfer_type: Self::parse_transfer_type(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                fee: row.get(6)?,
                date: row.get(7)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(entries)
    }

    fn get_by_team(&self, team_id: &str) -> Result<Vec<TransferHistoryEntry>, DatabaseError> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, player_id, player_name, from_team_id, to_team_id, transfer_type, fee, date
             FROM transfer_history
             WHERE from_team_id = ?1 OR to_team_id = ?1
             ORDER BY date DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let entries = stmt.query_map(params![team_id], |row| {
            Ok(TransferHistoryEntry {
                id: row.get(0)?,
                player_id: row.get(1)?,
                player_name: row.get(2)?,
                from_team_id: row.get(3)?,
                to_team_id: row.get(4)?,
                transfer_type: Self::parse_transfer_type(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                fee: row.get(6)?,
                date: row.get(7)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(entries)
    }

    fn get_recent(&self, limit: usize) -> Result<Vec<TransferHistoryEntry>, DatabaseError> {
        let conn = self.conn.read().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, player_id, player_name, from_team_id, to_team_id, transfer_type, fee, date
             FROM transfer_history
             ORDER BY date DESC
             LIMIT ?1"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let entries = stmt.query_map(params![limit as i64], |row| {
            Ok(TransferHistoryEntry {
                id: row.get(0)?,
                player_id: row.get(1)?,
                player_name: row.get(2)?,
                from_team_id: row.get(3)?,
                to_team_id: row.get(4)?,
                transfer_type: Self::parse_transfer_type(&row.get::<_, String>(5)?)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                fee: row.get(6)?,
                date: row.get(7)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(entries)
    }

    fn calculate_spending(&self, team_id: &str) -> Result<u32, DatabaseError> {
        let spending: u32 = self.conn.read().unwrap().query_row(
            "SELECT COALESCE(SUM(fee), 0) FROM transfer_history
             WHERE from_team_id = ?1 AND transfer_type IN ('Purchase', 'Exchange')",
            params![team_id],
            |row| row.get(0),
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(spending)
    }

    fn calculate_income(&self, team_id: &str) -> Result<u32, DatabaseError> {
        let income: u32 = self.conn.read().unwrap().query_row(
            "SELECT COALESCE(SUM(fee), 0) FROM transfer_history
             WHERE to_team_id = ?1 AND transfer_type IN ('Sale', 'Exchange')",
            params![team_id],
            |row| row.get(0),
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(income)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_transfer_type() {
        assert_eq!(SqliteTransferHistoryRepository::serialize_transfer_type(&TransferHistoryType::Purchase), "Purchase");
        assert_eq!(SqliteTransferHistoryRepository::serialize_transfer_type(&TransferHistoryType::Sale), "Sale");
    }

    #[test]
    fn test_parse_transfer_type() {
        assert!(matches!(SqliteTransferHistoryRepository::parse_transfer_type("Purchase").unwrap(), TransferHistoryType::Purchase));
        assert!(matches!(SqliteTransferHistoryRepository::parse_transfer_type("Sale").unwrap(), TransferHistoryType::Sale));
        assert!(SqliteTransferHistoryRepository::parse_transfer_type("Invalid").is_err());
    }
}
