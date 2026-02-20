use crate::kalshi::kalshi::make_authenticated_request;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct BalanceOutput {
    balance: i64,
    portfolio_value: i64,
}

pub async fn get_balance() -> Result<String> {
    let response = make_authenticated_request("GET", "/portfolio/balance").await?;
    let json = response.json::<BalanceOutput>().await?;
    Ok(json.balance.to_string())
}

pub async fn get_portfolio_value() -> Result<String> {
    let response = make_authenticated_request("GET", "/portfolio/balance").await?;
    let json = response.json::<BalanceOutput>().await?;
    Ok(json.portfolio_value.to_string())
}
