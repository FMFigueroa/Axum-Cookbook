use anyhow::Result;

//cmd: cargo watch -q -c -w examples/ -x 'run --example quick_dev'

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    Ok(())
}
