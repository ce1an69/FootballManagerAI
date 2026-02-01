use crate::data::{DatabaseError, TransferMarketRepository, MarketListing};
use crate::team::Player;

/// SQLite implementation of TransferMarketRepository
pub struct SqliteTransferMarketRepository;

impl TransferMarketRepository for SqliteTransferMarketRepository {
    fn add_to_market(&self, _player_id: &str, _price: u32) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn remove_from_market(&self, _player_id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_market_players(&self) -> Result<Vec<Player>, DatabaseError> {
        Ok(Vec::new())
    }

    fn get_market_listing(&self, _player_id: &str) -> Result<Option<MarketListing>, DatabaseError> {
        Ok(None)
    }

    fn update_price(&self, _player_id: &str, _new_price: u32) -> Result<(), DatabaseError> {
        Ok(())
    }
}
