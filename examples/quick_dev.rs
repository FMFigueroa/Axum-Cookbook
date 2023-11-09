use anyhow::Result;

//cmd: cargo watch -q -c -w examples/ -x 'run --example quick_dev'

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Jen").await?.print().await?;

    hc.do_get("/hello2/Mike").await?.print().await?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}
