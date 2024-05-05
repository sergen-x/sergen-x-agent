use crate::common::installer::InstallerFuture;
use crate::common::sysinfo::systeminfo;
use crate::common::sysinfo::systeminfo::SystemInfo;
use crate::dependencies::java::adoptium;

pub trait JavaDistribution: Send {
    fn get_versions(&self) -> InstallerFuture;
    fn download_version(&self, version: &str, system: SystemInfo, variant: &str) -> InstallerFuture;
}

pub fn download_and_install(
    version: String,
    system: SystemInfo,
    variant: String,
    distribution: Box<dyn JavaDistribution>,

) -> InstallerFuture {
    Box::pin(async move {
        let versions = distribution.get_versions().await?;
        distribution.download_version(version.as_str(), system, variant.as_str()).await?;
        Ok(())
    })
}

pub fn install(
    version: String,
    distribution: String,
    variant: String,
) -> InstallerFuture {
    Box::pin(async move {

        let distribution_object: Box<dyn JavaDistribution + Send> = match distribution.as_str() {
            "adoptium" => Box::new(adoptium::main::Adoptium),
            _ => return Err("Unsupported distribution variant".into()),
        };
        let system = systeminfo::get_system_info();
        download_and_install(version, system, variant, distribution_object).await;
        Ok(())

    })
}