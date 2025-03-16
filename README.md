# sget

A CLI tool to download files from the web with a nice progress display.

## Installation from GitHub Packages

### Add the repository

```bash
# Create a file for the repository configuration
echo "deb [trusted=yes] https://$(whoami):${GITHUB_TOKEN}@maven.pkg.github.com/yourusername/sget /" | sudo tee /etc/apt/sources.list.d/yourusername-sget.list

# Update package lists
sudo apt update
```

### Install the package

```bash
sudo apt install sget
```

## Manual Installation

If you prefer to install manually:

1. Download the latest .deb package from the [Releases](https://github.com/yourusername/sget/releases) page
2. Install with:
   ```
   sudo dpkg -i sget_*.deb
   sudo apt-get install -f  # Install any missing dependencies
   ```

## Usage

```
USAGE:
    sget [OPTIONS] <URL>

OPTIONS:
    -h, --help       Print help information
    -o, --output     Specify output file
    -q, --quiet      No progress bar, just download
    -v, --verbose    Show verbose output
```

## Examples

```bash
# Download a file with default progress bar
sget https://example.com/file.zip

# Download and specify output name
sget https://example.com/file.zip -o myfile.zip

# Show verbose output
sget https://example.com/file.zip -v
```
