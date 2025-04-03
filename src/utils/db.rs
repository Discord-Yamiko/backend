use mongodb::{ Client, Collection };

pub async fn connect_to_db(uri: String) -> Collection<mongodb::bson::Document> {
  let client = Client::with_uri_str(uri).await.unwrap();
  let db = client.database("beta");
  db.collection("BackgroundSessions")
}
