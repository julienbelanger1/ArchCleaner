use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct CleanableItem {
    pub name: String,
    pub description: String,
    pub size_bytes: u64,
    pub path: Option<String>,
}

#[async_trait]
pub trait Scanner: Send + Sync {
    /// The name of this category
    fn name(&self) -> &'static str;
    
    /// Does a dry run, returns what would be cleaned
    async fn scan(&self) -> Result<Vec<CleanableItem>>;
    
    /// Actually performs the cleanup
    /// If sudo_password is provided, commands will be executed with sudo
    async fn clean(&self, items: &[CleanableItem], sudo_password: Option<&str>) -> Result<u64>;
    
    /// Does this scanner require root privileges to clean?
    fn requires_sudo(&self) -> bool {
        false
    }
}
