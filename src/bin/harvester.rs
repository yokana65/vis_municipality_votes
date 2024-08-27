use vis_municipality_votes::app::data::get_data;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Let's start our Server
    let _ = get_data().await;
    Ok(())
}
