use crate::data::{DatabaseError, SeasonFinanceReportRepository};
use crate::team::SeasonFinanceReport;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of SeasonFinanceReportRepository
pub struct SqliteSeasonFinanceReportRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteSeasonFinanceReportRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl SeasonFinanceReportRepository for SqliteSeasonFinanceReportRepository {
    fn get(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError> {
        let conn = self.conn.read().unwrap();

        conn.query_row(
            "SELECT season, team_id, transfer_income, matchday_income, sponsorship, tv_revenue, prize_money,
                    transfer_expenses, wage_expenses, bonus_expenses, staff_wages, facilities, youth_academy
             FROM season_finance_reports
             WHERE season = ? AND team_id = ?",
            params![season, team_id],
            |row| {
                let mut report = SeasonFinanceReport {
                    season: row.get(0)?,
                    team_id: row.get(1)?,
                    transfer_income: row.get(2)?,
                    matchday_income: row.get(3)?,
                    sponsorship: row.get(4)?,
                    tv_revenue: row.get(5)?,
                    prize_money: row.get(6)?,
                    transfer_expense: row.get(7)?,
                    wage_payment: row.get(8)?,
                    bonus_payment: row.get(9)?,
                    staff_wages: row.get(10)?,
                    facilities: row.get(11)?,
                    youth_academy: row.get(12)?,
                    ..Default::default()
                };
                report.calculate_totals();
                Ok(report)
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(format!("{}-{}", season, team_id)),
            _ => DatabaseError::QueryError(e.to_string()),
        })
    }

    fn get_or_create(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError> {
        match self.get(team_id, season) {
            Ok(report) => Ok(report),
            Err(DatabaseError::NotFound(_)) => {
                // Create new report
                let new_report = SeasonFinanceReport::new(season.to_string(), team_id.to_string());
                let conn = self.conn.write().unwrap();

                conn.execute(
                    "INSERT INTO season_finance_reports (season, team_id, transfer_income, matchday_income, sponsorship, tv_revenue, prize_money,
                                                     transfer_expenses, wage_expenses, bonus_expenses, staff_wages, facilities, youth_academy)
                     VALUES (?, ?, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)",
                    params![season, team_id],
                ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

                Ok(new_report)
            }
            Err(e) => Err(e),
        }
    }

    fn update(&self, report: &SeasonFinanceReport) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE season_finance_reports
             SET transfer_income = ?, matchday_income = ?, sponsorship = ?, tv_revenue = ?, prize_money = ?,
                 transfer_expenses = ?, wage_expenses = ?, bonus_expenses = ?, staff_wages = ?, facilities = ?, youth_academy = ?
             WHERE season = ? AND team_id = ?",
            params![
                report.transfer_income,
                report.matchday_income,
                report.sponsorship,
                report.tv_revenue,
                report.prize_money,
                report.transfer_expense,
                report.wage_payment,
                report.bonus_payment,
                report.staff_wages,
                report.facilities,
                report.youth_academy,
                report.season,
                report.team_id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_history(&self, team_id: &str, limit: usize) -> Result<Vec<SeasonFinanceReport>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT season, team_id, transfer_income, matchday_income, sponsorship, tv_revenue, prize_money,
                    transfer_expenses, wage_expenses, bonus_expenses, staff_wages, facilities, youth_academy
             FROM season_finance_reports
             WHERE team_id = ?
             ORDER BY season DESC
             LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let reports = stmt.query_map(params![team_id, limit], |row| {
            let mut report = SeasonFinanceReport {
                season: row.get(0)?,
                team_id: row.get(1)?,
                transfer_income: row.get(2)?,
                matchday_income: row.get(3)?,
                sponsorship: row.get(4)?,
                tv_revenue: row.get(5)?,
                prize_money: row.get(6)?,
                transfer_expense: row.get(7)?,
                wage_payment: row.get(8)?,
                bonus_payment: row.get(9)?,
                staff_wages: row.get(10)?,
                facilities: row.get(11)?,
                youth_academy: row.get(12)?,
                ..Default::default()
            };
            report.calculate_totals();
            Ok(report)
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(reports)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    #[test]
    fn test_get_or_create_report() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteSeasonFinanceReportRepository::new(conn.clone());

        // Create the league first (foreign key requirement)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Get or create should create a new report
        let report = repo.get_or_create("team1", "2026-27").unwrap();
        assert_eq!(report.season, "2026-27");
        assert_eq!(report.team_id, "team1");
        assert_eq!(report.total_income, 0);

        // Get or create should return existing report
        let report2 = repo.get_or_create("team1", "2026-27").unwrap();
        assert_eq!(report2.season, "2026-27");
    }

    #[test]
    fn test_update_report() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteSeasonFinanceReportRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let mut report = repo.get_or_create("team1", "2026-27").unwrap();
        report.transfer_income = 5_000_000;
        report.wage_payment = 2_000_000;  // Expenses are positive
        report.calculate_totals();

        repo.update(&report).unwrap();

        let retrieved = repo.get("team1", "2026-27").unwrap();
        assert_eq!(retrieved.transfer_income, 5_000_000);
        assert_eq!(retrieved.wage_payment, 2_000_000);
        assert_eq!(retrieved.total_income, 5_000_000);
        assert_eq!(retrieved.total_expense, 2_000_000);
    }

    #[test]
    fn test_get_history() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteSeasonFinanceReportRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create multiple seasons
        repo.get_or_create("team1", "2024-25").unwrap();
        repo.get_or_create("team1", "2025-26").unwrap();
        repo.get_or_create("team1", "2026-27").unwrap();

        let history = repo.get_history("team1", 10).unwrap();
        assert_eq!(history.len(), 3);
        // Should be in descending order
        assert_eq!(history[0].season, "2026-27");
        assert_eq!(history[1].season, "2025-26");
        assert_eq!(history[2].season, "2024-25");
    }
}
