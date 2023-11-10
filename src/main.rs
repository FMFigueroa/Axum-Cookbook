pub use self::error::{Error, Result};
use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod error;
mod web;

//cmd: cargo watch -q -c -w src/ -x run

// region: ---Handler Hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler-hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("world!");
    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler-hello2 - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
// endregion: ---Handler Hello

// region: ---Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
// endregion: ---Routes Hello

// region: ---Static Route
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("public/")))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RESP_MAPPER");

    println!();

    res
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion: ---Start Server
}
