mod config;
mod db;
mod repositories;
mod models;

use config::Config;
use db::Db;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let config = match Config::load() {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to load configuration: {err}");
            std::process::exit(1);
        }
    };

    let db = match Db::new(&config.database).await {
        Ok(db) => {
            info!("Database connection established");
            db
        }
        Err(err) => {
            error!("Failed to connect to database: {err}");
            std::process::exit(1);
        }
    };

    if let Err(err) = db.migrate().await {
        error!("Failed to run migrations: {err}");
        std::process::exit(1);
    }

    info!("Database migrations completed");
}