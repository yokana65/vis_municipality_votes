use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};
use askama::Template;
use anyhow::{Result, Context};
use reqwest::Client;

use crate::structs::votes::Vote;
use crate::harvester::votes_lpz::harvest_votes;

pub fn views_factory(app: &mut ServiceConfig) {
    app_views_factory(app);
}

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", web::get().to(items));
}

pub async fn items() -> HttpResponse {
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


pub async fn get_data() -> Result<Vote> {
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

    // TODO: write read functionality that returns a Vote
    let vote = Vote::from_geojson(&name_votes)?;

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

    Ok(vote)
}


#[derive(Template)]
#[template(path = "main.html")]
struct VoteSummaryTemplate<'a> {
    vote_name: &'a str,
    vote_records: Vec<VoteRecordTemplate<'a>>,
}

struct VoteRecordTemplate<'a> {
    name_muni: &'a str,
    votes: &'a HashMap<String, i16>,
    has_geometry: bool,
}

fn render_html_summary(vote: &Vote) -> Result<String> {
    let vote_records: Vec<VoteRecordTemplate> = vote.vote_records.iter().map(|record| {
        VoteRecordTemplate {
            name_muni: &record.name_muni,
            votes: &record.votes,
            has_geometry: record.geometry.is_some(),
        }
    }).collect();

    let template = VoteSummaryTemplate {
        vote_name: &vote.name,
        vote_records,
    };

    Ok(template.render()?)
}

