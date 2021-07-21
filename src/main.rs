use binance::websockets::*;
use std::{sync::atomic::AtomicBool, thread, time::Duration};
use teleporter::{create_market_ticker, establish_connection, models::MarketTicker};

fn main() {
    let keep_running = AtomicBool::new(true);
    let agg_trade = format!("!ticker@arr");
    let conn = establish_connection();
    let t = MarketTicker::default();
    create_market_ticker(&conn, &t);
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::DayTickerAll(ticker_events) => {
                for tick_event in ticker_events {
                    println!("{:?}", tick_event);
                    thread::sleep(Duration::from_secs(1))
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
