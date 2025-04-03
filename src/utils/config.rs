use dotenvy::dotenv;

pub fn load_env() {
  dotenv().ok();
}

pub fn get_db_uri() -> String {
  std::env::var("DATABASE_URL").unwrap_or_else(|_| "mongodb://localhost:27017".to_string())
}

pub fn get_port() -> u16 {
  std::env
    ::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .unwrap_or(3000)
}
