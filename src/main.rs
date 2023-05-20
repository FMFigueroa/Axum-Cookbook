use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;

//cmd: cargo watch -q -c -w src/ -x run

// region: ---Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler-hello", "HANDLER");
    Html("Hello <strong>World!!!</strong>")
    // endregion: ---Handler Hello
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    // endregion: ---Start Server
}
