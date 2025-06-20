use clap::Parser;

pub use model;

/* 
For Future Reference -
    1. Building Raw agent will supply more consistent behavior and avoid token limits
    2. Tools structs will be useful for tasks that require accurate and consistent outputs
    3. Make a typed struct for lat, lon API data
    4. Cache API responses
    5. When calling openai from providers::Agents, .conetext() seems to be more effective than .preamble()
use rig::agent::Agent;
*/

#[derive(Parser, Debug)]
#[command(name = "Rig Weather Agent")]
#[command(about = "Fetches weather and alert info for a given city and state.")]
struct Args {
    #[arg(long)]
    city: String,
    #[arg(long)]
    state: String,
}

/* tokio lets main be async */
#[tokio::main] 
async fn main() -> Result<(), anyhow::Error>{
    let args = Args::parse();
    let _response = model::prompt_model(args.city, args.state).await?;
    
    Ok(())
}