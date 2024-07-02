use std::time::Duration;
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use csv::ReaderBuilder;
use scraper::{Html, Selector};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder().timeout(Duration::from_secs(300)).build()?;

    let url_votes = "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024";

    let body = reqwest::get(url_votes)
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a[href$='.csv'][href*='Ortsteil']").expect("Failed to parse CSS selector for CSV link");

    if let Some(link) = document.select(&selector).next() {
        println!("Scraping data from: {:?}", link.value().attr("href"));
        let csv_url = link.value().attr("href").unwrap();
        let csv_response = client.get(csv_url).send().await?;
        let csv_content = csv_response.text().await?;

        let mut reader = ReaderBuilder::new()
        .delimiter(b';')  
        .has_headers(true)
        .from_reader(csv_content.as_bytes());        
    
        let mut vote_records: Vec<VoteRecord> = Vec::new();

        let party_positions = [17, 29, 41, 53, 65, 77, 89, 99];
        let party_labels = ["Die Linke", "Grüne", "CDU", "AfD", "SPD", "FDP", "Die Partei", "BSW"];

        let headers = reader.headers()?.clone();
  
        if headers[17] != *"E1" {
            return Err(anyhow!("CSV does not follow the assumed order of parties."));
        }


        for result in reader.records() {
            let record = result?;

            let name_muni = record[4].to_string();
            let mut votes: HashMap<String, i16> = HashMap::new();

            for (&position, &label) in party_positions.iter().zip(party_labels.iter()) {
                if let Ok(vote_count) = record[position].parse::<i16>() {
                    votes.insert(label.to_string(), vote_count);
                } else {
                    return Err(anyhow!("Failed tp parse votes for municipality {}", name_muni));
                }
            }

            let vote_record = VoteRecord {
                name_muni,
                votes,
            };

            vote_records.push(vote_record);
        }


        let vote = Vote {
            name: "Leipzig Stadtratswahl 2024".to_string(),
            vote_records,

        };

        println!("Vote: {:?}", vote);

    }
    Ok(())
}

#[derive(Debug)]
pub struct Vote {
    pub name: String,
    pub vote_records: Vec<VoteRecord>,
}

#[derive(Debug)]
pub struct VoteRecord {
    pub name_muni: String,
    pub votes: HashMap<String, i16>,
}

// Parties in Leipzig are coded as follows:
// Die Linke: E1
// Grüne: E2
// CDU: E3
// AfD: E4
// SPD: E5
// FDP: E6
// Die PARTEI: E7
// BSW: E8
