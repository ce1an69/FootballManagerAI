use crate::team::{Player, Team, Position, calculate_wage};
use crate::game::GameDate;
use thiserror::Error;

/// Transfer error types
#[derive(Debug, Error)]
pub enum TransferError {
    #[error("Transfer window is closed")]
    TransferWindowClosed,

    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Team not found: {0}")]
    TeamNotFound(String),

    #[error("Player is not on the transfer market")]
    PlayerNotOnMarket,

    #[error("Player is already on the transfer market")]
    PlayerAlreadyOnMarket,

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u32, available: u32 },

    #[error("Cannot sell player to their own team")]
    CannotSellToSameTeam,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid offer: {0}")]
    InvalidOffer(String),
}

/// Transfer window types
#[derive(Debug, Clone, PartialEq)]
pub enum TransferWindow {
    Summer,   // June 1 - August 31
    Winter,   // January 1 - January 31
    Closed,   // Transfer window closed
}

impl TransferWindow {
    /// Determine transfer window from game date
    pub fn from_date(date: &GameDate) -> Self {
        if date.is_summer_transfer_window() {
            TransferWindow::Summer
        } else if date.is_winter_transfer_window() {
            TransferWindow::Winter
        } else {
            TransferWindow::Closed
        }
    }

    /// Check if transfers are allowed
    pub fn is_open(&self) -> bool {
        !matches!(self, TransferWindow::Closed)
    }
}

/// Transfer offer details
#[derive(Debug, Clone)]
pub struct TransferOffer {
    pub player_id: String,
    pub from_team_id: String,
    pub to_team_id: String,
    pub offer_amount: u32,
    pub wage_offer: u32,
    pub contract_years: u8,
    pub status: OfferStatus,
}

/// Offer status
#[derive(Debug, Clone, PartialEq)]
pub enum OfferStatus {
    Pending,
    Accepted,
    Rejected,
    Withdrawn,
}

/// Market filter for searching players
#[derive(Debug, Clone, Default)]
pub struct MarketFilter {
    pub positions: Option<Vec<Position>>,
    pub min_ability: Option<u16>,
    pub max_ability: Option<u16>,
    pub max_age: Option<u8>,
    pub max_wage: Option<u32>,
    pub max_price: Option<u32>,
    pub min_age: Option<u8>,
}

/// Market listing information
#[derive(Debug, Clone)]
pub struct MarketListing {
    pub player_id: String,
    pub asking_price: u32,
    pub listed_at: u64,
    pub reason: ListReason,
}

/// Reason for listing
#[derive(Debug, Clone, PartialEq)]
pub enum ListReason {
    TransferListed,
    LoanListed,
    UserRequest,
}

/// Transfer market manager
pub struct TransferMarket {
    // In a real implementation, these would be repository references
    // For now, we'll use in-memory storage
    players: Vec<Player>,
    listings: Vec<MarketListing>,
    teams: Vec<Team>,
}

