use anyhow::Result;
use serde_json::json;

// cmd: cargo watch -q -c -w examples/ -x 'run --example quick_dev'

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    // test Query Params
    hc.do_get("/hello?name=Jen").await?.print().await?;

    // test Path Params
    hc.do_get("/hello2/Mike").await?.print().await?;

    // test Static Route
    hc.do_get("/").await?.print().await?;

    // request login
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd":"welcome"
        }),
    );
    req_login.await?.print().await?;

    // create_ticket
    let req_create_ticket = hc.do_post(
        "/api/tickets",
        json!({
            "title":"Ticket AAA"
        }),
    );
    req_create_ticket.await?.print().await?;

    // delete_ticket
    //hc.do_delete("/api/tickets/1").await?.print().await?;

    // get_tickets
    hc.do_get("/api/tickets").await?.print().await?;

    Ok(())
}
