use std::net::SocketAddr;

pub struct Environtment {
   pub address: SocketAddr,
   pub db_connection_string: String,
   pub db_max_pool: u32
}

impl Environtment {
    pub fn new(port_key: &str, db_user: &str, db_password: &str, db_host: &str,db_name: &str, max_pool: &str,) -> Self {
        use dotenv::dotenv;
        use std::env;
        
        dotenv().ok();
        
        // parse port from environtment variable
        let port = env::var(port_key).unwrap_or_else(|_| {
            "6969".to_string()
        });

        // format address string
        let address: SocketAddr = format!("0.0.0.0:{port}").parse().expect("Cannot create socket address from envs");

        // parse database user
        let db_user = env::var(db_user).expect("Database user name is required");    

        // parse database password
        let db_password = env::var(db_password).expect("Database password is required");

        // parse database name
        let db_name = env::var(db_name).expect("Database name is required");

        // parse database host
        let db_host = env::var(db_host).expect("Database host is required");

        // create database connection
        let db_connection_string = format!("postgres://{db_user}:{db_password}@{db_host}/{db_name}");   

        // parse database max pool number
        let db_max_pool = env::var(max_pool).unwrap_or_else(|_| {
            "5".to_string()
        }).parse::<u32>().expect("Cannot parse a database max pool");

        Self {
            address,
            db_connection_string,
            db_max_pool    
        }
        
    }
}







