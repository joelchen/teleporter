use bigdecimal::BigDecimal;
use diesel::sql_types::*;
use diesel_citext::sql_types::*;

use super::schema::market_tickers;

#[derive(Queryable, Insertable)]
#[table_name = "market_tickers"]
pub struct MarketTicker {
    pub exchange: Citext,
    pub market_type: Citext,
    pub symbol: Citext,
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
    pub open_time: Timestamp,
    pub close_time: Timestamp,
    pub event_time: Timestamp,
}
