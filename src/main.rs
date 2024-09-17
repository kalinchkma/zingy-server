use zingy::startup;


#[tokio::main]
async fn main() {
    // get application
    let (address, app) = startup::application().await.unwrap_or_else(|e| {
        panic!("Error: {:?}", e)
    });

    // create tcp listener
    // let listener = tokio::net::TcpListener::bind(address).await.unwrap_or_else(|e| {
    //     panic!("Error {:?}",e);
    // });

    // println!("Server running on {:?}", address);

    // run the server
    // axum::serve(listener, app).await.unwrap_or_else(|e| {
    //     panic!("Error {:?}", e)
    // })

}
