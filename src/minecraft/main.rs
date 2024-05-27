use crate::common::error::SergenError;
use crate::common::installer::{Installer, Runner, SimpleInstaller};
use crate::minecraft;
use async_trait::async_trait;
use std::io::Write;
use std::sync::Arc;
use std::{fs, io};

pub(crate) struct Minecraft;

#[async_trait]
impl Installer for Minecraft {
    async fn install(
        &self,
        version: Option<String>,
        variant: Option<String>,
    ) -> Result<(), SergenError> {
        let variant_str = variant.unwrap().clone();
        let version_str = version.clone();

        let installer: Arc<dyn SimpleInstaller> = match variant_str.as_str() {
            "vanilla" => {
                Arc::new(minecraft::vanilla::vanilla::VanillaMinecraft)
            }
            _ => {
                return Err(SergenError::InstallationError(
                    "Unsupported distribution variant".into(),
                ))
            }
        };

        let result = installer.install(version_str).await;
        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(SergenError::InstallationError(
                "Failed to spawn blocking task".into(),
            )),
        }
    }

    async fn install_dependencies(&self) -> Result<(), SergenError> {
        let _ = crate::dependencies::java::main::install(
            "21".to_string(),
            "adoptium".to_string(),
            "jdk".to_string(),
        )
        .await;
        Ok(())
    }
}

#[async_trait]
impl Runner for Minecraft {
    async fn start(&self) -> Result<(), SergenError> {
        let eula_path = "EULA.txt";
        match Minecraft::is_eula_accepted(eula_path) {
            Ok(true) => println!("Minecraft's EULA is accepted."),
            Ok(false) => match Minecraft::accept_eula(eula_path) {
                Ok(_) => println!("Minecraft's EULA is now accepted."),
                Err(e) => eprintln!("Error accepting EULA: {}", e),
            },
            Err(e) => eprintln!("Error loading EULA: {}", e),
        }
        Ok(())
    }
}

impl Minecraft {
    pub fn is_eula_accepted(file_path: &str) -> Result<bool, io::Error> {
        let content = fs::read_to_string(file_path)?;
        let accepted = content.contains("eula=true");
        Ok(accepted)
    }

    pub fn accept_eula(file_path: &str) -> Result<(), io::Error> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;
        file.write_all(b"eula=true")?;
        Ok(())
    }

    // Use aikar's flags - https://aikar.co/2018/07/02/tuning-the-jvm-g1gc-garbage-collector-flags-for-minecraft/
    pub fn startup_command(
        memory: i32,
        jar_name: &str,
    ) -> String {
        let common_flags = format!(
            r#"
            java
            -Xms{}G
            -Xmx{}G
            -XX:+UseG1GC
            -XX:+ParallelRefProcEnabled
            -XX:MaxGCPauseMillis=200
            -XX:+UnlockExperimentalVMOptions
            -XX:+DisableExplicitGC
            -XX:+AlwaysPreTouch
            -XX:G1ReservePercent=20
            -XX:G1HeapWastePercent=5
            -XX:G1MixedGCCountTarget=4
            -XX:InitiatingHeapOccupancyPercent=15
            -XX:G1MixedGCLiveThresholdPercent=90
            -XX:G1RSetUpdatingPauseTimePercent=5
            -XX:SurvivorRatio=32
            -XX:+PerfDisableSharedMem
            -XX:MaxTenuringThreshold=1
            -Dusing.aikars.flags=https://mcflags.emc.gs
            -Daikars.new.flags=true -jar {} --nogui
        "#,
            memory, memory, jar_name
        );

        let flags = if memory < 12 {
            format!(
                r#"{}
                -XX:G1NewSizePercent=30
                -XX:G1MaxNewSizePercent=40
                -XX:G1HeapRegionSize=8M
            "#,
                common_flags
            )
        } else {
            format!(
                r#"{}
                -XX:G1NewSizePercent=40
                -XX:G1MaxNewSizePercent=50
                -XX:G1HeapRegionSize=16M
            "#,
                common_flags
            )
        };
        flags
    }
}
