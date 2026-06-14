use crate::scanner::{CleanableItem, Scanner};
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

use crate::sudo::run_with_sudo;

pub struct JournalScanner;

#[async_trait]
impl Scanner for JournalScanner {
    fn name(&self) -> &'static str {
        "Systemd Journal"
    }

    async fn scan(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        // Just checking how much disk usage journal has
        if let Ok(output) = Command::new("journalctl").args(["--disk-usage"]).output().await {
            let out_str = String::from_utf8_lossy(&output.stdout);
            items.push(CleanableItem {
                name: "Systemd Journal Logs".into(),
                description: format!("Old logs over 2 weeks (Current: {})", out_str.trim()),
                size_bytes: 1,
                path: Some("/var/log/journal".into()),
            });
        }
        Ok(items)
    }

    async fn clean(&self, items: &[CleanableItem], sudo_password: Option<&str>) -> Result<u64> {
        let pwd = sudo_password.unwrap_or("");
        if items.iter().any(|i| i.name == "Systemd Journal Logs") {
            let _ = run_with_sudo("journalctl", &["--vacuum-time=2weeks"], pwd).await;
            return Ok(1);
        }
        Ok(0)
    }

    fn requires_sudo(&self) -> bool {
        true
    }
}
