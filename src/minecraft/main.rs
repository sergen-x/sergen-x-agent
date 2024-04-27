use std::future::Future;
use std::pin::Pin;
use crate::{dependencies, minecraft};
use crate::common::installer::Installer;

pub(crate) struct Minecraft;

impl Installer for Minecraft {
    fn install(&self) ->  Pin<Box<dyn Future<Output=()>>> {
        Box::pin(async {
            let manifest = minecraft::vanilla::vanilla::get_all_versions()
                .await
                .expect("Failed to fetch version manifest");
            let release_version = manifest.latest.release.clone();
            let version = manifest.get_download_url(&release_version)
                .await
                .expect("Failed to fetch download URL");

            if let Some(ref version_info) = version {
                let _ = version.expect("Missing version").download()
                    .await
                    .expect("Failed to download version");
            } else {
                panic!("VersionInfo is None, cannot download version");
            }
        })
    }

    fn install_dependencies(&self) -> Pin<Box<dyn Future<Output=()>>> {
        Box::pin(async {
            let os = std::env::consts::OS;
            let arch = dependencies::java::adoptium::main::get_architecture().expect("Missing system architecture");
            let _ = dependencies::java::adoptium::main::download_version("21", os, arch.as_str(), "jdk").await;
        })
    }
}