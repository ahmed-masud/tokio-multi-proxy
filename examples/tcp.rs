use tokio_proxy::start_tcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting TCP proxy: 0.0.0.0:8080 → 127.0.0.1:9000");
    start_tcp("0.0.0.0:8080", "127.0.0.1:9000").await
}

