use std::{fs, net::SocketAddr, process::exit};
use serde_derive::Deserialize;
use toml;

use super::CONFIG_PATH;

#[derive(Deserialize)]
struct EnvirontmentData {
    database: Database,
    env: Env
}

#[derive(Deserialize)]
struct Database {
    db_user: String,
    db_password: String,
    db_host: String,
    db_name: String,
    db_max_pool: u32
}

#[derive(Deserialize)]
struct Env {
    port: i32
}

#[derive(Debug)]
pub struct Environtment {
   pub address: SocketAddr,
   pub db_connection_string: String,
   pub db_max_pool: u32
}

impl Environtment {
    pub fn new() -> Self {
        // read environtment form path of CONFIG_PATH as string
        let env_contents = match fs::read_to_string(CONFIG_PATH) {
            Ok(content) => {
               content
            },
            Err(e) => {
                eprintln!("{}", e);
                exit(1)
            }
        };
        
        // convert environtment string to serialize environtment data
        let envs: EnvirontmentData = match toml::from_str(&env_contents) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("{}", e);
                exit(1)
            }
        }; 

        // format the address string
        let address: SocketAddr = format!("0.0.0.0:{}", envs.env.port).parse().expect("Connot create socket address fron configuration");

        // create database connection string
        let db_connection_string = format!("postgres://{}:{}@{}/{}",envs.database.db_user, envs.database.db_password, envs.database.db_host, envs.database.db_name);

        Self {
            address,
            db_connection_string,
            db_max_pool: envs.database.db_max_pool
        }
    }
}







