use crate::common::error::SergenError;
use async_trait::async_trait;

#[async_trait]
pub trait Installer: Sync + Send {
    async fn install(
        &self,
        version: Option<String>,
        variant: Option<String>,
    ) -> Result<(), SergenError>;
    async fn install_dependencies(&self) -> Result<(), SergenError> {
        unimplemented!("install_dependencies is not implemented");
    }
}

#[async_trait]
pub trait SimpleInstaller: Sync + Send {
    async fn install(
        &self,
        version: Option<String>,
    ) -> Result<(), SergenError>;
}

#[async_trait]
pub trait Runner: Sync {
    async fn start(&self) -> Result<(), SergenError>;
}
