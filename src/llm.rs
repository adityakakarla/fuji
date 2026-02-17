use crate::config;
use anyhow::Result;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::{Deserialize, Serialize};

const GROK_MODEL: &str = "grok-4-1-fast-reasoning";

#[derive(Debug, Serialize)]
struct LLMInput {
    model: String,
    input: Vec<LLMMessage>,
}

#[derive(Debug, Serialize)]
struct LLMMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RawLLMResponse {
    created_at: i32,
    completed_at: i32,
    id: String,
    model: String,
    output: Vec<LLMOutput>,
    temperature: f32,
    usage: LLMUsage,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanLLMResponse {
    pub output: String,
    pub error: Option<String>,
    pub cost: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMUsage {
    input_tokens: u32,
    output_tokens: u32,
    total_tokens: u32,
    num_sources_used: u32,
    num_server_side_tools_used: u32,
    cost_in_usd_ticks: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMOutput {
    content: Vec<LLMContent>,
    id: String,
    role: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LLMContent {
    text: String,
}

pub async fn generate_text(prompt: &str) -> Result<CleanLLMResponse> {
    let api_key = config::get_grok_api_key()?;
    let client = Client::new();

    let mut header_map = HeaderMap::new();
    let content_type = HeaderValue::from_str("application/json")?;
    header_map.insert("Content-Type", content_type);
    let authorization = HeaderValue::from_str(format!("Bearer {}", api_key).as_str())?;
    header_map.insert("Authorization", authorization);

    let body = serde_json::to_string(&LLMInput {
        model: GROK_MODEL.to_string(),
        input: vec![LLMMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    })?;

    let res = client
        .post("https://api.x.ai/v1/responses")
        .body(body)
        .headers(header_map)
        .send()
        .await?;

    let status = res.status();
    if !status.is_success() {
        return Err(anyhow::Error::msg(res.text().await?));
    }

    let response = res.json::<RawLLMResponse>().await?;

    let output = &response.output[0];
    let content = &output.content[0];
    let text = content.text.clone();
    let error = response.error;
    let cost = response.usage.cost_in_usd_ticks / 10_000_000_000.0;

    Ok(CleanLLMResponse {
        output: text,
        error,
        cost,
    })
}
