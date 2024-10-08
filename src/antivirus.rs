use std::{env, thread};
use std::process::{Command, Stdio};
use std::io::{self, BufRead, Write};
use rand::Rng;
use regex::Regex;
use clap::Parser;
use std::fs::{self, File};
use std::path::Path;

mod linux;
mod macos;
mod windows;

/// Command-line arguments for the ClamAV scanning utility.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The path to the directory that you want to scan. Defaults to "HOME".
    #[arg(short, long, default_value_t = String::from("HOME"))]
    pub dir: String,

    /// Option to update the ClamAV virus database before scanning. Defaults to "Yes".
    #[arg(short, long, default_value_t = String::from("Yes"))]
    pub update: String,
}

pub struct Antivirus {
    home_dir: String,
    google_chat_url: String,
    summary: String,
    infected_files: String,
    args: Args,
    tmp_file : String
}

fn is_clamav_installed() -> io::Result<bool> {
    let output = Command::new("which")
        .arg("clamscan")
        .output()?;

    Ok(!output.stdout.is_empty())
}

fn generate_random_file_name() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = (0..10)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    format!("/tmp/{}.txt", random_string)
}

pub fn handle_freshclam_copy_windows(config_dir: &str) -> io::Result<()> {
    // Ensure the configuration directory exists
    if !Path::new(config_dir).exists() {
        fs::create_dir_all(config_dir)?;
    }

    // Path to the default freshclam.conf.example file
    let freshclam_conf_source = r"C:\ProgramData\chocolatey\lib\clamav\tools\clamav-1.4.1.win.x64\conf_examples\freshclam.conf.sample";
    let freshclam_conf_destination = r"C:\ProgramData\chocolatey\lib\clamav\tools\clamav-1.4.1.win.x64\freshclam.conf";

    // Copy and rename the freshclam.conf file to the configuration directory
    fs::copy(freshclam_conf_source, &freshclam_conf_destination)?;

    println!("Copied freshclam.conf to {}", config_dir);

    let config_file = fs::File::open(freshclam_conf_destination)?;
    let reader = io::BufReader::new(config_file);

    // Use a Windows-compatible temp file path
    let temp_config_path = r"C:\ProgramData\chocolatey\lib\clamav\tools\clamav-1.4.1.win.x64\freshclam_temp.conf"; // Ensure this directory exists

    // Create a temporary file for the updated config
    let mut temp_file = fs::File::create(temp_config_path)?;

    // Process each line, filtering out lines that start with "Example"
    for line in reader.lines() {
        let line = line?;
        if !line.trim_start().starts_with("Example") {
            writeln!(temp_file, "{}", line)?;
        }
    }

    // Rename the temporary file to overwrite the original config file
        fs::rename(&temp_config_path, &freshclam_conf_destination)?;
        println!("Processed and renamed freshclam.conf at {}", config_dir);

        // Ensure the freshclam.conf file has the correct permissions for ClamAV to read
        let freshclam_permissions = fs::metadata(&freshclam_conf_destination)?.permissions();
        println!("freshclam.conf permissions: {:?}", freshclam_permissions);

        // Run freshclam to check for issues
        let freshclam_output = Command::new("freshclam")
            .arg("--config-file")
            .arg(&freshclam_conf_destination)
            .output()?;

        // Print the output and error message (if any)
        if freshclam_output.status.success() {
            println!("Freshclam ran successfully.");
        } else {
            eprintln!(
                "Freshclam failed: {}",
                String::from_utf8_lossy(&freshclam_output.stderr)
            );
        }

    Ok(())
}

fn handle_freshclam_copy(path: &str) -> std::io::Result<()>{
    let sample_path = format!("{}/freshclam.conf.sample",path);
    let config_path = &format!("{}/freshclam.conf",path);
    fs::copy(sample_path, config_path)?;

    let config_file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(config_file);

    let temp_config_path = "/tmp/freshclam_temp.conf";
    let mut temp_file = fs::File::create(temp_config_path)?;

    for line in reader.lines() {
        let line = line?;
        if !line.trim_start().starts_with("Example") {
            writeln!(temp_file, "{}", line)?;
        }
    }

    fs::rename(temp_config_path, config_path)?;
    Ok(())
}

