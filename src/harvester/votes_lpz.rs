use std::collections::HashMap;

use anyhow::{anyhow, Result};
use csv::ReaderBuilder;
use reqwest::Client;
use scraper::{Html, Selector};

use crate::{
    harvester::muni_geo::fetch_geom,
    structs::votes::{Vote, VoteRecord},
};

pub async fn harvest_votes(client: &Client, url: &str, name: &str) -> Result<Vote> {
    let body = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href$='.csv'][href*='Ortsteil']")
        .expect("Failed to parse CSS selector for CSV link");

    let link = document
        .select(&selector)
        .next()
        .expect("No CSV Link found on page");
    println!("Scraping data from: {:?}", link.value().attr("href"));
    let csv_url = link.value().attr("href").unwrap();
    let csv_response = client.get(csv_url).send().await?;
    let csv_content = csv_response.text().await?;

    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(csv_content.as_bytes());

    let mut vote_records: Vec<VoteRecord> = Vec::new();

    // Parties in Leipzig are coded as follows:
    // Die Linke: E1
    // Grüne: E2
    // CDU: E3
    // AfD: E4
    // SPD: E5
    // FDP: E6
    // Die PARTEI: E7
    // BSW: E8
    let party_positions = [17, 29, 41, 53, 65, 77, 89, 99];
    let party_labels = [
        "Die Linke",
        "Grüne",
        "CDU",
        "AfD",
        "SPD",
        "FDP",
        "Die Partei",
        "BSW",
    ];

    let headers = reader.headers()?.clone();

    if headers[17] != *"E1" {
        return Err(anyhow!("CSV does not follow the assumed order of parties."));
    }

    let geom_map = fetch_geom(&client).await?;

    for result in reader.records() {
        let record = result?;
        let name_muni = record[4].to_string();
        let mut votes = HashMap::new();

        for (&position, &label) in party_positions.iter().zip(party_labels.iter()) {
            if let Ok(vote_count) = record[position].parse::<i32>() {
                votes.insert(label.to_string(), vote_count);
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
