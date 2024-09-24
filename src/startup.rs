use std::net::SocketAddr;

use askama_axum::Template;
use axum::{http::StatusCode, response::{IntoResponse, Html}, routing::get, Router};
use tower_http::services::ServeDir;

use crate::configuration::{database::DatabaseConfiguration, environtment::Environtment};

// template
#[derive(Template)]
#[template(path = "home.html")]
struct Home<'a> {
    name: &'a str
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    name: &'a str,
    desciption: &'a str
}

async fn home() -> impl IntoResponse {
    let idx = Home {
        name: "Hunter"
    };
    (StatusCode::OK, Html(idx.render().unwrap()))
}

async fn index() -> impl IntoResponse {
    let idx = Index {
        name: "Zingy",
        desciption: "This is the first template"
    };
    (StatusCode::OK, Html(idx.render().unwrap()))
}

#[derive(Template)]
#[template(path = "test/test.html")]
struct Test<'a> {
    name: &'a str
}
async  fn test() -> impl IntoResponse {
    let t = Test {
        name: "Mr hunter"
    };
    (StatusCode::OK, Html(t.render().unwrap()))
}

// static file servering
fn static_route() -> Router {
    Router::new().nest_service("/static", ServeDir::new("./public"))
}

// main server stratup function
pub async fn application() -> Result<(SocketAddr, Router), sqlx::Error> {
    // get environtment variables
    let envs = Environtment::new();

    // connect the database
    let _pool = DatabaseConfiguration::new(envs.db_connection_string, envs.db_max_pool).connect().await?;

    // build appication
    let app: Router = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/home", get(home))
        .route("/index", get(index))
        .route("/test", get(test))
        .merge(static_route());

    // return 
    Ok((envs.address, app))
}
