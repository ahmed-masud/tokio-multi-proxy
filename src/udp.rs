//! UDP proxy module.
//!
//! This sets up a stateless UDP forwarder that relays packets from a bind address to a target address,
//! and relays replies back to the original sender.
//!
//! Ideal for scenarios like forwarding DNS, QUIC, or SIP over UDP.

use tokio::net::UdpSocket;
use anyhow::Result;

/// Start a UDP proxy from `bind_addr` to `target_addr`.
///
/// This forwards datagrams from external clients to the internal `target_addr`,
/// and replies from the target back to the original client.
///
/// ⚠️ This is a naive implementation without connection tracking or session isolation.
///
/// ## Example
/// ```no_run
/// tokio_proxy::start_udp("0.0.0.0:5353", "127.0.0.1:8053").await?;
/// ```
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

