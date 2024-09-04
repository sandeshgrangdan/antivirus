use std::process::Command;
use std::io;

pub fn install_clamav_linux() -> io::Result<()> {
    // Detect the Linux distribution
    let output = Command::new("lsb_release")
        .arg("-i")
        .output()?;

    let binding = String::from_utf8_lossy(&output.stdout);
    let dist_id = binding.trim();

    match dist_id {
        "Distributor ID: Ubuntu" | "Distributor ID: Debian" => {
            // Install for Debian-based distributions
            println!("Detected Debian-based distribution.");
            Command::new("sudo")
                .arg("apt-get")
                .arg("update")
                .status()?;

            Command::new("sudo")
                .arg("apt-get")
                .arg("install")
                .arg("-y")
                .arg("clamav")
                .arg("clamav-daemon")
                .status()?;
        }
        "Distributor ID: CentOS" | "Distributor ID: RedHatEnterpriseServer" => {
            // Install for Red Hat-based distributions
            println!("Detected Red Hat-based distribution.");
            Command::new("sudo")
                .arg("yum")
                .arg("install")
                .arg("-y")
                .arg("clamav")
                .status()?;
        }
        "Distributor ID: Fedora" => {
            // Install for Fedora
            println!("Detected Fedora distribution.");
            Command::new("sudo")
                .arg("dnf")
                .arg("install")
                .arg("-y")
                .arg("clamav")
                .status()?;
        }
        "Distributor ID: Arch" => {
            // Install for Arch Linux
            println!("Detected Arch Linux distribution.");
            Command::new("sudo")
                .arg("pacman")
                .arg("-Syu")
                .arg("--noconfirm")
                .status()?;

            Command::new("sudo")
                .arg("pacman")
                .arg("-S")
                .arg("--noconfirm")
                .arg("clamav")
                .status()?;
        }
        _ => {
            eprintln!("Unsupported distribution: {}", dist_id);
            return Err(io::Error::new(io::ErrorKind::Other, "Unsupported distribution"));
        }
    }

    Ok(())
}
