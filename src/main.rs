use actix_service::Service;
use actix_web::{App, HttpServer};

use app::views_factory;

mod harvester;
mod structs;
mod app;

#[tokio::main]
async fn main() -> std::io::Result<()> {


    // Let's start our Server
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views_factory);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
