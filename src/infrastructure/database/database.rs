use sea_orm::{Database, DatabaseConnection};
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::OnceCell;

pub static DATABASE_INSTANCE: OnceCell<Arc<DatabaseConnection>> = OnceCell::const_new();
pub async fn connect_to_database() {
    const SQLITE_PATH: &'static str = "mix.sqlite";

    if !Path::new(SQLITE_PATH).exists() {
        println!("Database file not exists, creating...");

        File::create(SQLITE_PATH).expect("Failed to create sqlite file!");
    };

    let database_url = format!("sqlite://{}", SQLITE_PATH);

    let database = Database::connect(database_url)
        .await
        .expect("Failed to create database connection");

    DATABASE_INSTANCE.set(Arc::new(database))
        .expect("Failed to set database instance!");
}
