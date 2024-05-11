use std::{fs, io};
use std::io::Write;
use crate::minecraft;
use crate::common::installer::{Installer, InstallerFuture, Runner, SimpleInstaller};

pub(crate) struct Minecraft;

impl Installer for Minecraft {
    fn install(
        &self,
        version: Option<String>,
        variant: Option<String>
    ) -> InstallerFuture {
        Box::pin(async move {
            let variant_str = variant.unwrap().clone();
            let version_str = version.clone();

            let installer: Box<dyn SimpleInstaller> = match variant_str.as_str() {
                "vanilla" => Box::new(minecraft::vanilla::vanilla::VanillaMinecraft),
                _ => return Err("Unsupported distribution variant".into()),
            };
            installer.install(version_str).await.expect("TODO: panic message");
            Ok(())
        })
    }

    fn install_dependencies(&self) -> InstallerFuture {
        Box::pin(async move {
            let _ = crate::dependencies::java::main::install(
                "21".to_string(),
                "adoptium".to_string(),
                "jdk".to_string()
            ).await;
            Ok(())
        })
    }
}

impl Runner for Minecraft {
    fn start(&self) -> InstallerFuture {
        Box::pin(async move {
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
        })
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
    pub fn startup_command(memory: i32, jar_name: &str) -> String {
        let common_flags= format!(r#"
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
        "#, memory, memory, jar_name);

        let flags = if memory < 12 {
            format!(r#"{}
                -XX:G1NewSizePercent=30
                -XX:G1MaxNewSizePercent=40
                -XX:G1HeapRegionSize=8M
            "#, common_flags)
        } else {
            format!(r#"{}
                -XX:G1NewSizePercent=40
                -XX:G1MaxNewSizePercent=50
                -XX:G1HeapRegionSize=16M
            "#, common_flags)
        };
        flags
    }
}