impl TransferMarket {
    /// Create a new transfer market
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            listings: Vec::new(),
            teams: Vec::new(),
        }
    }

    /// Add a team to the market
    pub fn add_team(&mut self, team: Team) {
        self.teams.push(team);
    }

    /// List a player on the transfer market
    pub fn list_player(
        &mut self,
        player: Player,
        asking_price: u32,
        reason: ListReason,
        current_date: &GameDate,
    ) -> Result<(), TransferError> {
        // Check if transfer window is open (optional - can list anytime)
        let _window = TransferWindow::from_date(current_date);

        // Check if player is already listed
        if self.listings.iter().any(|l| l.player_id == player.id) {
            return Err(TransferError::PlayerAlreadyOnMarket);
        }

        let listing = MarketListing {
            player_id: player.id.clone(),
            asking_price,
            listed_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            reason,
        };

        self.players.push(player);
        self.listings.push(listing);

        Ok(())
    }

    /// Remove a player from the market
    pub fn delist_player(&mut self, player_id: &str) -> Result<(), TransferError> {
        self.listings.retain(|l| l.player_id != player_id);
        self.players.retain(|p| p.id != player_id);
        Ok(())
    }

    /// Get all players on the market
    pub fn get_market_players(&self) -> Vec<Player> {
        self.players.clone()
    }

    /// Get market listings
    pub fn get_market_listings(&self) -> Vec<MarketListing> {
        self.listings.clone()
    }

    /// Search players with filters
    pub fn search_players(&self, filters: MarketFilter) -> Vec<Player> {
        let mut players = self.get_market_players();

        // Apply filters
        if let Some(positions) = filters.positions {
            players.retain(|p| positions.contains(&p.position));
        }

        if let Some(min_ability) = filters.min_ability {
            players.retain(|p| p.current_ability >= min_ability);
        }

        if let Some(max_ability) = filters.max_ability {
            players.retain(|p| p.current_ability <= max_ability);
        }

        if let Some(max_age) = filters.max_age {
            players.retain(|p| p.age <= max_age);
        }

        if let Some(min_age) = filters.min_age {
            players.retain(|p| p.age >= min_age);
        }

        if let Some(max_wage) = filters.max_wage {
            players.retain(|p| p.wage <= max_wage);
        }

        if let Some(max_price) = filters.max_price {
            players.retain(|p| p.market_value <= max_price);
        }

        players
    }

    /// Buy a player from the market
    pub fn buy_player(
        &mut self,
        buyer_team_id: &str,
        player_id: &str,
        offer_amount: u32,
        current_date: &GameDate,
    ) -> Result<Player, TransferError> {
        // Check transfer window
        let window = TransferWindow::from_date(current_date);
        if !window.is_open() {
            return Err(TransferError::TransferWindowClosed);
        }

        // Find the player
        let player_index = self.players
            .iter()
            .position(|p| p.id == player_id)
            .ok_or_else(|| TransferError::PlayerNotFound(player_id.to_string()))?;

        let player = &self.players[player_index];

        // Check if player is on market
        let listing = self.listings
            .iter()
            .find(|l| l.player_id == player_id)
            .ok_or(TransferError::PlayerNotOnMarket)?;

        // Find buyer team
        let team_index = self.teams
            .iter()
            .position(|t| t.id == buyer_team_id)
            .ok_or_else(|| TransferError::TeamNotFound(buyer_team_id.to_string()))?;

        let team = &self.teams[team_index];

        // Check budget
        if team.budget < offer_amount {
            return Err(TransferError::InsufficientFunds {
                required: offer_amount,
                available: team.budget,
            });
        }

        // Execute transfer
        let mut player = player.clone();
        player.team_id = Some(buyer_team_id.to_string());
        player.wage = calculate_wage(&player);
        player.contract_years = 3;

        // Update market
        self.players.remove(player_index);
        self.listings.retain(|l| l.player_id != player_id);

        // Update team budget (would be persisted in real implementation)
        // self.teams[team_index].budget -= offer_amount;

        Ok(player)
    }

    /// Get listing for a specific player
    pub fn get_market_listing(&self, player_id: &str) -> Option<&MarketListing> {
        self.listings.iter().find(|l| l.player_id == player_id)
    }

    /// Update asking price
    pub fn update_price(&mut self, player_id: &str, new_price: u32) -> Result<(), TransferError> {
        let listing = self.listings
            .iter_mut()
            .find(|l| l.player_id == player_id)
            .ok_or(TransferError::PlayerNotOnMarket)?;

        listing.asking_price = new_price;
        Ok(())
    }

    /// Get player by ID
    pub fn get_player(&self, player_id: &str) -> Option<&Player> {
        self.players.iter().find(|p| p.id == player_id)
    }

    /// Get team by ID
    pub fn get_team(&self, team_id: &str) -> Option<&Team> {
        self.teams.iter().find(|t| t.id == team_id)
    }

    /// Get team mutably
    pub fn get_team_mut(&mut self, team_id: &str) -> Option<&mut Team> {
        self.teams.iter_mut().find(|t| t.id == team_id)
    }
}

impl Default for TransferMarket {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::{Player, Position};
    use crate::game::GameDate;

    fn create_test_player(id: &str, position: Position, age: u8, value: u32) -> Player {
        let mut player = Player::new(id.to_string(), "Test Player".to_string(), position);
        player.age = age;
        player.market_value = value;
        player.current_ability = 100;
        player
    }

    fn create_test_team(id: &str, budget: u32) -> Team {
        Team::new(id.to_string(), "Test Team".to_string(), "league1".to_string(), budget)
    }

    #[test]
    fn test_transfer_window_detection() {
        let summer = GameDate { year: 2026, month: 7, day: 15 };
        assert_eq!(TransferWindow::from_date(&summer), TransferWindow::Summer);
        assert!(TransferWindow::from_date(&summer).is_open());

        let winter = GameDate { year: 2026, month: 1, day: 15 };
        assert_eq!(TransferWindow::from_date(&winter), TransferWindow::Winter);
        assert!(TransferWindow::from_date(&winter).is_open());

        let closed = GameDate { year: 2026, month: 4, day: 15 };
        assert_eq!(TransferWindow::from_date(&closed), TransferWindow::Closed);
        assert!(!TransferWindow::from_date(&closed).is_open());
    }

    #[test]
    fn test_list_player() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let date = GameDate { year: 2026, month: 7, day: 15 };

