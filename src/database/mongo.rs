use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, bson::{Bson, doc}};
use rocket::{tokio, futures::StreamExt};


fn env_var(var: &str) -> String {
    dotenv().ok();
    std::env::var(var).unwrap()
}

#[tokio::main]
async fn main() {
    let db_url = env_var("MONGO_URI");
    let db_name = "test";

    let mut client_options = ClientOptions::parse(&db_url).await.unwrap();
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).unwrap();
    let db = client.database(db_name);

    let collection = db.collection("users");
    let user = doc! { "name": "John Doe" };

    collection.insert_one(user, None).await.unwrap();

    let mut cursor = collection.find(None, None).await.unwrap();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(name) = document.get("name").and_then(Bson::as_str) {
                    println!("name: {}", name);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

