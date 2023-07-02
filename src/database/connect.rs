use mongodb::{bson, options::ClientOptions, Client, Database};
use rocket::fairing::AdHoc;
use crate::env::MONGO_URI;

pub struct MongoDB {
    pub(crate) database: Database,
}

pub async fn init() -> AdHoc {
    AdHoc::on_ignite("Mongodb oK!", |rocket| async{
        match  connect().await {
            Ok(database) => rocket.manage(MongoDB::new(database)),
            Err(error) => {
                panic!("Cannot connect to MDB instance:: {:?}", error)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let mut client_options = ClientOptions::parse(MONGO_URI).await?;
    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(bson::doc! {"ping":1}, None)
        .await?;
    println!("connected!");
    Ok(client.database("authorization"))
}