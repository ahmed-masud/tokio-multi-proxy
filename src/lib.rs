//! Tokio Proxy: multi-interface asynchronous proxy library using Tokio.
//!
//! # Example
//! ```rust
//! tokio_proxy::start_tcp("0.0.0.0:8000", "127.0.0.1:9000").await?;
//! ```

pub mod tcp;
pub mod udp;
pub mod unix;

pub use tcp::start_tcp;
pub use udp::start_udp;
pub use unix::start_unix;

