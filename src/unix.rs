//! Unix domain socket proxy module.
//!
//! Listens on a local UDS socket and forwards incoming stream connections
//! to a target TCP address.
//!
//! Useful for local inter-process communication or socket activation scenarios.

use tokio::{net::{UnixListener, UnixStream, TcpStream}, io::copy_bidirectional};
use anyhow::Result;

/// Start a Unix domain socket proxy from `unix_path` to `target_addr`.
///
/// Removes the existing socket file if it exists before binding.
///
/// ## Example
/// ```no_run
/// async {
///     tokio_proxy::start_unix("/tmp/proxy.sock", "127.0.0.1:9000").await.unwrap();
/// };
/// ```
pub async fn start_unix(unix_path: &str, target_addr: &str) -> Result<()> {
    let _ = std::fs::remove_file(unix_path); // Clean up old socket
    let listener = UnixListener::bind(unix_path)?;
    println!("Unix socket proxy listening on {}", unix_path);

    loop {
        let (stream, _) = listener.accept().await?;
        let target = target_addr.to_string();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, &target).await {
                eprintln!("Unix proxy error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut inbound: UnixStream, target_addr: &str) -> Result<()> {
    let mut outbound = TcpStream::connect(target_addr).await?;
    copy_bidirectional(&mut inbound, &mut outbound).await?;
    Ok(())
}

