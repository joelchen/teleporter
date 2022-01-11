use super::schema::market_tickers;
use bigdecimal::{BigDecimal, FromPrimitive, Zero};
use diesel::{pg::upsert::on_constraint, prelude::*};
use diesel_citext::types::CiString;
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    time::{Duration, SystemTime},
};
use uuid::Uuid;

#[derive(Queryable, Insertable, Debug, AsChangeset, Deserialize, Serialize)]
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
    pub best_bid_qty: BigDecimal,
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
    fn default() -> Self {
        MarketTicker {
            id: uuid::Uuid::new_v4(),
            exchange: CiString::new(),
            market_type: CiString::new(),
            symbol: CiString::new(),
            open_time: SystemTime::now(),
            close_time: SystemTime::now(),
            event_time: SystemTime::now(),
            price_change: BigDecimal::default(),
            price_change_percent: BigDecimal::default(),
            average_price: BigDecimal::default(),
            prev_close: BigDecimal::default(),
            current_close: BigDecimal::default(),
            current_close_qty: BigDecimal::default(),
            best_bid: BigDecimal::default(),
            best_bid_qty: BigDecimal::default(),
            best_ask: BigDecimal::default(),
            best_ask_qty: BigDecimal::default(),
            open: BigDecimal::default(),
            high: BigDecimal::default(),
            low: BigDecimal::default(),
            volume: BigDecimal::default(),
            quote_volume: BigDecimal::default(),
            num_trades: BigDecimal::default(),
        }
    }
}

impl From<binance::ws_model::DayTickerEvent> for MarketTicker {
    fn from(event: binance::ws_model::DayTickerEvent) -> Self {
        MarketTicker {
            id: Uuid::new_v4(),
            exchange: CiString::from("binance"),
            market_type: CiString::from("spot"),
            symbol: CiString::from(event.symbol),
            price_change: match BigDecimal::from_str(&event.price_change) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            price_change_percent: match BigDecimal::from_str(&event.price_change_percent) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            average_price: match BigDecimal::from_str(&event.average_price) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            prev_close: match BigDecimal::from_str(&event.prev_close) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            current_close: match BigDecimal::from_str(&event.current_close) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            current_close_qty: match BigDecimal::from_str(&event.current_close_qty) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            best_bid: match BigDecimal::from_str(&event.best_bid) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            best_bid_qty: match BigDecimal::from_str(&event.best_bid_qty) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            best_ask: match BigDecimal::from_str(&event.best_ask) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            best_ask_qty: match BigDecimal::from_str(&event.best_ask_qty) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            open: match BigDecimal::from_str(&event.open) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            high: match BigDecimal::from_str(&event.high) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            low: match BigDecimal::from_str(&event.low) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            volume: match BigDecimal::from_str(&event.volume) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            quote_volume: match BigDecimal::from_str(&event.quote_volume) {
                Ok(v) => v,
                Err(_) => BigDecimal::zero(),
            },
            num_trades: match BigDecimal::from_u64(event.num_trades) {
                Some(v) => v,
                None => BigDecimal::zero(),
            },
            open_time: match SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_millis(event.open_time))
            {
                Some(v) => v,
                None => SystemTime::now(),
            },

            close_time: match SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_millis(event.close_time))
            {
                Some(v) => v,
                None => SystemTime::now(),
            },
            event_time: match SystemTime::UNIX_EPOCH
                .checked_add(Duration::from_millis(event.event_time))
            {
                Some(v) => v,
                None => SystemTime::now(),
            },
        }
    }
}

pub fn create_market_ticker(conn: &PgConnection, ticker: &MarketTicker) -> MarketTicker {
    diesel::insert_into(market_tickers::table)
        .values(ticker)
        .on_conflict(on_constraint("const_uidx_market_tickers_primary"))
        .do_update()
        .set(ticker)
        .get_result(conn)
        .expect("Error on saving")
}

pub fn find_market_ticker(
    conn: &PgConnection,
    arg_exchange: String,
    arg_market_type: String,
    arg_symbol: String,
) -> MarketTicker {
    use super::schema::market_tickers::dsl::*;

    let result = market_tickers
        .filter(
            exchange
                .eq(CiString::from(arg_exchange))
                .and(market_type.eq(CiString::from(arg_market_type)))
                .and(symbol.eq(CiString::from(arg_symbol))),
        )
        .limit(1)
        .first::<MarketTicker>(conn)
        .expect("Error loading ticker");

    return result;
}
