mod harvester;

use harvester::harvest;

use anyhow::{Result, Context};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let sources = vec!["votes", "area_lpz"];
    let mut tasks = JoinSet::new();

    for source in sources {
        let source = source.to_string();
        tasks.spawn(async move {
            let result = harvest(&source)
                .await
                .with_context(|| format!("Failed to harvest {}", source));

            if result.is_err() {
                eprintln!("Error: {:?}", result);
            }
            
        });
    }

    Ok(())
}
