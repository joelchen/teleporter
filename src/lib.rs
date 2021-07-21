#[macro_use]
extern crate diesel;
extern crate bigdecimal;
// extern crate uuid;

pub mod models;
pub mod schema;

// use binance::market;
use diesel::prelude::*;
use models::MarketTicker;
use std::env;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error on connecting to {}, err: {}", database_url, e))
}

pub fn create_market_ticker(conn: &PgConnection, ticker: &MarketTicker) -> MarketTicker {
    use schema::market_tickers;

    diesel::insert_into(market_tickers::table)
        .values(ticker)
        .get_result(conn)
        .expect("Error on saving")
}
