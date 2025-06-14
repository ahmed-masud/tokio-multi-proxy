use tokio_multi_proxy::start_tls_tcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting TLS proxy: 0.0.0.0:8443 → 127.0.0.1:9000");
    start_tls_tcp(
        "0.0.0.0:8443",
        "127.0.0.1:9000",
        "examples/cert.pem",
        "examples/key.pem"
    ).await
}

