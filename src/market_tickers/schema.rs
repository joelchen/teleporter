table! {
    use diesel::sql_types::*;
    use diesel_citext::sql_types::*;

    market_tickers (id) {
        id -> Uuid,
        exchange -> Citext,
        market_type -> Citext,
        symbol -> Citext,
        price_change -> Numeric,
        price_change_percent -> Numeric,
        average_price -> Numeric,
        prev_close -> Numeric,
        current_close -> Numeric,
        current_close_qty -> Numeric,
        best_bid -> Numeric,
        best_bid_qty -> Numeric,
        best_ask -> Numeric,
        best_ask_qty -> Numeric,
        open -> Numeric,
        high -> Numeric,
        low -> Numeric,
        volume -> Numeric,
        quote_volume -> Numeric,
        num_trades -> Numeric,
        open_time -> Timestamp,
        close_time -> Timestamp,
        event_time -> Timestamp,
    }
}
