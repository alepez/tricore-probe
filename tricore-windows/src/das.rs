//! Module to interface with a DAS instance

use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use anyhow::{bail, Context};

/// Spawns a DAS instance.
pub fn run_console() -> anyhow::Result<()> {
    let das_home = PathBuf::from(
        std::env::var("DAS_HOME").context("DAS_HOME not defined, is DAS installed?")?,
    );

    #[cfg(not(feature = "dasv8"))]
    {
        log::trace!("Starting dashpas");
        let mut process = Command::new(das_home.join("dashpas/das_dashpas.exe"));
        let _started_dashpas = process.spawn().context("Failed to spawn das_dashpas")?;
    }

    log::trace!("Starting UDAS_Console");
    let udas_console_path = das_home.join("servers/udas/UDAS_Console.exe");

    // Check if the file exists
    if !udas_console_path.exists() {
        bail!(
            "UDAS_Console.exe not found at {:?}, is DAS installed?",
            udas_console_path
        )
    }

    log::info!("Starting {:?}", udas_console_path);

    let mut udas_console = Command::new(udas_console_path);

    let udas_console = udas_console.stderr(Stdio::inherit()).stdout(Stdio::null());
    let mut udas_console = udas_console
        .spawn()
        .context("Failed to spawn UDAS_Console")?;

    log::info!("DAS server started");
    let result = udas_console
        .wait()
        .context("UDAS_Console.exe process aborted")?;

    if !result.success() {
        bail!("Das server exited unsuccessfully: {result:?}")
    }

    Ok(())
}
