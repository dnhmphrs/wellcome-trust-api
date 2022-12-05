use rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore};

// config goes beyond my knowledge of rust
// config unchanged from example in: https://github.com/actix/examples/tree/master/https-tls/awc-https

/// Create simple rustls client config from root certificates.
pub fn client_config() -> ClientConfig {
  let mut root_store = RootCertStore::empty();
  root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
      OwnedTrustAnchor::from_subject_spki_name_constraints(
          ta.subject,
          ta.spki,
          ta.name_constraints,
      )
  }));

  rustls::ClientConfig::builder()
      .with_safe_defaults()
      .with_root_certificates(root_store)
      .with_no_client_auth()
}