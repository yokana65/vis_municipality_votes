use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use csv::ReaderBuilder;
use geo::Polygon;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::structs::votes::{Vote, VoteRecord};

pub async fn harvest_votes(
    client: &Client,
    url: &str,
    name: &str,
    csv_url: &Option<String>,
    party_map: &HashMap<String, String>,
    geom_map: &HashMap<String, Polygon>,
) -> Result<Vote> {
    let body = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href$='.csv'][href*='Ortsteil']")
        .expect("Failed to parse CSS selector for CSV link");

    let csv_url_scrp = match document.select(&selector).next() {
        Some(link) => link.value().attr("href").map(|s| s.to_string()),
        None => None,
    };

    match csv_url_scrp {
        Some(ref url) => println!("Scraping data from: {}", url),
        None => println!("Using default CSV URL: {:?}", csv_url),
    }

    let csv_url_scrp =
        csv_url_scrp.unwrap_or_else(|| csv_url.clone().expect("No url in source provided."));
    let csv_response = client.get(csv_url_scrp).send().await?;
    let csv_content = csv_response.text().await?;

    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let mut vote_records: Vec<VoteRecord> = Vec::new();

    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let muni_idx =
            get_column_index(&headers, "gebiet-name").context("Failed to get column index")?;
        let name_muni = record[muni_idx].to_string();
        let mut votes = HashMap::new();

        for (position, party_name) in party_map {
            let party_idx =
                get_column_index(&headers, position).context("Failed to get column index")?;
            if let Ok(vote_count) = record[party_idx].parse::<i32>() {
                votes.insert(party_name.to_string(), vote_count);
            } else {
                return Err(anyhow!(
                    "Failed tp parse votes for municipality {}",
                    name_muni
                ));
            }
        }

        let geometry = geom_map.get(&name_muni).map(|polygon| polygon.to_owned());

        let vote_record = VoteRecord::new(name_muni, votes, geometry);
        vote_records.push(vote_record);
    }
    let vote = Vote {
        name: name.to_string(),
        vote_records,
    };
    Ok(vote)
}

fn get_column_index(headers: &csv::StringRecord, column_name: &str) -> Option<usize> {
    headers.iter().position(|h| h == column_name)
}
