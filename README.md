# sget

A CLI tool to download files from the web with a nice progress display.

## Installation

### From GitHub Releases

1. Go to the [Releases](https://github.com/yourusername/sget/releases) page
2. Download the latest `.deb` package
3. Install it with:
   ```bash
   sudo dpkg -i sget_*.deb
   sudo apt-get install -f  # Install any missing dependencies
   ```

### From Source

If you prefer to build from source:

```bash
# Clone the repository
git clone https://github.com/yourusername/sget.git
cd sget

# Build with cargo
cargo build --release

# The binary will be in target/release/sget
sudo cp target/release/sget /usr/local/bin/
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
