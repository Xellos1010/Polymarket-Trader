use std::sync::OnceLock;

static RUSTLS_PROVIDER_STATE: OnceLock<Result<String, String>> = OnceLock::new();

/// Installs the process-level rustls crypto provider exactly once.
///
/// The workspace currently pulls both `aws-lc-rs` and `ring` backends through
/// transitive dependencies. Without explicit installation, TLS clients may panic
/// at runtime when they first attempt to connect.
pub fn ensure_rustls_crypto_provider() -> Result<String, String> {
    RUSTLS_PROVIDER_STATE
        .get_or_init(|| {
            if rustls::crypto::CryptoProvider::get_default().is_some() {
                return Ok("already-installed".to_string());
            }

            if rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .is_ok()
            {
                return Ok("aws_lc_rs".to_string());
            }

            if rustls::crypto::ring::default_provider()
                .install_default()
                .is_ok()
            {
                return Ok("ring".to_string());
            }

            Err(
                "failed to install rustls CryptoProvider (aws_lc_rs and ring both failed)"
                    .to_string(),
            )
        })
        .clone()
}

#[cfg(test)]
mod tests {
    use super::ensure_rustls_crypto_provider;

    #[test]
    fn rustls_provider_install_is_idempotent() {
        let first = ensure_rustls_crypto_provider();
        let second = ensure_rustls_crypto_provider();
        assert!(first.is_ok());
        assert_eq!(first, second);
    }
}
