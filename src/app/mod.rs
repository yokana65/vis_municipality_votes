use actix_web::web::ServiceConfig;
use actix_web::{web, HttpResponse};

use crate::app::data::{data_items, read_file};

pub mod data;

pub fn views_factory(app: &mut ServiceConfig) {
    app_views_factory(app);
}

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", web::get().to(data_items));
    app.route("/v1/map", web::get().to(map));
}

pub async fn map() -> HttpResponse {
    println!("Current dir: {:?}", std::env::current_dir().unwrap());
    let mut html_map = read_file("./templates/map.html");
    let javascript_data = read_file("./src/javascript/map.js");

    html_map = html_map.replace("{{JAVASCRIPT}}", &javascript_data);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_map)
}
