use std::future::Future;
use std::pin::Pin;

pub(crate) trait Installer {
    fn install(&self) ->  Pin<Box<dyn Future<Output=()>>>;
    fn install_dependencies(&self) ->  Pin<Box<dyn Future<Output=()>>>;
}