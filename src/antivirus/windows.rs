use std::process::{Command, Stdio};
use std::io::Result;

pub fn install_clamav_windows() -> Result<()> {
    println!("Installing ClamAV on Windows...");

    // Check if Chocolatey is installed
    let check_choco = Command::new("powershell")
        .arg("-Command")
        .arg("Get-Command choco -ErrorAction SilentlyContinue")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !check_choco.success() {
        println!("Chocolatey is not installed. Installing Chocolatey...");

        let install_choco = Command::new("powershell")
            .arg("-Command")
            .arg("Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))")
            .status()?;

        if !install_choco.success() {
            println!("Failed to install Chocolatey.");
            return Ok(());
        }
    } else {
        println!("Chocolatey is already installed.");
    }

    // Install ClamAV using Chocolatey
    println!("Installing ClamAV...");
    let install_clamav = Command::new("powershell")
        .arg("-Command")
        .arg("choco install clamav -y")
        .status()?;

    if install_clamav.success() {
        println!("ClamAV installed successfully.");
    } else {
        println!("Failed to install ClamAV.");
        return Ok(());
    }

    Ok(())
}

