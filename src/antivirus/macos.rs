use std::process::{Command, Stdio};

pub fn install_clamav_macos() -> std::io::Result<()> {
    println!("Installing ClamAV on macOS...");

    let check_brew = Command::new("brew")
        .arg("--version")
        .stdout(Stdio::null())  
        .stderr(Stdio::null()) 
        .status();
    
    match check_brew {
        Ok(status) if status.success() => {
            println!("Homebrew is already installed.");
        }
        _ => {
            println!("Homebrew is not installed. Installing...");
            let install_command = Command::new("/bin/bash")
                .arg("-c")
                .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)")
                .status();
            
            match install_command {
                Ok(status) if status.success() => {
                    println!("Homebrew installed successfully.");
                }
                Ok(_) | Err(_) => {
                    println!("Failed to install Homebrew.");
                }
            }
        }
    }

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
