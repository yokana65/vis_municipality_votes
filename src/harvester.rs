use anyhow::Result;
use env_logger;
use scraper::{Html, Selector};
use reqwest::Client;
use csv::Reader;

use std::time::Duration;

#[derive(Debug)]
pub struct Vote {
    pub name: String,
    pub count: u32,
}

pub async fn harvest(source: &str) -> Result<()> {
    println!("Harvesting {}", source);

    if source.contains("votes") {
        harvest_votes(source).await?;
    }

    Ok(())
}

pub async fn harvest_votes(_source: &str) -> Result<()> {
    env_logger::init();

    // let url = "https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024";
    let url = "https://www.weltwaerts-uganda.org";

    println!("Getting the votes from {}", url);

    let body = reqwest::get("https://www.leipzig.de/buergerservice-und-verwaltung/wahlen-in-leipzig/stadtratswahlen/stadtratswahl-2024")
    .await?
    .text()
    .await?;

    println!("body = {body:?}");
    // let client = Client::builder()
    // .timeout(Duration::from_secs(30))
    // .build()?;

    // let response = client.get(url).send().await?;
    // let body = response.text().await?;

    // let _document = Html::parse_document(&body);
    // let selector = Selector::parse("a[href$='.csv']").unwrap();

    Ok(())
}
