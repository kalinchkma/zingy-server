use std::net::SocketAddr;

pub struct Environtment {
   pub address: SocketAddr,
}


impl Environtment {
    pub fn new(port_key: &str) -> Self {
        use dotenv::dotenv;
        use std::env;
        
        dotenv().ok();
        
        // parse port from environtment variable
        let port = env::var(port_key).expect("Port environtment variable is required");

        // format address string
        let address: SocketAddr = format!("0.0.0.0:{port}").parse().expect("Cannot create socket address from envs");

        Self {
            address            
        }
        
    }
}







