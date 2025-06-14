use tokio_multi_proxy::start_unix;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting Unix proxy: /tmp/proxy.sock → 127.0.0.1:9000");
    start_unix("/tmp/proxy.sock", "127.0.0.1:9000").await
}

