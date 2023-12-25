use tcloud_library::async_trait;
use tcloud_library::error::PluginError;
use tcloud_library::Plugin;
use std::path::Path;

#[derive(Debug)]
pub struct ArchivePlugin;

#[async_trait]
impl Plugin for ArchivePlugin {
    fn new() -> Result<(String, Self), PluginError>
    where
        Self: Sized,
    {
        Ok(("archive".to_string(), ArchivePlugin {}))
    }
    async fn process_api_request(&mut self, req: String, path: &Path) -> Result<String, PluginError> {
        Ok("test test".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn init_test() {
        println!("{:?}", ArchivePlugin::new());
    }
}
