use tokio_proxy::start_udp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting UDP proxy: 0.0.0.0:5353 → 127.0.0.1:8053");
    start_udp("0.0.0.0:5353", "127.0.0.1:8053").await
}

