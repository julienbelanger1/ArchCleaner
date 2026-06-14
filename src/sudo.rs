use anyhow::{bail, Result};
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Run a command using sudo -S, feeding it the provided password.
pub async fn run_with_sudo(cmd: &str, args: &[&str], password: &str) -> Result<String> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(format!("{}\n", password).as_bytes()).await?;
    }

    let output = child.wait_with_output().await?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        bail!(
            "Sudo command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}
