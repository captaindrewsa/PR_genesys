mod mirror_json;
mod hello_world;
use axum::{body::Body, routing::{get, post}, Router};
use hello_world::hello_world;
use mirror_json::mirror_json;




pub fn create_routes()-> Router<> {
    axum::Router::new().route("/", get(hello_world))
        .route("/mirror_json", post(mirror_json))

}