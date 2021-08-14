#[macro_use]
extern crate diesel;
extern crate bigdecimal;

pub mod models;
pub mod schema;
pub mod handlers;
pub mod routes;


use diesel::{pg::upsert::on_constraint, prelude::*};
use std::env;
use models::MarketTicker;
use binance::websockets::*;
use std::{sync::atomic::AtomicBool, thread, time::Duration};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error on connecting to {}, err: {}", database_url, e))
}

pub fn create_market_ticker(conn: &PgConnection, ticker: &MarketTicker) -> MarketTicker {
    use schema::market_tickers;

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
    arg_symbol: String
) -> MarketTicker {
    use schema::market_tickers::dsl::*;
    use diesel_citext::types::CiString;

    let result = market_tickers.filter(
        exchange.eq(CiString::from(arg_exchange))
        .and(market_type.eq(CiString::from(arg_market_type)))
        .and(symbol.eq(CiString::from(arg_symbol)))
    ).limit(1)
    .first::<MarketTicker>(conn)
    .expect("Error loading ticker");

    return result;
}


pub async fn start_http_server() {
    let market_ticker_routes = routes::get_market();
    warp::serve(market_ticker_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

pub async fn start_websocket_client() {
    let keep_running = AtomicBool::new(true);
    let agg_trade = format!("!ticker@arr");
    let conn = establish_connection();

    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::DayTickerAll(ticker_events) => {
                for tick_event in ticker_events {
                    let ticker = MarketTicker::from(tick_event);
                    create_market_ticker(&conn, &ticker);
                    thread::sleep(Duration::from_millis(100));
                    println!("created id: {:?}", ticker.id);
                }
            }
            _ => (),
        };

        Ok(())
    });

    match web_socket.connect(agg_trade.as_str()) {
        Ok(_) => match web_socket.event_loop(&keep_running) {
            Err(err) => println!("event_loop Error: {:?}", err),
            _ => (),
        },

        Err(e) => println!("connect Error: {:?}", e),
    };
}
