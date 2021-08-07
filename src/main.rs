use warp::Filter;
use binance::websockets::*;
use std::{sync::atomic::AtomicBool, thread, time::Duration};
use teleporter::{create_market_ticker, establish_connection, find_market_ticker, models::MarketTicker};
use tokio::task;
// use teleporter::models::MarketTicker;

#[tokio::main]
async fn main() {
    let _httpserver = task::spawn(start_http_server());
    let _websocketclient= task::spawn(start_websocket_client());

    let _ = tokio::join!(_httpserver, _websocketclient);
}

async fn start_http_server() {
    let market_ticker = warp::path!("ticker" / String / String / String)
        .map(|exchange, market_type, market| {
            let conn = establish_connection();
            let market_ticker = find_market_ticker(&conn, exchange, market_type, market);
            format!("ticker, {:?}!", market_ticker)
        });

    warp::serve(market_ticker)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

async fn start_websocket_client() {
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
