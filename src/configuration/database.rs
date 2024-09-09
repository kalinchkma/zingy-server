use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub struct DatabaseConfiguration {
    connection_string: String,
    max_pool_number: u32
}

impl  DatabaseConfiguration {
    // create new database configuration
    pub fn new(connection_string: String, max_pool_number: u32) -> Self {
        Self {
            connection_string,
            max_pool_number
        }
    }

    // create connection to the database return a pool of connection
    pub async fn connect(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(self.max_pool_number)
            .connect(&self.connection_string).await?;
        Ok(pool)
    }
}

