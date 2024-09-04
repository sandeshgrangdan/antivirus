use std::{env, thread};
use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use regex::Regex;
use reqwest::Client;
use clap::Parser;

mod linux;
mod macos;

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

    /// The notification is sent in "Google Chat" using the URL provided in the `ANTIVIRUS_WEBHOOK_URLS` environment variable.
    #[arg(short, long, default_value_t = String::from("Google Chat"))]
    pub notify: String,
}

pub struct Antivirus {
    home_dir: String,
    webhook_url: String,
    summary: String,
    args: Args
}

fn is_clamav_installed() -> io::Result<bool> {
    let output = Command::new("which")
        .arg("clamscan")
        .output()?;

    Ok(!output.stdout.is_empty())
}

fn install_clamav() -> std::io::Result<()> {
    match env::consts::OS {
        "linux" => linux::install_clamav_linux(),
        "macos" => macos::install_clamav_macos(),
        _ => {
            eprintln!("Unsupported operating system: {}", env::consts::OS);
            std::process::exit(1);
        }
    }
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

    if let Err(e) = execute_command("freshclam", &[]) {
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

        let webhook_url = if args.notify == "Google Chat"{
            env::var("ANTIVIRUS_WEBHOOK_URL").expect("Failed to get ANTIVIRUS_WEBHOOK_URL environment variables.")
        }else{
            String::new()
        };

        Self {
            home_dir : env::var("HOME").expect("Failed to get HOME directory"),
            summary: String::new(),
            webhook_url,
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
            "--max-filesize=4095M",
            "--max-scansize=4095M",
            "--max-files=1000000",
            "--max-recursion=512",
            "--max-embeddedpe=256M",
            "--max-htmlnormalize=256M",
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

        self.summary.push_str(&format!("{}\n\n", self.home_dir));
        
        if let Some(stdout) = child.stdout.take() {
            let reader = io::BufReader::new(stdout);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        println!("{}",line);
                        if regex_patterns.iter().any(|regex| regex.is_match(&line)) {
                            self.summary.push_str(&format!("{}\n", line));
                        }
                    },
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
        }

        let status = child.wait().expect("Failed to wait on child");

        println!("Scan Process exited with: {}", status);

    }

    pub async fn notify(&self){
        if self.args.notify == "Google Chat"{
            let client = Client::new();
            let response = client.post(&self.webhook_url)
            .json(&serde_json::json!({ "text": self.summary }))
            .send()
            .await
            .expect("Failed to send request");

            println!("Message sent to Google Chat with status: {}", response.status());
        }
    }

    // fn generate_random_file_name() -> String {
    //     let mut rng = rand::thread_rng();
    //     let random_string: String = (0..10)
    //         .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
    //         .collect();
    //     format!("/tmp/{}.txt", random_string)
    // }


}