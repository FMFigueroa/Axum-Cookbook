use anyhow::Result;
use serde_json::json;

// cmd: cargo watch -q -c -w examples/ -x 'run --example quick_dev'
// test client with httpc-test.
#[tokio::main]
async fn main() -> Result<()> {
    // Create Client
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // test Static Route
    hc.do_get("/").await?.print().await?;

    // test request login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd":"welcome"
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
