use axum::{ extract::State, http::StatusCode, middleware::Next, body::Body };
use mongodb::{ Collection, bson::doc };
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
  pub db: Collection<mongodb::bson::Document>,
}

pub async fn check_session(
  State(state): State<Arc<AppState>>,
  req: axum::http::Request<Body>,
  next: Next
) -> Result<axum::http::Response<axum::body::Body>, StatusCode> {
  let session_key = req
    .headers()
    .get("x-session-key")
    .and_then(|v| v.to_str().ok());

  if let Some(key) = session_key {
    let db = &state.db;
    let session = db.find_one(doc! { "session_key": key }, None).await.unwrap();

    if let Some(_) = session {
      db.delete_one(doc! { "session_key": key }, None).await.unwrap();

      return Ok(next.run(req).await);
    }
  }

  Err(StatusCode::UNAUTHORIZED)
}
