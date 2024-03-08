use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion}, Client
};
use dotenv::var;

pub async fn connect() -> Client {
    let username = var("MONGO_USERNAME").unwrap();
    let password = var("MONGO_PASSWORD").unwrap();
    let host = var("MONGO_HOST").unwrap();

    let mut client_options = ClientOptions::parse(
        format!("mongodb+srv://{username}:{password}@{host}/?retryWrites=true&w=majority")
    ).await.unwrap();

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options).unwrap();

    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await.expect("Database connection failed");

    println!("successfully connected to database");
    return client;
}