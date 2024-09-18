use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::configuration::{database::DatabaseConfiguration, environtment::Environtment};



// main server stratup function
pub async fn application() -> Result<(SocketAddr, Router), sqlx::Error> {
    // get environtment variables
    let envs = Environtment::new();

    // connect the database
    let _pool = DatabaseConfiguration::new(envs.db_connection_string, envs.db_max_pool).connect().await?;

    // build appication
    let app: Router = Router::new().route("/", get(|| async { "Hello world" }));

    // return 
    Ok((envs.address, app))
}
