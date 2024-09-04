use std::process::Command;

pub fn install_clamav_macos() -> std::io::Result<()> {
    println!("Installing ClamAV on macOS...");

    // Install Homebrew if it's not already installed
    if !Command::new("brew").output().is_ok() {
        println!("Homebrew not found. Installing Homebrew...");
        Command::new("bash")
            .arg("-c")
            .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
            .status()?;
    }

    // Install ClamAV using Homebrew
    Command::new("brew")
        .arg("install")
        .arg("clamav")
        .status()?;

    Ok(())
}
