use axum::{ extract::{ Multipart, Path }, response::Json, http::StatusCode };
use serde_json::json;
use crate::controllers::core;

pub async fn upload_background(mut multipart: Multipart) -> Json<serde_json::Value> {
  match core::save_file(&mut multipart).await {
    Ok(filename) => Json(json!({ "status": "success", "filename": filename })),
    Err(_) => Json(json!({ "status": "error" })),
  }
}

pub async fn get_background(Path(filename): Path<String>) -> Result<impl axum::response::IntoResponse, StatusCode> {
  if core::file_exists(&filename) {
    let path = format!("./assets/backgrounds/{}", filename);
    match tokio::fs::read(&path).await {
      Ok(file_data) => {
        let content_type = match filename.split('.').last() {
          Some("jpg") | Some("jpeg") => "image/jpeg",
          Some("png") => "image/png",
          _ => "application/octet-stream",
        };

        Ok(([(axum::http::header::CONTENT_TYPE, content_type)], file_data))
      }
      Err(_) => Err(StatusCode::NOT_FOUND),
    }
  } else {
    Err(StatusCode::NOT_FOUND)
  }
}

pub async fn delete_background(Path(filename): Path<String>) -> Json<serde_json::Value> {
  if core::delete_file(&filename).await { Json(json!({ "status": "success" })) } else { Json(json!({ "status": "error" })) }
}
