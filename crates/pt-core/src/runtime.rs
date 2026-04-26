use crate::RuntimeConfig;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeRole {
    Homebase,
    Exec,
}

pub fn apply_runtime_controls(
    cfg: &RuntimeConfig,
    role: RuntimeRole,
) -> Result<Vec<String>, String> {
    let mut notes = Vec::new();

    #[cfg(target_os = "linux")]
    {
        if cfg.affinity.enabled {
            let cores = select_affinity_cores(cfg, role);
            if cores.is_empty() {
                return Err("runtime.affinity enabled but selected core list is empty".to_string());
            }
            set_current_thread_affinity(&cores)?;
            notes.push(format!(
                "linux affinity applied for role={:?} cores={:?}",
                role, cores
            ));
        }
        if cfg.jitter_controls.enabled {
            notes.push(format!(
                "linux jitter controls configured: nic_polling={} disable_cpu_powersave={} isolate_cores={} rcu_nocbs={} nohz_full={}",
                cfg.jitter_controls.nic_polling,
                cfg.jitter_controls.disable_cpu_powersave,
                cfg.jitter_controls.isolate_cores,
                cfg.jitter_controls.rcu_nocbs,
                cfg.jitter_controls.nohz_full,
            ));
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        if cfg.affinity.enabled && matches!(role, RuntimeRole::Exec) {
            return Err(
                "runtime.affinity is enabled for exec mode, but affinity enforcement is currently linux-only".to_string(),
            );
        }
        if cfg.affinity.enabled {
            notes.push("runtime.affinity configured but ignored on non-linux host".to_string());
        }
        if cfg.jitter_controls.enabled {
            notes.push(
                "runtime.jitter_controls configured; apply equivalent host tuning via scripts"
                    .to_string(),
            );
        }
    }

    Ok(notes)
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn select_affinity_cores(cfg: &RuntimeConfig, role: RuntimeRole) -> Vec<usize> {
    let mut cores = Vec::new();
    match role {
        RuntimeRole::Homebase => {
            cores.extend_from_slice(&cfg.affinity.feed_cores);
            cores.extend_from_slice(&cfg.affinity.persistence_cores);
        }
        RuntimeRole::Exec => {
            cores.extend_from_slice(&cfg.affinity.feed_cores);
            cores.extend_from_slice(&cfg.affinity.strategy_cores);
            cores.extend_from_slice(&cfg.affinity.execution_cores);
            cores.extend_from_slice(&cfg.affinity.persistence_cores);
        }
    }
    cores.sort_unstable();
    cores.dedup();
    cores
}

#[cfg(target_os = "linux")]
fn set_current_thread_affinity(cores: &[usize]) -> Result<(), String> {
    unsafe {
        let mut set: libc::cpu_set_t = std::mem::zeroed();
        libc::CPU_ZERO(&mut set);
        for &core in cores {
            libc::CPU_SET(core, &mut set);
        }
        let rc = libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &set);
        if rc != 0 {
            return Err(format!(
                "sched_setaffinity failed: {}",
                std::io::Error::last_os_error()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{apply_runtime_controls, ensure_rustls_crypto_provider, RuntimeRole};
    use crate::{RuntimeAffinityConfig, RuntimeConfig, RuntimeJitterControlsConfig};

    #[test]
    fn rustls_provider_install_is_idempotent() {
        let first = ensure_rustls_crypto_provider();
        let second = ensure_rustls_crypto_provider();
        assert!(first.is_ok());
        assert_eq!(first, second);
    }

    #[test]
    fn runtime_controls_non_linux_exec_rejects_affinity() {
        let cfg = RuntimeConfig {
            affinity: RuntimeAffinityConfig {
                enabled: true,
                feed_cores: vec![0],
                strategy_cores: vec![1],
                execution_cores: vec![2],
                persistence_cores: vec![3],
            },
            jitter_controls: RuntimeJitterControlsConfig {
                enabled: true,
                nic_polling: true,
                disable_cpu_powersave: true,
                isolate_cores: true,
                rcu_nocbs: true,
                nohz_full: true,
            },
        };

        #[cfg(not(target_os = "linux"))]
        {
            assert!(apply_runtime_controls(&cfg, RuntimeRole::Exec).is_err());
        }
        #[cfg(target_os = "linux")]
        {
            assert!(apply_runtime_controls(&cfg, RuntimeRole::Exec).is_ok());
        }
    }
}
