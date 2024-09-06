# ClamAV Scanner Utility

A command-line utility for scanning directories with ClamAV, updating virus definitions, and sending notifications upon completion.

## Features
- **Directory Scanning:** Specify the directory you want to scan with ClamAV.
- **Automatic Updates:** Optionally update the ClamAV virus database before scanning.
- **Notifications:** Send scan summaries to a specified notification method, such as Google Chat.

## Prerequisite
1. [Install](https://brew.sh/) Homebrew for (MacOS).

## Installation

### *Method 1*: From binaries (Unix, Linux, MacOS)
- Download the [latest release binary](https://github.com/sandeshgrangdan/antivirus/releases) for your system

### *Method 2*: Install prebuilt binaries via shell script (Linux, macOS)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/sandeshgrangdan/antivirus/releases/download/v0.1.3/antivirus-installer.sh | sh
```

### *SETUP PATH*:
To add $HOME/.cargo/bin to your PATH, either restart your shell or run:
```bash
    source $HOME/.cargo/env (sh, bash, zsh)
    source $HOME/.cargo/env.fish (fish)
```

To update, run this command to update.
```bash
antivirus-update
```

## Usage

This will install the ClamAV Antivirus
```bash
$ antivirus
```
>You need to first setup freshclam
- *(Linux)*: Create /usr/local/etc/clamav/freshclam.conf from /usr/local/etc/clamav/freshclam.conf.sample.
- Remove or comment-out the Example line from freshclam.conf
- Run freshclam to download the latest malware definitions.
```
$ antivirus -h
A command-line utility for scanning directories with ClamAV, updating virus definitions, and sending notifications.

Usage: antivirus [OPTIONS]

Options:
  -d, --dir <DIR>        The path to the directory that you want to scan. Defaults to "HOME" [default: HOME]
  -u, --update <UPDATE>  Option to update the ClamAV virus database before scanning. Defaults to "Yes" [default: Yes]
  -h, --help             Print help
  -V, --version          Print version
```

### Example
```
antivirus --dir /path/to/scan --update Yes --notify "Google Chat"
```

### Environment Variables
- **ANTIVIRUS_GOOGLE_CHAT_URL**: The environment variable that stores the webhook URLs used for sending notifications. Ensure that this variable is set to the correct URL(s) before running the utility.

## Contributing
Contributions are welcome! Please submit a pull request or open an issue to discuss any changes or improvements.
