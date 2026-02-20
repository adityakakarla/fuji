use anyhow::Result;

use crate::kalshi::kalshi::make_request;

async fn get_markets_by_series_ticker(series_ticker: &str) -> Result<String> {
    let request = make_request("GET", &format!("/markets?series_ticker={}", series_ticker)).await?;
    let response = request.text().await?;
    Ok(response)
}

pub async fn get_t20_markets() -> Result<String> {
    Ok(get_markets_by_series_ticker("KXT20MATCH").await?)
}
