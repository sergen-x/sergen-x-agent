use std::sync::Arc;
use async_trait::async_trait;
use crate::common::error::SergenError;
use crate::common::sysinfo::systeminfo;
use crate::common::sysinfo::systeminfo::SystemInfo;
use crate::dependencies::java::adoptium;

#[async_trait]
pub trait JavaDistribution: Sync + Send {
    async fn get_versions(
        &self
    ) -> Result<(), SergenError>;
    async fn download_version(
        &self,
        version: &str,
        system: SystemInfo,
        variant: &str
    ) -> Result<(), SergenError>;
}

pub async fn download_and_install(
    version: String,
    system: SystemInfo,
    variant: String,
    distribution: Arc<dyn JavaDistribution>,

) -> Result<(), SergenError> {
    // let versions = distribution.get_versions().await?;
    distribution.download_version(version.as_str(), system, variant.as_str()).await?;
    Ok(())
}

pub async fn install(
    version: String,
    distribution: String,
    variant: String,
) -> Result<(), SergenError>  {
    let distribution_object: Arc<dyn JavaDistribution + Send> = match distribution.as_str() {
        "adoptium" => Arc::new(adoptium::main::Adoptium),
        _ => return Err(SergenError::InstallationError("Unsupported distribution variant".into())),
    };
    let system = systeminfo::get_system_info();
    match download_and_install(version, system, variant, distribution_object).await {
        Ok(_) => println!("Download succeeded"),
        Err(e) => eprintln!("Download Operation failed: {}", e),
    }
    Ok(())
}