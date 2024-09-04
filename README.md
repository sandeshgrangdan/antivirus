# ClamAV Scanner Utility

A command-line utility for scanning directories with ClamAV, updating virus definitions, and sending notifications upon completion.

## Features
- **Directory Scanning:** Specify the directory you want to scan with ClamAV.
- **Automatic Updates:** Optionally update the ClamAV virus database before scanning.
- **Notifications:** Send scan summaries to a specified notification method, such as Google Chat.

## Prerequisite
1. [Install](https://brew.sh/) Homebrew for (MacOS).

## Installation

### From binaries (Unix, Linux, MacOS)
- Download the [latest release binary](https://github.com/sandeshgrangdan/antivirus/releases) for your system
- Set the `PATH` environment variable

### Install prebuilt binaries via shell script (Linux, macOS)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/sandeshgrangdan/antivirus/releases/download/v0.1.0/antivirus-installer.sh | sh
```

To update, run the same command again.
```bash
antivirus-update
```

## Usage

```bash
$ ssm-tui
```
You can customize the behavior of the utility using the following command-line arguments:
```
$ ssm-tui -h
A command-line utility for scanning directories with ClamAV, updating virus definitions, and sending notifications.

Usage: antivirus [OPTIONS]

Options:
  -d, --dir <DIR>        The path to the directory that you want to scan. Defaults to "HOME" [default: HOME]
  -u, --update <UPDATE>  Option to update the ClamAV virus database before scanning. Defaults to "Yes" [default: Yes]
  -n, --notify <NOTIFY>  The notification is sent in "Google Chat" using the URL provided in the `ANTIVIRUS_WEBHOOK_URLS` environment variable [default: "Google Chat"]
  -h, --help             Print help
  -V, --version          Print version
```

### Example
```
antivirus --dir /path/to/scan --update Yes --notify "Google Chat"
```

### Environment Variables
- **ANTIVIRUS_WEBHOOK_URLS**:: The environment variable that stores the webhook URLs used for sending notifications. Ensure that this variable is set to the correct URL(s) before running the utility `--notify "Google Chat"` option.

## Contributing
Contributions are welcome! Please submit a pull request or open an issue to discuss any changes or improvements.
