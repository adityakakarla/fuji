use crate::kalshi::{
    api::make_authenticated_get_request, markets::get_market_information_by_ticker,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Orders {
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub ticker: String,
    pub side: String,
    pub action: String,
    #[serde(rename = "type")]
    pub order_type: String,
    pub yes_price_dollars: String,
    pub no_price_dollars: String,
    pub status: String,
    pub fill_count: u32,
    pub taker_fees: u32,
    pub maker_fees: u32,
    pub taker_fill_cost_dollars: String,
    pub maker_fill_cost_dollars: String,
}

async fn get_open_orders() -> Result<Orders> {
    let response = make_authenticated_get_request("/portfolio/orders").await?;
    let json = response.json::<Orders>().await?;
    Ok(json)
}

pub async fn get_open_order_details() -> Result<String> {
    let orders = get_open_orders().await?;
    let mut order_details = String::new();

    for order in orders.orders {
        order_details.push_str(&format!("{:?}", order));
        order_details.push_str(
            format!(
                "{:?}",
                get_market_information_by_ticker(&order.ticker).await?
            )
            .as_str(),
        );
    }
    Ok(order_details)
}
