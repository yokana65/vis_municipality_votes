use actix_web::web::ServiceConfig;
use actix_web::web;

use crate::app::data::data_items;

mod data;

pub fn views_factory(app: &mut ServiceConfig) {
    app_views_factory(app);
}

pub fn app_views_factory(app: &mut ServiceConfig) {
    app.route("/", web::get().to(data_items));
}