fn install_clamav() -> std::io::Result<()> {
    match env::consts::OS {
        "linux" => {
            let _ = linux::install_clamav_linux();
            handle_freshclam_copy("/opt/homebrew/etc/clamav")?
        },
        "macos" => {
            let _ = macos::install_clamav_macos();
            handle_freshclam_copy("/usr/local/etc/clamav")?
        }
        "windows" => {
                    let _ = windows::install_clamav_windows();
                    handle_freshclam_copy_windows("C:\\ProgramData\\.clamav")?
                }
        _ => {
            eprintln!("Unsupported operating system: {}", env::consts::OS);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn run_freshclam() -> io::Result<()> {
    fn execute_command(cmd: &str, args: &[&str]) -> io::Result<()> {
        let mut process = Command::new(cmd)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = process.stdout.take().unwrap();
        let stderr = process.stderr.take().unwrap();

        let stdout_handle = thread::spawn(move || {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(e) => eprintln!("Error reading stdout: {}", e),
                }
            }
        });

        let stderr_handle = thread::spawn(move || {
            let reader = io::BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(line) => eprintln!("{}", line),
                    Err(e) => eprintln!("Error reading stderr: {}", e),
                }
            }
        });

        let output = process.wait()?;
        stdout_handle.join().unwrap(); // Wait for stdout thread to finish
        stderr_handle.join().unwrap(); // Wait for stderr thread to finish

        if output.success() {
            println!("Command executed successfully.\n");
            Ok(())
        } else {
            eprintln!("Command failed with exit code: {}", output.code().unwrap_or(-1));
            Err(io::Error::new(io::ErrorKind::Other, "Command failed"))
        }
    }

    match env::consts::OS {
        "linux" => {
            let _ = handle_freshclam_copy("/usr/local/etc/clamav");
        },
        "macos" =>{
            let _ = handle_freshclam_copy("/opt/homebrew/etc/clamav");
        },
        "windows" =>{
            let _ =  handle_freshclam_copy_windows("C:\\ProgramData\\.clamav")?;
                },
        _ => {
            eprintln!("Unsupported operating system to setup freshclam : {}", env::consts::OS);
            std::process::exit(1);
        }
    }

    if let Err(e) = execute_command("freshclam", &[""]) {
        eprintln!("Failed to run freshclam: {}", e);

        println!("Retrying with sudo...");
        if let Err(e) = execute_command("sudo", &["freshclam"]) {
            eprintln!("Failed to run freshclam with sudo: {}", e);
            Err(io::Error::new(io::ErrorKind::Other, "Failed to run freshclam with sudo"))
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

impl Antivirus {
    pub fn new(args: Args) ->  Self{
        println!("Checking if ClamAV is installed...");

        if is_clamav_installed().unwrap() {
            println!("ClamAV is already installed.\n");
        } else {
            println!("ClamAV is not installed. Installing ClamAV...");

            if let Err(e) = install_clamav() {
                eprintln!("Error installing ClamAV: {}", e);
                std::process::exit(1);
            }
        }

        if let Err(e) = run_freshclam() {
            eprintln!("Error updating ClamAV DB: {}", e);
            std::process::exit(1);
        }

        let google_chat_url = match env::var("ANTIVIRUS_GOOGLE_CHAT_URL=") {
            Ok(url) => url,
            _ => String::new()
        };

        Self {
            home_dir : env::var("HOME").expect("Failed to get HOME directory"),
            summary: String::new(),
            infected_files: String::new(),
            tmp_file: generate_random_file_name(),
            google_chat_url,
            args
        }
    }

    pub fn scan(&mut self) {

        let mut dir = &self.home_dir;

        if self.args.dir != "HOME"{
            dir = &self.args.dir;
        }
        let mut child = Command::new("clamscan")
        .args(&[
            "--archive-verbose",
            "--alert-exceeds-max=yes",
            "--alert-encrypted=yes",
            "--max-filesize=10000M",
            "--max-scansize=10000M",
            "--max-files=1000000",
            "--max-recursion=512",
            "--max-htmlnotags=256M",
            "--max-scriptnormalize=256M",
            "--max-ziptypercg=16M",
            "--pcre-max-filesize=4095M",
            "-r",
            dir
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute clamscan");

        let regex_patterns = vec![
            Regex::new(r"^----------- SCAN SUMMARY -----------").unwrap(),
            Regex::new(r"^Known viruses:").unwrap(),
            Regex::new(r"^Engine version:").unwrap(),
            Regex::new(r"^Scanned directories:").unwrap(),
            Regex::new(r"^Scanned files:").unwrap(),
            Regex::new(r"^Infected files:").unwrap(),
            Regex::new(r"^Data scanned:").unwrap(),
            Regex::new(r"^Data read:").unwrap(),
            Regex::new(r"^Time:").unwrap(),
            Regex::new(r"^Start Date:").unwrap(),
            Regex::new(r"^End Date:").unwrap(),
        ];

        let infected_regex_patterns = vec![
            Regex::new(r" FOUND$").unwrap(),
        ];

        self.summary.push_str(&format!("_Scanned directory_: `{}`\n", dir));
        self.summary.push_str(&format!("_Result Output_: `{}`\n\n", self.tmp_file));

        let mut found_infected = false;
        
        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("{}",line);
                        if regex_patterns.iter().any(|regex| regex.is_match(&line)) {
                            self.summary.push_str(&format!("{}\n", line));
                        }
                        if infected_regex_patterns.iter().any(|regex| regex.is_match(&line)) {
                            if found_infected == false {
                                found_infected = true;
                                self.infected_files.push_str("===================================================\n");
                                self.infected_files.push_str("                           *Infected File Summary*\n");
                                self.infected_files.push_str("===================================================\n\n");
                            }
                            self.infected_files.push_str(&format!("- {}\n", line));
                        }
                    },
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
        }

        let status = child.wait().expect("Failed to wait on child");
        if self.infected_files != "" {
            self.infected_files.push_str("\n_Action Required:_\n");
            self.infected_files.push_str("- Review the file and determine if it needs further action.\n");
            self.infected_files.push_str("- Consider running additional scans or consulting with security team.\n");
        }

        println!("Scan Process exited with: {}", status);

    }

    pub async fn notify(&mut self){
        if self.google_chat_url != "" {
            self.google_chat(&self.summary);
            if self.infected_files != "" {
                self.google_chat(&format!("{}",&self.infected_files));
            }
        }
    }

    fn google_chat(&self,message: &String){
        let send_message = format!(r#"{{"text": "{}"}}"#, message);

        let output = Command::new("curl")
            .arg("-X")
            .arg("POST")
            .arg("-H")
            .arg("Content-Type: application/json")
            .arg("-d")
            .arg(send_message)
            .arg(&self.google_chat_url)
            .output()
            .expect("Failed to execute curl");

        if output.status.success() {
            println!("Message sent successfully to google chat!");
        } else {
            println!("Failed to send message.");
        }
    }

    pub fn save_infected_file_on_temp(&self){
        let mut output = File::create(&self.tmp_file).unwrap();

        write!(output, "{}", self.summary).unwrap();
        if self.infected_files != "" {
            write!(output, "{}", self.infected_files).unwrap();
        }
    }

}