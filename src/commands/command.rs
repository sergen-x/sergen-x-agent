use std::future::Future;
use std::pin::Pin;

pub(crate) type SergenCommand = Pin<Box<dyn Future<Output = ()> + Send>>;

pub(crate) trait AsyncCommand: Send {
    fn execute(&self, args: &clap::ArgMatches) -> SergenCommand;
}