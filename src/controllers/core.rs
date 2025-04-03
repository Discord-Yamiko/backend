use axum::extract::Multipart;
use tokio::fs::{ File, remove_file };
use tokio::io::AsyncWriteExt;
use std::fs;

pub async fn save_file(multipart: &mut Multipart) -> Result<String, ()> {
  while let Some(mut field) = multipart.next_field().await.unwrap_or(None) {
    if let Some(filename) = field.file_name().map(|name| name.to_string()) {
      let path = format!("./assets/backgrounds/{}", filename);
      let mut file = File::create(&path).await.map_err(|_| ())?;
      while let Some(chunk) = field.chunk().await.map_err(|_| ())? {
        file.write_all(&chunk).await.map_err(|_| ())?;
      }
      return Ok(filename);
    }
  }
  Err(())
}

pub fn file_exists(filename: &str) -> bool {
  let path = format!("./assets/backgrounds/{}", filename);
  fs::metadata(path).is_ok()
}

pub async fn delete_file(filename: &str) -> bool {
  let path = format!("./assets/backgrounds/{}", filename);
  remove_file(path).await.is_ok()
}
