//! # tokio-multi-proxy
//!
//! `tokio-multi-proxy` is a modular, async multi-interface proxy built with [Tokio](https://tokio.rs) and [Rustls](https://github.com/rustls/rustls).
//!
//! It supports three modes of operation:
//!
//! - **Plain TCP passthrough** (default)
//! - **TLS termination** (`--features tls`)
//! - **Mutual TLS (mTLS)** with client authentication (`--features mtls`)
//!
//! ## Feature Flags
//!
//! | Feature     | Description                         |
//! |-------------|-------------------------------------|
//! | `passthrough` | Raw TCP proxying (enabled by default) |
//! | `tls`         | TLS termination on incoming connections |
//! | `mtls`        | Mutual TLS (requires `tls`)           |
//!
//! ## Example
//!
//! ```rust,no_run
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     tokio_multi_proxy::start_tcp("0.0.0.0:8080", "127.0.0.1:9000").await
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]


pub mod tcp;
pub mod udp;
pub mod unix;
#[cfg(feature = "tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
pub mod tls;


pub use tcp::start_tcp;
pub use udp::start_udp;
pub use unix::start_unix;


#[cfg(feature = "tls")]
#[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
pub use tls::start_tls_tcp;

#[cfg(feature = "mtls")]
#[cfg_attr(docsrs, doc(cfg(feature = "mtls")))]
pub use tls::start_mtls_tcp;
