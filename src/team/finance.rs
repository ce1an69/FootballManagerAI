use serde::{Deserialize, Serialize};
use crate::game::GameDate;

/// Team financial information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamFinance {
    pub team_id: String,
    pub balance: i64,              // Current balance (can be negative)
    pub wage_budget: i64,          // Weekly wage budget
    pub transfer_budget: i64,      // Transfer budget
}

impl TeamFinance {
    /// Create new team finances
    pub fn new(team_id: String, balance: i64, wage_budget: i64, transfer_budget: i64) -> Self {
        Self {
            team_id,
            balance,
            wage_budget,
            transfer_budget,
        }
    }

    /// Check if team can afford transfer fee
    pub fn can_afford_transfer(&self, fee: i64) -> bool {
        self.transfer_budget >= fee
    }

    /// Check if team can afford weekly wages
    pub fn can_afford_wages(&self, weekly_wages: i64) -> bool {
        self.wage_budget >= weekly_wages
    }

    /// Process transfer expenditure
    pub fn spend_on_transfer(&mut self, amount: i64) -> bool {
        if self.can_afford_transfer(amount) {
            self.transfer_budget -= amount;
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    /// Process transfer income
    pub fn earn_from_transfer(&mut self, amount: i64) {
        self.transfer_budget += amount;
        self.balance += amount;
    }

    /// Pay weekly wages
    pub fn pay_wages(&mut self, amount: i64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

/// Financial transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinanceTransaction {
    pub id: String,
    pub team_id: String,
    pub transaction_type: TransactionType,
    pub amount: i64,               // Positive for income, negative for expense
    pub description: String,
    pub date_year: u32,
    pub date_month: u8,
    pub date_day: u8,
    pub related_player_id: Option<String>,
}

impl FinanceTransaction {
    /// Get the date as GameDate
    pub fn date(&self) -> GameDate {
        GameDate {
            year: self.date_year as u16,
            month: self.date_month,
            day: self.date_day,
        }
    }

    /// Create transaction with date
    pub fn with_date(mut self, date: &GameDate) -> Self {
        self.date_year = date.year as u32;
        self.date_month = date.month;
        self.date_day = date.day;
        self
    }
}

/// Type of transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    // Income types
    TransferIncome,
    MatchdayIncome,
    Sponsorship,
    TVRevenue,
    PrizeMoneyWin,

    // Expense types
    TransferExpense,
    WagePayment,
    BonusPayment,
    StaffWages,
    Facilities,
    YouthAcademy,
}

/// Season financial report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeasonFinanceReport {
    pub season: String,            // e.g., "2026-27"
    pub team_id: String,

    // Income breakdown
    pub transfer_income: i64,
    pub matchday_income: i64,
    pub sponsorship: i64,
    pub tv_revenue: i64,
    pub prize_money: i64,
    pub total_income: i64,

    // Expense breakdown
    pub transfer_expense: i64,
    pub wage_payment: i64,
    pub bonus_payment: i64,
    pub staff_wages: i64,
    pub facilities: i64,
    pub youth_academy: i64,
    pub total_expense: i64,

    // Summary
    pub net_profit: i64,
}

impl SeasonFinanceReport {
    /// Create new season report
    pub fn new(season: String, team_id: String) -> Self {
        Self {
            season,
            team_id,
            ..Default::default()
        }
    }

    /// Calculate totals
    pub fn calculate_totals(&mut self) {
        self.total_income = self.transfer_income + self.matchday_income +
            self.sponsorship + self.tv_revenue + self.prize_money;

        self.total_expense = self.transfer_expense + self.wage_payment +
            self.bonus_payment + self.staff_wages + self.facilities + self.youth_academy;

        self.net_profit = self.total_income - self.total_expense;
    }

    /// Add income transaction
    pub fn add_income(&mut self, transaction_type: TransactionType, amount: i64) {
        match transaction_type {
            TransactionType::TransferIncome => self.transfer_income += amount,
            TransactionType::MatchdayIncome => self.matchday_income += amount,
            TransactionType::Sponsorship => self.sponsorship += amount,
            TransactionType::TVRevenue => self.tv_revenue += amount,
            TransactionType::PrizeMoneyWin => self.prize_money += amount,
            _ => {}
        }
        self.calculate_totals();
    }

    /// Add expense transaction
    pub fn add_expense(&mut self, transaction_type: TransactionType, amount: i64) {
        match transaction_type {
            TransactionType::TransferExpense => self.transfer_expense += amount.abs(),
            TransactionType::WagePayment => self.wage_payment += amount.abs(),
            TransactionType::BonusPayment => self.bonus_payment += amount.abs(),
            TransactionType::StaffWages => self.staff_wages += amount.abs(),
            TransactionType::Facilities => self.facilities += amount.abs(),
            TransactionType::YouthAcademy => self.youth_academy += amount.abs(),
            _ => {}
        }
        self.calculate_totals();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finance_creation() {
        let finance = TeamFinance::new("1".to_string(), 1_000_000, 100_000, 500_000);
        assert_eq!(finance.balance, 1_000_000);
        assert_eq!(finance.transfer_budget, 500_000);
    }

    #[test]
    fn test_can_afford_transfer() {
        let finance = TeamFinance::new("1".to_string(), 1_000_000, 100_000, 500_000);
        assert!(finance.can_afford_transfer(300_000));
        assert!(!finance.can_afford_transfer(600_000));
    }

    #[test]
    fn test_spend_on_transfer() {
        let mut finance = TeamFinance::new("1".to_string(), 1_000_000, 100_000, 500_000);
        let success = finance.spend_on_transfer(200_000);

        assert!(success);
        assert_eq!(finance.transfer_budget, 300_000);
        assert_eq!(finance.balance, 800_000);
    }

    #[test]
    fn test_earn_from_transfer() {
        let mut finance = TeamFinance::new("1".to_string(), 1_000_000, 100_000, 500_000);
        finance.earn_from_transfer(150_000);

        assert_eq!(finance.transfer_budget, 650_000);
        assert_eq!(finance.balance, 1_150_000);
    }

    #[test]
    fn test_pay_wages() {
        let mut finance = TeamFinance::new("1".to_string(), 1_000_000, 100_000, 500_000);
        let success = finance.pay_wages(50_000);

        assert!(success);
        assert_eq!(finance.balance, 950_000);
    }

    #[test]
    fn test_season_report() {
        let mut report = SeasonFinanceReport::new("2026-27".to_string(), "1".to_string());

        report.add_income(TransactionType::TransferIncome, 1_000_000);
        report.add_income(TransactionType::MatchdayIncome, 500_000);
        report.add_expense(TransactionType::WagePayment, -300_000);
        report.add_expense(TransactionType::TransferExpense, -500_000);

        assert_eq!(report.total_income, 1_500_000);
        assert_eq!(report.total_expense, 800_000);
        assert_eq!(report.net_profit, 700_000);
    }
}
