use vis_municipality_votes::app::data::get_data;
use vis_municipality_votes::harvester::{load_config, Config};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Let's start our Server
    let config = load_config();
    dbg!(&config);
    
    // let _ = get_data().await;
    Ok(())
}
