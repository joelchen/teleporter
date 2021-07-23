use binance::websockets::*;
use std::{sync::atomic::AtomicBool, thread, time::Duration};
use teleporter::{create_market_ticker, establish_connection, models::MarketTicker};
// use teleporter::models::MarketTicker;

fn main() {
    let keep_running = AtomicBool::new(true);
    let agg_trade = format!("!ticker@arr");
    let conn = establish_connection();

    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::DayTickerAll(ticker_events) => {
                for tick_event in ticker_events {
                    // TODO: finish the trait From
                    let ticker = MarketTicker::from(tick_event);
                    create_market_ticker(&conn, &ticker);
                    thread::sleep(Duration::from_millis(100))
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
