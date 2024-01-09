use std::error::Error;

use dotenv::dotenv;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client, Database,
};

pub async fn connect_cluster() -> Result<Database, Box<dyn Error>> {
    dotenv().ok();
    let database_url =
        std::env::var("MONGODB_URI").expect("MONGODB URL is expected to connec to cluster");
    let client_options = ClientOptions::parse(database_url).await?;
    let client = Client::with_options(client_options).expect("Database couldnt connect");
    println!("Database Connected");
    let database = client.database("diagnoai");

    Ok(database)
}
