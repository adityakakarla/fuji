use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::kalshi::kalshi::make_request;

#[derive(Debug, Serialize, Deserialize)]
struct Markets {
    markets: Vec<Market>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Market {
    ticker: String,
    event_ticker: String,
    title: String,
    subtitle: String,
    yes_sub_title: String,
    no_sub_title: String,
    status: String,
    yes_bid_dollars: String,
    yes_ask_dollars: String,
    no_bid_dollars: String,
    no_ask_dollars: String,
    volume: u64,
}

async fn get_markets_by_series_ticker(series_ticker: &str) -> Result<String> {
    let request = make_request(
        "GET",
        &format!("/markets?series_ticker={}&status=open", series_ticker),
    )
    .await?;
    let response = request.json::<Markets>().await?;
    Ok(format!("{:?}", response))
}

pub async fn get_t20_markets() -> Result<String> {
    get_markets_by_series_ticker("KXT20MATCH").await
}
