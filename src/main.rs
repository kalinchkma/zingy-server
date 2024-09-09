use zingy::startup;




#[tokio::main]
async fn main() {
    // get application
    let (envs, app) = startup::application();

    // create tcp listener
    let listener = tokio::net::TcpListener::bind(envs.address).await.unwrap_or_else(|e| {
        panic!("Error {:?}",e);
    });

    println!("Server running on {:?}", envs.address);

    // run the server
    axum::serve(listener, app).await.unwrap_or_else(|e| {
        panic!("Error {:?}", e)
    })

}
