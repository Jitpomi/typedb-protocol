use tonic::transport::{Certificate, ClientTlsConfig, Endpoint, Identity, Server, ServerTlsConfig};

#[tokio::test]
async fn tls_requires_a_trusted_certificate_and_matching_hostname() -> Result<(), Box<dyn std::error::Error>> {
    let rcgen::CertifiedKey { cert, signing_key } = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let pem = cert.pem();
    let identity = Identity::from_pem(&pem, signing_key.serialize_pem());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let address = listener.local_addr()?;
    let server = Server::builder().tls_config(ServerTlsConfig::new().identity(identity))?
        .add_routes(tonic::service::Routes::default());
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let task = tokio::spawn(server.serve_with_incoming_shutdown(
        tokio_stream::wrappers::TcpListenerStream::new(listener), async { let _ = stopped.await; }));
    let endpoint = Endpoint::from_shared(format!("https://{address}"))?;
    let trusted = ClientTlsConfig::new().ca_certificate(Certificate::from_pem(&pem)).domain_name("localhost");
    endpoint.clone().tls_config(trusted)?.connect().await?;
    let wrong_host = ClientTlsConfig::new().ca_certificate(Certificate::from_pem(&pem)).domain_name("wrong.example");
    assert!(endpoint.clone().tls_config(wrong_host)?.connect().await.is_err());
    let untrusted = ClientTlsConfig::new().domain_name("localhost").with_native_roots();
    assert!(endpoint.tls_config(untrusted)?.connect().await.is_err());
    let _ = stop.send(());
    task.await??;
    Ok(())
}
