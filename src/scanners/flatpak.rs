use crate::scanner::{CleanableItem, Scanner};
use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;

pub struct FlatpakScanner;

#[async_trait]
impl Scanner for FlatpakScanner {
    fn name(&self) -> &'static str {
        "Flatpak Unused Runtimes"
    }

    async fn scan(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        // Check if flatpak is installed
        if let Ok(output) = Command::new("flatpak").args(["uninstall", "--unused", "--dry-run"]).output().await {
            let out_str = String::from_utf8_lossy(&output.stdout);
            if out_str.contains("Nothing to do") {
                return Ok(items);
            }
            items.push(CleanableItem {
                name: "Unused Flatpak Runtimes".into(),
                description: "Old flatpak runtimes and extensions".into(),
                size_bytes: 1,
                path: None,
            });
        }
        Ok(items)
    }

    async fn clean(&self, items: &[CleanableItem], _sudo_password: Option<&str>) -> Result<u64> {
        if !items.is_empty() {
            let _ = Command::new("flatpak").args(["uninstall", "--unused", "-y"]).output().await;
            return Ok(1);
        }
        Ok(0)
    }

    fn requires_sudo(&self) -> bool {
        false // Usually flatpak can be run by user for user installation, but system needs sudo. Let's assume system needs sudo for safety, or flatpak handles its own polkit prompt. We'll set false here.
    }
}
