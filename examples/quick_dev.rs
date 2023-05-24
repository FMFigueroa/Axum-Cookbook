use anyhow::Result;
use serde_json::json;

//cmd: cargo watch -q -c -w examples/ -x 'run --example quick_dev'

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    //test Query Params
    hc.do_get("/hello?name=Jen").await?.print().await?;

    //test Path Params
    hc.do_get("/hello2/Mike").await?.print().await?;

    //test Static Route
    hc.do_get("/").await?.print().await?;

    //request login
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
