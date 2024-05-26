use async_trait::async_trait;
use crate::common::error::SergenError;

#[async_trait]
pub(crate) trait AsyncCommand: Sync + Send {
    async fn execute(
        &self,
        args: &clap::ArgMatches
    ) -> Result<(), SergenError>;
}