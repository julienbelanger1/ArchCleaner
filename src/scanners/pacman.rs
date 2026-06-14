use crate::scanner::{CleanableItem, Scanner};
use anyhow::Result;
use async_trait::async_trait;
use std::process::Stdio;
use tokio::process::Command;

use crate::sudo::run_with_sudo;

pub struct PacmanScanner;

#[async_trait]
impl Scanner for PacmanScanner {
    fn name(&self) -> &'static str {
        "Pacman & Paru Cache + Orphans"
    }

    async fn scan(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        
        // 1. Pacman Cache (paccache)
        if let Ok(output) = Command::new("paccache").args(["-dk1"]).output().await {
            let out_str = String::from_utf8_lossy(&output.stdout);
            if out_str.contains("freed") {
                items.push(CleanableItem {
                    name: "Pacman Cache".into(),
                    description: "Old pacman cache files (keeping 1 latest version)".into(),
                    size_bytes: 1, // non-zero to show it did something
                    path: Some("/var/cache/pacman/pkg".into()),
                });
            }
        }

        // 2. Paru Cache
        let user_cache = dirs::cache_dir().unwrap_or_default().join("paru/clone");
        if user_cache.exists() {
            items.push(CleanableItem {
                name: "Paru Build Cache".into(),
                description: "Old AUR build files cached by paru".into(),
                size_bytes: 1, 
                path: Some(user_cache.to_string_lossy().into_owned()),
            });
        }

        // 3. Orphans
        if let Ok(output) = Command::new("pacman").args(["-Qtdq"]).output().await {
            let out_str = String::from_utf8_lossy(&output.stdout);
            let orphans: Vec<&str> = out_str.trim().split('\n').filter(|s| !s.is_empty()).collect();
            for orphan in orphans {
                items.push(CleanableItem {
                    name: format!("Orphan: {}", orphan),
                    description: "Unused orphaned package".into(),
                    size_bytes: 1,
                    path: None,
                });
            }
        }

        Ok(items)
    }

    async fn clean(&self, items: &[CleanableItem], sudo_password: Option<&str>) -> Result<u64> {
        let pwd = sudo_password.unwrap_or("");
        let mut freed = 0;
        let mut do_paccache = false;
        let mut orphans = Vec::new();
        let mut paru_cache = None;

        for item in items {
            if item.name == "Pacman Cache" {
                do_paccache = true;
                freed += item.size_bytes;
            } else if item.name == "Paru Build Cache" {
                paru_cache = item.path.clone();
                freed += item.size_bytes;
            } else if item.name.starts_with("Orphan: ") {
                orphans.push(item.name.replace("Orphan: ", ""));
                freed += item.size_bytes;
            }
        }

        if do_paccache {
            let _ = run_with_sudo("paccache", &["-rk1"], pwd).await;
        }

        if !orphans.is_empty() {
            let mut args = vec!["-Rns", "--noconfirm"];
            args.extend(orphans.iter().map(|s| s.as_str()));
            let _ = run_with_sudo("pacman", &args, pwd).await;
        }

        if let Some(path) = paru_cache {
            let _ = tokio::fs::remove_dir_all(&path).await;
        }

        Ok(freed)
    }

    fn requires_sudo(&self) -> bool {
        true
    }
}
