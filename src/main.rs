
use axum::{routing::get, Router};
use zingy::logs;
use dotenv::dotenv;
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = env::var("PORT").expect("PORT is needed to run the server");
    let host = env::var("HOST").expect("HOST is needed to run the the server");

    let host_address: SocketAddr= format!("{host}:{port}").parse().expect("Cannot make an address form given host and port");

    let app = Router::new().route("/", get(|| async {"Hello world"}));

    logs::run();

    // region:     -- start server
    println!("Listening on port {}", host_address);
    let listener = tokio::net::TcpListener::bind(host_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    // endregion:  -- start server
}
