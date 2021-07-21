use bigdecimal::BigDecimal;
// use diesel::sql_types::*;
// use diesel::sql_types::Uuid;
use diesel_citext::types::CiString;
use std::time::SystemTime;

use super::schema::market_tickers;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "market_tickers"]
pub struct MarketTicker {
    pub id: uuid::Uuid,
    pub exchange: CiString,
    pub market_type: CiString,
    pub symbol: CiString,
    pub price_change: BigDecimal,
    pub price_change_percent: BigDecimal,
    pub average_price: BigDecimal,
    pub prev_close: BigDecimal,
    pub current_close: BigDecimal,
    pub current_close_qty: BigDecimal,
    pub best_bid: BigDecimal,
    pub best_bit_qty: BigDecimal,
    pub best_ask: BigDecimal,
    pub best_ask_qty: BigDecimal,
    pub open: BigDecimal,
    pub high: BigDecimal,
    pub low: BigDecimal,
    pub volume: BigDecimal,
    pub quote_volume: BigDecimal,
    pub num_trades: BigDecimal,
    pub open_time: SystemTime,
    pub close_time: SystemTime,
    pub event_time: SystemTime,
}

impl Default for MarketTicker {
    fn default() -> MarketTicker {
        MarketTicker {
            exchange: CiString::new(),
            market_type: CiString::new(),
            symbol: CiString::new(),
            open_time: SystemTime::now(),
            close_time: SystemTime::now(),
            event_time: SystemTime::now(),
            ..Default::default()
        }
    }
}
