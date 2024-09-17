use std::net::SocketAddr;

use axum::{routing::get, Router};

use crate::configuration::{database::DatabaseConfiguration, environtment::Environtment};

use toml;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Deserialize)]
struct Data {
    config: Config
}

#[derive(Deserialize)]
struct Config {
    port: u16,
    database_user: String
}


// main server stratup function
pub async fn application() -> Result<(String, Router), sqlx::Error> {
    // get environtment variables
    // let envs = Environtment::new(
    //     "PORT", 
    //     "DB_USER",
    //     "DB_PASSWORD",
    //     "DB_HOST",
    //     "DB_NAME",
    //     "DB_MAX_POOL"
    // );

    // connect the database
    // let pool = DatabaseConfiguration::new(envs.db_connection_string, envs.db_max_pool).connect().await?;

    // println!("{:?}", pool);
    let content = match fs::read_to_string("Config.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file ");
            exit(1);
        }
    };

    // data
    let data: Data = match toml::from_str(&content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data ");
            exit(1);
        }
    };

    println!("port {}, db_user {}", data.config.port, data.config.database_user);


    // build appication
    let app: Router = Router::new().route("/", get(|| async { "Hello world" }));

    // return 
    Ok(("".to_owned(), app))
}
