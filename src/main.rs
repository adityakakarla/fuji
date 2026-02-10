mod config;

use clap::Parser;

#[derive(Parser)]
#[command(name = "fuji")]
#[command(about = "The AI copilot for students")]
struct Args {
    prompt: String,
}

fn main() {
    let args = Args::parse();
    println!("Prompt: {}", args.prompt);
}
