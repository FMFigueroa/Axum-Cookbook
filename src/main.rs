use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

//cmd: cargo watch -q -c -w src/ -x run

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async { Html("Hello <strong>World!!!</strong>") }),
    );

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr}");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: ---Start Server
}
