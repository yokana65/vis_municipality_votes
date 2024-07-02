use std::time::Duration;

use anyhow::Result;
use reqwest::Client;

use harvester::votes_lpz::harvest_votes;

mod harvester;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;

    let url_votes = "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024";

    let vote = harvest_votes(&client, &url_votes).await?;

    println!("Vote name: {}", vote.name);
    for record in &vote.vote_records {
        println!("Municipality: {}", record.name_muni);
        for (party, vote_count) in &record.votes {
            println!("  {}: {}", party, vote_count);
        }
    }

    Ok(())
}
