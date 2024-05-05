use std::error::Error;
use std::future::Future;
use std::pin::Pin;

pub type InstallerFuture = Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send>>;

pub(crate) trait Installer: Send {
    fn install(&self) -> InstallerFuture;
    fn install_dependencies(&self) -> InstallerFuture;
}

pub(crate) trait Runner: Send {
    fn start(&self) -> InstallerFuture;
}