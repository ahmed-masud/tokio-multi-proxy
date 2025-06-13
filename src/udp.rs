use tokio::net::UdpSocket;
use anyhow::Result;

/// Start a UDP proxy from `bind_addr` to `target_addr`.
pub async fn start_udp(bind_addr: &str, target_addr: &str) -> Result<()> {
    let socket = UdpSocket::bind(bind_addr).await?;
    let target = UdpSocket::bind("0.0.0.0:0").await?;
    target.connect(target_addr).await?;
    println!("UDP proxy listening on {}", bind_addr);

    let mut buf = [0u8; 2048];

    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        target.send(&buf[..len]).await?;

        let len = target.recv(&mut buf).await?;
        socket.send_to(&buf[..len], &peer).await?;
    }
}

