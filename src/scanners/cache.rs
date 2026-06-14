use crate::scanner::{CleanableItem, Scanner};
use anyhow::Result;
use async_trait::async_trait;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;

pub struct CacheScanner;

#[async_trait]
impl Scanner for CacheScanner {
    fn name(&self) -> &'static str {
        "User Cache (~/.cache)"
    }

    async fn scan(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        let cache_dir = dirs::cache_dir().unwrap_or_default();
        if !cache_dir.exists() {
            return Ok(items);
        }

        let targets = vec![
            "mozilla/firefox",
            "google-chrome",
            "chromium",
            "yay",
            "thumbnails",
            "electron",
        ];

        for target in targets {
            let target_path = cache_dir.join(target);
            if target_path.exists() {
                let size = get_dir_size(&target_path).await.unwrap_or(0);
                if size > 0 {
                    items.push(CleanableItem {
                        name: format!("Cache: {}", target),
                        description: "User application cache".into(),
                        size_bytes: size,
                        path: Some(target_path.to_string_lossy().into_owned()),
                    });
                }
            }
        }

        Ok(items)
    }

    async fn clean(&self, items: &[CleanableItem], _sudo_password: Option<&str>) -> Result<u64> {
        let mut freed = 0;
        for item in items {
            if let Some(path) = &item.path {
                freed += item.size_bytes;
                let _ = tokio::fs::remove_dir_all(path).await;
            }
        }
        Ok(freed)
    }

    fn requires_sudo(&self) -> bool {
        false
    }
}

// Helper to get directory size
async fn get_dir_size(path: &PathBuf) -> Result<u64> {
    let mut total = 0;
    let mut stack = vec![path.clone()];

    while let Some(current) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(current) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        stack.push(entry.path());
                    } else {
                        total += metadata.size();
                    }
                }
            }
        }
    }
    Ok(total)
}
