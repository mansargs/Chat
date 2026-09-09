use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::config::DbConfig;

pub struct Db {
    pub pool : PgPool,
}

impl Db {
    pub async fn new(config : &DbConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .connect(&config.db_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
    }
}
