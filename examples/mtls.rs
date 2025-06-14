use tokio_multi_proxy::start_mtls_tcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting mTLS proxy: 0.0.0.0:9443 → 127.0.0.1:9000");
    start_mtls_tcp(
        "0.0.0.0:9443",
        "127.0.0.1:9000",
        "examples/cert.pem",
        "examples/key.pem",
        "examples/ca.pem"
    ).await
}

