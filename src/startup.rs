use axum::{routing::get, Router};

use crate::configuration::environtment::Environtment;

// main server stratup function
pub fn application() -> (Environtment, Router) {
    // get environtment variables
    let envs = Environtment::new("PORT");

    // build appication
    let app: Router = Router::new().route("/", get(|| async { "Hello world" }));

    // return 
    (envs, app)
}