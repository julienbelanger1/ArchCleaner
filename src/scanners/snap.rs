use crate::scanner::{CleanableItem, Scanner};
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

use crate::sudo::run_with_sudo;

pub struct SnapScanner;

#[async_trait]
impl Scanner for SnapScanner {
    fn name(&self) -> &'static str {
        "Snap Unused Runtimes"
    }

    async fn scan(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        // Check if snap is installed
        if let Ok(output) = Command::new("snap").args(["list", "--all"]).output().await {
            let out_str = String::from_utf8_lossy(&output.stdout);
            for line in out_str.lines() {
                if line.contains("disabled") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(name) = parts.first() {
                        items.push(CleanableItem {
                            name: format!("Unused Snap: {}", name),
                            description: "Disabled snap package keeping older version".into(),
                            size_bytes: 1,
                            path: None,
                        });
                    }
                }
            }
        }
        Ok(items)
    }

    async fn clean(&self, items: &[CleanableItem], sudo_password: Option<&str>) -> Result<u64> {
        let pwd = sudo_password.unwrap_or("");
        let mut freed = 0;
        for item in items {
            if item.name.starts_with("Unused Snap: ") {
                let pkg = item.name.replace("Unused Snap: ", "");
                let _ = run_with_sudo("snap", &["remove", &pkg], pwd).await;
                freed += 1;
            }
        }
        Ok(freed)
    }

    fn requires_sudo(&self) -> bool {
        true // Snap removal needs sudo
    }
}
