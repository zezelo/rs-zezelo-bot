pub mod infrastructure;
pub mod core;

use infrastructure::discord;
use crate::infrastructure::*;

#[tokio::main]
async fn main() {
    database::database::connect_to_database().await;
    discord::configure_discord().await;
}
