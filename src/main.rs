use rustapi::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}
