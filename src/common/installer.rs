use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub type InstallerFuture = Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send>>;

pub trait Installer: Send {
    fn install(
        &self,
        version: Option<String>,
        variant: Option<String>
    ) -> InstallerFuture;
    fn install_dependencies(
        &self
    ) -> InstallerFuture {
        unimplemented!("install_dependencies is not implemented");
    }
}

pub trait SimpleInstaller: Send {
    fn install(&self, version: Option<String>) -> InstallerFuture;
}

pub trait Runner: Send {
    fn start(&self) -> InstallerFuture;
}