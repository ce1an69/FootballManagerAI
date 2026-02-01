/// Database index definitions for performance optimization
///
/// This module defines all database indexes to optimize query performance.
/// Indexes should be created after table creation in migrations.
use rusqlite::Connection;

/// Create all database indexes for performance optimization
pub fn create_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Players table indexes
    create_players_indexes(conn)?;

    // Teams table indexes
    create_teams_indexes(conn)?;

    // Matches table indexes
    create_matches_indexes(conn)?;

    // Scheduled matches table indexes
    create_scheduled_matches_indexes(conn)?;

    // Transfer history table indexes
    create_transfer_history_indexes(conn)?;

    // Finance tables indexes
    create_finance_indexes(conn)?;

    // Player stats indexes
    create_player_stats_indexes(conn)?;

    Ok(())
}

/// Create indexes for players table
fn create_players_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for team_id lookups (very frequent)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_players_team_id ON players(team_id)",
        [],
    )?;

    // Index for free agents (team_id IS NULL)
    // Note: Partial indexes are more efficient for NULL checks
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_players_free_agents ON players(id) WHERE team_id IS NULL",
        [],
    )?;

    // Index for position filtering (transfer market search)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_players_position ON players(position)",
        [],
    )?;

    // Composite index for team + name (common query pattern)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_players_team_name ON players(team_id, name)",
        [],
    )?;

    // Index for age-based queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_players_age ON players(age)",
        [],
    )?;

    Ok(())
}

/// Create indexes for teams table
fn create_teams_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for league_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_teams_league_id ON teams(league_id)",
        [],
    )?;

    // Index for name sorting
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_teams_name ON teams(name)",
        [],
    )?;

    // Composite index for league + name
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_teams_league_name ON teams(league_id, name)",
        [],
    )?;

    Ok(())
}

/// Create indexes for matches table
fn create_matches_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for league_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_league_id ON matches(league_id)",
        [],
    )?;

    // Index for home team lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_home_team ON matches(home_team_id)",
        [],
    )?;

    // Index for away team lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_away_team ON matches(away_team_id)",
        [],
    )?;

    // Index for round-based queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_round ON matches(round)",
        [],
    )?;

    // Composite index for league + round (common query pattern)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_league_round ON matches(league_id, round)",
        [],
    )?;

    // Index for played_at (date-based queries)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_matches_played_at ON matches(played_at)",
        [],
    )?;

    Ok(())
}

/// Create indexes for scheduled_matches table
fn create_scheduled_matches_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for league_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scheduled_matches_league_id ON scheduled_matches(league_id)",
        [],
    )?;

    // Index for round_number queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scheduled_matches_round ON scheduled_matches(round_number)",
        [],
    )?;

    // Composite index for league + round (very common query)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scheduled_matches_league_round ON scheduled_matches(league_id, round_number)",
        [],
    )?;

    // Index for played flag
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scheduled_matches_played ON scheduled_matches(played)",
        [],
    )?;

    // Composite index for finding unplayed matches in a round
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_scheduled_matches_league_round_unplayed ON scheduled_matches(league_id, round_number, played) WHERE played = 0",
        [],
    )?;

    Ok(())
}

/// Create indexes for transfer_history table
fn create_transfer_history_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for player_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transfer_history_player_id ON transfer_history(player_id)",
        [],
    )?;

    // Index for from_team_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transfer_history_from_team ON transfer_history(from_team_id)",
        [],
    )?;

    // Index for to_team_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transfer_history_to_team ON transfer_history(to_team_id)",
        [],
    )?;

    // Index for date-based sorting (most recent transfers)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transfer_history_date ON transfer_history(date DESC)",
        [],
    )?;

    // Index for transfer_type filtering
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_transfer_history_type ON transfer_history(transfer_type)",
        [],
    )?;

    Ok(())
}

/// Create indexes for finance tables
fn create_finance_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for finance_transactions team_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_finance_transactions_team ON finance_transactions(team_id)",
        [],
    )?;

    // Index for finance_transactions date queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_finance_transactions_date ON finance_transactions(date_year, date_month, date_day)",
        [],
    )?;

    // Index for finance_transactions type queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_finance_transactions_type ON finance_transactions(transaction_type)",
        [],
    )?;

    // Index for season_finance_reports season lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_season_reports_season ON season_finance_reports(season)",
        [],
    )?;

    // Index for season_finance_reports team lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_season_reports_team ON season_finance_reports(team_id)",
        [],
    )?;

    Ok(())
}

/// Create indexes for player_season_stats table
fn create_player_stats_indexes(conn: &Connection) -> Result<(), rusqlite::Error> {
    // Index for player_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_player ON player_season_stats(player_id)",
        [],
    )?;

    // Index for season lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_season ON player_season_stats(season)",
        [],
    )?;

    // Index for team_id lookups
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_team ON player_season_stats(team_id)",
        [],
    )?;

    // Composite index for season + goals (leaderboard queries)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_goals ON player_season_stats(season, goals DESC)",
        [],
    )?;

    // Composite index for season + assists (leaderboard queries)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_assists ON player_season_stats(season, assists DESC)",
        [],
    )?;

    // Composite index for team + season (squad stats queries)
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_player_stats_team_season ON player_season_stats(team_id, season)",
        [],
    )?;

    Ok(())
}

/// Analyze query performance and verify indexes are being used
///
/// This should be called in development/testing to verify index effectiveness
#[cfg(debug_assertions)]
pub fn analyze_index_usage(conn: &Connection) -> Result<Vec<IndexStats>, rusqlite::Error> {
    // Get index statistics
    let mut stmt = conn.prepare(
        "SELECT name, tbl_name
         FROM sqlite_master
         WHERE type = 'index'
         AND name LIKE 'idx_%'
         ORDER BY tbl_name, name"
    )?;

    let indexes = stmt.query_map([], |row| {
        Ok(IndexStats {
            name: row.get(0)?,
            table_name: row.get(1)?,
        })
    })?.collect::<Result<Vec<_>, _>>()?;

    Ok(indexes)
}

/// Index statistics information
#[derive(Debug, Clone)]
pub struct IndexStats {
    pub name: String,
    pub table_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_indexes() {
        let conn = Connection::open_in_memory().unwrap();

        // Create tables first
        conn.execute(
            "CREATE TABLE players (
                id TEXT PRIMARY KEY,
                team_id TEXT,
                name TEXT,
                position TEXT,
                age INTEGER
            )",
            [],
        ).unwrap();

        conn.execute(
            "CREATE TABLE teams (
                id TEXT PRIMARY KEY,
                league_id TEXT,
                name TEXT
            )",
            [],
        ).unwrap();

        // Create indexes
        create_players_indexes(&conn).unwrap();
        create_teams_indexes(&conn).unwrap();

        // Verify indexes exist
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'index' AND name LIKE 'idx_%'").unwrap();
        let index_names: Vec<String> = stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(index_names.iter().any(|n| n.contains("idx_players_team_id")));
        assert!(index_names.iter().any(|n| n.contains("idx_teams_league_id")));
    }

    #[test]
    fn test_analyze_index_usage() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            "CREATE TABLE test_table (id TEXT PRIMARY KEY, value TEXT)",
            [],
        ).unwrap();

        conn.execute(
            "CREATE INDEX idx_test_value ON test_table(value)",
            [],
        ).unwrap();

        let stats = analyze_index_usage(&conn).unwrap();
        assert!(!stats.is_empty());
    }
}
