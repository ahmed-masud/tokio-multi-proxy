# tokio-proxy


[![Crates.io](https://img.shields.io/crates/v/tokio-proxy.svg)](https://crates.io/crates/tokio-proxy)
[![Docs.rs](https://docs.rs/tokio-proxy/badge.svg)](https://docs.rs/tokio-proxy)
[![CI](https://github.com/ahmed-masud/tokio-proxy/actions/workflows/ci.yml/badge.svg)](https://github.com/ahmed-masud/tokio-proxy/actions/workflows/ci.yml)

**An async, modular proxy with support for TCP, UDP, Unix sockets, TLS, and mutual TLS. Built on [Tokio](https://tokio.rs) and [Rustls](https://github.com/rustls/rustls).**

---

## ✨ Features

- 🔌 Raw TCP passthrough
- 🌊 UDP datagram proxying
- 🧱 Unix Domain Socket bridging
- 🔐 TLS termination
- 🔐 Mutual TLS (mTLS) authentication
- ⚙️ Compile-time feature flags to minimize dependencies

---

## 📦 Install

Add to your `Cargo.toml`:

```toml
tokio-proxy = { version = "0.1.0", features = ["tls", "mtls"] }
```

Or if using only TCP passthrough (no TLS):

```toml
tokio-proxy = "0.1.0"
```

---

## 🚀 Examples

Run examples with:

```bash
cargo run --example tcp           # plain TCP
cargo run --example udp           # UDP
cargo run --example unix          # Unix socket
cargo run --example tls --features tls         # TLS
cargo run --example mtls --features mtls       # mTLS
```

---

## 🔧 Usage

### TCP

```rust
tokio_proxy::start_tcp("0.0.0.0:8080", "127.0.0.1:9000").await?;
```

### TLS

```rust
tokio_proxy::start_tls_tcp("0.0.0.0:443", "127.0.0.1:8080", "cert.pem", "key.pem").await?;
```

### mTLS

```rust
tokio_proxy::start_mtls_tcp("0.0.0.0:443", "127.0.0.1:8080", "cert.pem", "key.pem", "ca.pem").await?;
```

---

## 🛠 Features

| Feature     | Description                         |
|-------------|-------------------------------------|
| `passthrough` | Raw TCP proxying (default)         |
| `tls`         | TLS termination support            |
| `mtls`        | Mutual TLS (requires `tls`)        |

Enable with:

```bash
cargo build --features tls
```

---

## 🔐 Dev Certificate (for local testing)

```bash
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout examples/key.pem -out examples/cert.pem \
  -subj "/CN=localhost"

# Optional mTLS CA
cp examples/cert.pem examples/ca.pem
```

---

## 📚 Documentation

- [API Docs (docs.rs)](https://docs.rs/tokio-proxy)
- Build locally:
  ```bash
  cargo doc --all-features --open
  ```

---

## 🪪 License

Licensed under either of:

- MIT
- Apache-2.0

---

## 👤 Author

**Ahmed Masud** — [saf.ai](https://saf.ai)  
MIT/Apache dual-license. Contributions welcome!


