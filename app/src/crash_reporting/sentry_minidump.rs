//! Minidump crash reporting stub. Sentry integration has been removed.

use std::path::Path;

/// No-op server stub. Returns an error since crash reporting is not supported.
pub fn run_server(_socket_path: &Path) -> anyhow::Result<()> {
    anyhow::bail!("Minidump crash reporting server is not supported in this build");
}
