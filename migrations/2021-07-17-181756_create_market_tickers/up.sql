-- Your SQL goes here
create extension if not exists "uuid-ossp";
create extension if not exists "citext";

create table market_tickers (
	id uuid primary key default uuid_generate_v4(),
	exchange citext not null,
	market_type citext not null,
	symbol citext not null,
	price_change numeric not null,
	price_change_percent numeric not null,
	average_price numeric not null,
	prev_close numeric not null,
	current_close numeric not null,
	current_close_qty numeric not null,
	best_bid numeric not null,
	best_bid_qty numeric not null,
	best_ask numeric not null,
	best_ask_qty numeric not null,
	open  numeric not null,
	high numeric not null,
	low numeric not null,
	volume numeric not null,
	quote_volume numeric not null,
	num_trades numeric not null,
	open_time timestamp not null,
	close_time timestamp not null,
	event_time timestamp default now() not null,
	constraint const_uidx_market_tickers_primary unique(exchange, symbol,	market_type)
);

create unique index uidx_market_tickers_primary on market_tickers (
	exchange, 
	symbol,
	market_type
);