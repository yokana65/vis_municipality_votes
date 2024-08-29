use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use actix_web::HttpResponse;
use anyhow::{Context, Result};
use reqwest::Client;

use crate::harvester::muni_geo::fetch_geom;
use crate::harvester::votes_lpz::harvest_votes;
use crate::structs::askama::render_html_summary;
use crate::structs::votes::Vote;

pub async fn data_items() -> HttpResponse {
    println!("Data Items function started.");
    let votes = match get_data().await {
        Ok(votes) => votes,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get data"),
    };

    let html_summary = match render_html_summary(&votes[0]) {
        Ok(result) => result,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to render Html"),
    };

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_summary)
}

pub async fn get_data() -> Result<Vec<Vote>> {
    let start = Instant::now();
    println!("Instant started.");

    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .build()?;

    let mut vote_sources = HashMap::from([(
            "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024",
            "Leipzig_Stadtratswahl_2024"
        ),(
            "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/europawahlen/europawahl-2024",
            "Leipzig_Europawahl_2024"
        )]);

    let mut votes = Vec::new();

    let geom_map = fetch_geom(&client).await?;

    for (url_votes, name_votes) in vote_sources {
        let data_dir = "data";
        let path = PathBuf::from(data_dir).join(name_votes);
        if !path.exists() {
            std::fs::create_dir_all(data_dir)?;
            println!("Data harvest starts for {}", name_votes);
            let vote = harvest_votes(&client, url_votes, name_votes, &geom_map).await?;
            let vote_wgs84 = vote.convert_wgs84().unwrap();

            let _ = vote_wgs84
                .write_geojson()
                .context("Failed to write GeoJson.");
        }
        println!("Read started.");
        let vote = Vote::from_geojson(name_votes)?;
        votes.push(vote);
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    Ok(votes)
}

pub fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Could not find html file for page rendering")
}
