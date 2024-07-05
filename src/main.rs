use std::any::{type_name, type_name_of_val};
use std::time::{Duration, Instant};
use std::path::PathBuf;

use anyhow::{Context, Result};
use geo::Polygon;
use reqwest::Client;

use harvester::votes_lpz::harvest_votes;

mod harvester;
mod structs;

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;

    let url_votes = "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024";
    let name_votes = "Leipzig Stadtratswahl 2024";

    let data_dir = "data";
    let path = PathBuf::from(data_dir).join(name_votes);

    if !path.exists() {
        std::fs::create_dir_all(data_dir)?;


        let vote = harvest_votes(&client, &url_votes, &name_votes).await?;

        let _ = vote.write_geojson().context("Failed to write GeoJson.");
    }

    // println!("Vote name: {}", vote.name);
    // for record in &vote.vote_records {
    //     println!("Municipality: {}", record.name_muni);
    //     for (party, vote_count) in &record.votes {
    //         println!("  {}: {}", party, vote_count);
    //     }
    //     for geom in &record.geometry {
    //         if type_name_of_val(geom) == type_name::<Polygon>() {
    //             println!("Contains a geometry");
    //         }
    //     }
    // }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    Ok(())
}
