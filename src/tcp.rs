//! TCP proxy module.

use tokio::{net::{TcpListener, TcpStream}, io::copy_bidirectional};
use anyhow::Result;

/// Start a plain TCP proxy from `bind_addr` to `target_addr`.
///
/// This is raw passthrough — bytes are copied in both directions without encryption.
///
/// ## Example
/// ```no_run
/// tokio_proxy::start_tcp("0.0.0.0:8080", "127.0.0.1:9000").await?;
/// ```
pub async fn start_tcp(bind_addr: &str, target_addr: &str) -> Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    println!("TCP proxy listening on {}", bind_addr);

    loop {
        let (inbound, _) = listener.accept().await?;
        let target = target_addr.to_string();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(inbound, &target).await {
                eprintln!("TCP Proxy error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut inbound: TcpStream, target_addr: &str) -> Result<()> {
    let mut outbound = TcpStream::connect(target_addr).await?;
    copy_bidirectional(&mut inbound, &mut outbound).await?;
    Ok(())
}

