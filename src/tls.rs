
#![cfg(feature = "tls")]

use std::{fs::File, io::BufReader, sync::Arc};

use tokio::{
    io::copy_bidirectional,
    net::{TcpListener, TcpStream},
};
use tokio_rustls::TlsAcceptor;

use tokio_rustls::rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::ServerConfig,
};

#[cfg(feature = "mtls")]
use tokio_rustls::rustls::{
    RootCertStore,
    server::WebPkiClientVerifier,
};

use rustls_pemfile::{certs, private_key};

use anyhow::{Context, Result};

/// TLS proxy

/// Start a TLS-terminating proxy server.
///
/// Incoming TLS connections are decrypted and proxied to `target_addr` over plaintext.
///
/// Requires the `tls` feature.
///
/// ## Example
/// ```no_run
/// tokio_proxy::start_tls_tcp("0.0.0.0:443", "127.0.0.1:8080", "cert.pem", "key.pem").await?;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "tls")))]
pub async fn start_tls_tcp(bind_addr: &str, target_addr: &str, cert_path: &str, key_path: &str) -> Result<()> {
    let config = Arc::new(load_tls_config(cert_path, key_path)?);
    let acceptor = TlsAcceptor::from(config);
    let listener = TcpListener::bind(bind_addr).await?;
    println!("TLS proxy listening on {}", bind_addr);

    loop {
        let (inbound, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let target = target_addr.to_string();

        tokio::spawn(async move {
            match acceptor.accept(inbound).await {
                Ok(mut tls_stream) => {
                    if let Ok(mut outbound) = TcpStream::connect(&target).await {
                        let _ = copy_bidirectional(&mut tls_stream, &mut outbound).await;
                    }
                }
                Err(e) => eprintln!("TLS error: {e:?}"),
            }
        });
    }
}

/// Start a mutual TLS (mTLS) proxy server.
///
/// Requires clients to present valid client certificates signed by your provided CA cert.
///
/// Requires the `mtls` feature.
///
/// ## Example
/// ```no_run
/// tokio_proxy::start_mtls_tcp("0.0.0.0:443", "127.0.0.1:8080", "cert.pem", "key.pem", "ca.pem").await?;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "mtls")))]

/// mTLS proxy
#[cfg(feature = "mtls")]
pub async fn start_mtls_tcp(
    bind_addr: &str,
    target_addr: &str,
    cert_path: &str,
    key_path: &str,
    ca_cert_path: &str,
) -> Result<()> {
    let config = Arc::new(load_mtls_config(cert_path, key_path, ca_cert_path)?);
    let acceptor = TlsAcceptor::from(config);
    let listener = TcpListener::bind(bind_addr).await?;
    println!("mTLS proxy listening on {}", bind_addr);

    loop {
        let (inbound, _) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let target = target_addr.to_string();

        tokio::spawn(async move {
            match acceptor.accept(inbound).await {
                Ok(mut tls_stream) => {
                    if let Ok(mut outbound) = TcpStream::connect(&target).await {
                        let _ = copy_bidirectional(&mut tls_stream, &mut outbound).await;
                    }
                }
                Err(e) => eprintln!("mTLS error: {e:?}"),
            }
        });
    }
}

fn load_tls_config(cert_path: &str, key_path: &str) -> Result<ServerConfig> {
    let certs = load_certs(cert_path)?;
    let key = load_private_key(key_path)?;
    ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("building TLS ServerConfig")
}

#[cfg(feature = "mtls")]
fn load_mtls_config(cert_path: &str, key_path: &str, ca_cert_path: &str) -> Result<ServerConfig> {
    let certs = load_certs(cert_path)?;
    let key = load_private_key(key_path)?;
    let ca_certs = load_certs(ca_cert_path)?;

    let mut root_store = RootCertStore::empty();
    for ca in ca_certs {
        root_store.add(ca)?;
    }

    let verifier = WebPkiClientVerifier::builder(Arc::new(root_store))
        .build()
        .context("building mTLS client verifier")?;

    ServerConfig::builder()
        .with_client_cert_verifier(verifier)
        .with_single_cert(certs, key)
        .context("building mTLS ServerConfig")
}

fn load_certs(path: &str) -> Result<Vec<CertificateDer<'static>>> {
    let mut reader = BufReader::new(File::open(path).context("open cert file")?);
    certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().map(|c| CertificateDer::from(c.into_owned())).collect())
        .context("read certs")
}

fn load_private_key(path: &str) -> Result<PrivateKeyDer<'static>> {
    let mut reader = BufReader::new(File::open(path).context("open key file")?);
    private_key(&mut reader)
        .context("read private key")?
        .map(PrivateKeyDer::from)
        .ok_or_else(|| anyhow::anyhow!("no private key found in file"))
}

