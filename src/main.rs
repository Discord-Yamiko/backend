mod middleware;
mod utils;
mod handlers;
mod controllers;

use axum::{ Router, routing::{ get, post, delete }, middleware::from_fn_with_state };
use axum_server::Server;
use middleware::auth::check_session;
use crate::handlers::backgrounds::{ get_background, upload_background, delete_background };
use tower_http::cors::CorsLayer;
use utils::{ config::{ get_port, load_env }, db::connect_to_db };
use std::sync::Arc;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  load_env();
  let db_uri = utils::config::get_db_uri();
  let db = connect_to_db(db_uri).await;
  let state = Arc::new(middleware::auth::AppState { db });

  let app = Router::new()
    .route("/api/backgrounds", post(upload_background))
    .route("/api/backgrounds/:filename", delete(delete_background))
    .layer(from_fn_with_state(state, check_session))
    .route("/api/backgrounds/:filename", get(get_background))
    .layer(CorsLayer::permissive());
  let port = get_port();
  let addr = SocketAddr::from(([0, 0, 0, 0], port));

  println!("BACKEND images on {}", addr);

  Server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
