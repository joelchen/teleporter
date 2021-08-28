use super::handlers::get_market_ticker;
use warp::{self, Filter};

pub fn get_market() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("ticker" / String / String / String)
        .and(warp::get())
        .and_then(get_market_ticker)
}
