#[macro_use]
extern crate diesel;
extern crate bigdecimal;

mod market_tickers;

use binance::websockets::*;
use diesel::prelude::*;
use market_tickers::models::{create_market_ticker, MarketTicker};
use market_tickers::routes;
use std::env;
use std::{sync::atomic::AtomicBool, thread, time::Duration};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error on connecting to {}, err: {}", database_url, e))
}

pub async fn start_http_server() {
    let market_ticker_routes = routes::get_market();
    warp::serve(market_ticker_routes)
        .run(([0, 0, 0, 0], 3000))
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
