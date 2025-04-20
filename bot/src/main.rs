mod config;
pub mod core;

use crate::config::*;

#[tokio::main]
async fn main() {
    discord::configure_discord().await;
    database::connect_to_database().await;
}
