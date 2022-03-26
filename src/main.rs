#[tokio::main]
async fn main() {
    // run it
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(rustapi::app().into_make_service())
        .await
        .unwrap();
}
