use std::path::PathBuf;
use std::time::{Duration, Instant};

use actix_web::HttpResponse;
use anyhow::{Result, Context};
use reqwest::Client;

use crate::structs::votes::Vote;
use crate::harvester::votes_lpz::harvest_votes;
use crate::structs::askama::render_html_summary;

pub async fn data_items() -> HttpResponse {
    let vote = match get_data().await {
        Ok(vote) => vote,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get data"),
    };

    let html_summary = match render_html_summary(&vote) {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to render Html"),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_summary)
}


async fn get_data() -> Result<Vote> {
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

    let vote = Vote::from_geojson(&name_votes)?;

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(vote)
}
