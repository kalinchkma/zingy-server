use std::net::SocketAddr;

pub struct Environtment {
   pub address: SocketAddr,
}

impl Environtment {
    pub fn new(port_key: &str, db_user_key: &str, db_password_key: &str, db_host: &str) -> Self {
        use dotenv::dotenv;
        use std::env;
        
        dotenv().ok();
        
        // parse port from environtment variable
        let port = env::var(port_key).unwrap_or_else(|_| {
            "6969".to_string()
        });

        // format address string
        let address: SocketAddr = format!("0.0.0.0:{port}").parse().expect("Cannot create socket address from envs");

        Self {
            address            
        }
        
    }
}







