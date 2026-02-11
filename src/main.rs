mod config;
mod llm;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fuji")]
#[command(about = "The AI copilot for students")]
struct Args {
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let prompt = args.prompt;
    let response = llm::generate_text(&prompt).await?;
    println!("Response: {}", response);
    Ok(())
}
