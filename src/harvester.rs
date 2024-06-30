use anyhow::Result;

pub async fn harvest(source: &str) -> Result<()> {
    println!("Harvesting {}", source);
    Ok(())
}
