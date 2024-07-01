use std::time::Duration;

use anyhow::Result;
use csv::Reader;
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

        let mut reader = Reader::from_reader(csv_content.as_bytes());
        let mut votes = Vec::new();

        for result in reader.records() {
            let record = result?;
            println!("{:?}", record);
            if record.len() >= 2 {
                votes.push(Vote {
                    name: record[0].to_string(),
                    count: record[1].parse().unwrap_or(0),
                });
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct Vote {
    pub name: String,
    pub count: u32,
}
