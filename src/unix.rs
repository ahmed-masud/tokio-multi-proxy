use tokio::{net::{UnixListener, UnixStream, TcpStream}, io::copy_bidirectional};
use anyhow::Result;

/// Start a UNIX domain socket proxy from `unix_path` to `target_addr`.
pub async fn start_unix(unix_path: &str, target_addr: &str) -> Result<()> {
    let _ = std::fs::remove_file(unix_path);
    let listener = UnixListener::bind(unix_path)?;
    println!("UNIX socket proxy listening on {}", unix_path);

    loop {
        let (stream, _) = listener.accept().await?;
        let target = target_addr.to_string();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, &target).await {
                eprintln!("UNIX Proxy error: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut inbound: UnixStream, target_addr: &str) -> Result<()> {
    let mut outbound = TcpStream::connect(target_addr).await?;
    copy_bidirectional(&mut inbound, &mut outbound).await?;
    Ok(())
}