        let result = market.list_player(player, 1500000, ListReason::TransferListed, &date);
        assert!(result.is_ok());
        assert_eq!(market.get_market_players().len(), 1);
    }

    #[test]
    fn test_list_duplicate_player() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(player.clone(), 1500000, ListReason::TransferListed, &date).unwrap();
        let result = market.list_player(player, 1500000, ListReason::TransferListed, &date);

        assert!(matches!(result, Err(TransferError::PlayerAlreadyOnMarket)));
    }

    #[test]
    fn test_search_players_by_position() {
        let mut market = TransferMarket::new();
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(create_test_player("p1", Position::ST, 25, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p2", Position::CM, 25, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p3", Position::ST, 25, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();

        let filters = MarketFilter {
            positions: Some(vec![Position::ST]),
            ..Default::default()
        };

        let results = market.search_players(filters);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.position == Position::ST));
    }

    #[test]
    fn test_search_players_by_age() {
        let mut market = TransferMarket::new();
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(create_test_player("p1", Position::ST, 20, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p2", Position::ST, 30, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p3", Position::ST, 25, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();

        let filters = MarketFilter {
            max_age: Some(25),
            ..Default::default()
        };

        let results = market.search_players(filters);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.age <= 25));
    }

    #[test]
    fn test_buy_player_success() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let team = create_test_team("team1", 5000000);
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(player, 1500000, ListReason::TransferListed, &date).unwrap();
        market.add_team(team);

        let result = market.buy_player("team1", "player1", 1500000, &date);
        assert!(result.is_ok());
        assert_eq!(market.get_market_players().len(), 0);
    }

    #[test]
    fn test_buy_player_insufficient_funds() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let team = create_test_team("team1", 100000); // Low budget
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(player, 1500000, ListReason::TransferListed, &date).unwrap();
        market.add_team(team);

        let result = market.buy_player("team1", "player1", 1500000, &date);
        assert!(matches!(result, Err(TransferError::InsufficientFunds { .. })));
    }

    #[test]
    fn test_buy_player_transfer_window_closed() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let team = create_test_team("team1", 5000000);
        let date = GameDate { year: 2026, month: 4, day: 15 }; // Transfer window closed

        market.list_player(player, 1500000, ListReason::TransferListed, &date).unwrap();
        market.add_team(team);

        let result = market.buy_player("team1", "player1", 1500000, &date);
        assert!(matches!(result, Err(TransferError::TransferWindowClosed)));
    }

    #[test]
    fn test_delist_player() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(player, 1500000, ListReason::TransferListed, &date).unwrap();
        assert_eq!(market.get_market_players().len(), 1);

        market.delist_player("player1").unwrap();
        assert_eq!(market.get_market_players().len(), 0);
    }

    #[test]
    fn test_update_price() {
        let mut market = TransferMarket::new();
        let player = create_test_player("player1", Position::ST, 25, 1000000);
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(player, 1500000, ListReason::TransferListed, &date).unwrap();

        market.update_price("player1", 2000000).unwrap();
        let listing = market.get_market_listing("player1").unwrap();
        assert_eq!(listing.asking_price, 2000000);
    }

    #[test]
    fn test_search_by_price() {
        let mut market = TransferMarket::new();
        let date = GameDate { year: 2026, month: 7, day: 15 };

        market.list_player(create_test_player("p1", Position::ST, 25, 1000000), 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p2", Position::ST, 25, 3000000), 5000000, ListReason::TransferListed, &date).unwrap();
        market.list_player(create_test_player("p3", Position::ST, 25, 800000), 1000000, ListReason::TransferListed, &date).unwrap();

        let filters = MarketFilter {
            max_price: Some(1500000),
            ..Default::default()
        };

        let results = market.search_players(filters);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_by_ability() {
        let mut market = TransferMarket::new();
        let date = GameDate { year: 2026, month: 7, day: 15 };

        let mut p1 = create_test_player("p1", Position::ST, 25, 1000000);
        p1.current_ability = 120;

        let mut p2 = create_test_player("p2", Position::ST, 25, 1000000);
        p2.current_ability = 80;

        let mut p3 = create_test_player("p3", Position::ST, 25, 1000000);
        p3.current_ability = 100;

        market.list_player(p1, 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(p2, 1500000, ListReason::TransferListed, &date).unwrap();
        market.list_player(p3, 1500000, ListReason::TransferListed, &date).unwrap();

        let filters = MarketFilter {
            min_ability: Some(100),
            max_ability: Some(120),
            ..Default::default()
        };

        let results = market.search_players(filters);
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|p| p.current_ability >= 100 && p.current_ability <= 120));
    }
}
