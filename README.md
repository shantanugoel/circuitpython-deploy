# CircuitPython Deploy (cpd)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)

A fast, reliable command-line tool for deploying CircuitPython projects from your development environment to CircuitPython boards.

## ✨ Features

- 🔍 **Automatic board detection** - Finds CircuitPython boards automatically
- 📁 **Smart file filtering** - Supports `.cpdignore` files with gitignore-style patterns
- 💾 **Backup functionality** - Safely backup existing board contents before deployment
- 📊 **Progress tracking** - Visual progress bars for file operations
- 🚀 **High performance** - Deploy dozens of files in milliseconds
- 🔧 **Cross-platform** - Works on Windows, macOS, and Linux
- 🎯 **Dry-run mode** - Preview deployments without making changes
- 💬 **Helpful error messages** - Clear guidance when things go wrong

## 🚀 Quick Start

```bash
# List detected CircuitPython boards
cpd --list-boards

# Deploy current directory to auto-detected board
cpd

# Deploy with backup
cpd --backup ./backup

# Preview deployment (dry-run)
cpd --dry-run

# Deploy to specific board
cpd --board /media/CIRCUITPY
```

## 📦 Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/yourusername/circuitpython-deploy/releases).

### From Source

```bash
# Install Rust if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build
git clone https://github.com/yourusername/circuitpython-deploy.git
cd circuitpython-deploy
cargo build --release

# The binary will be at target/release/cpd
```

### Package Managers

Coming soon: Homebrew, Chocolatey, and AUR packages.

## 📖 Usage

### Basic Deployment

Deploy your current directory to an automatically detected CircuitPython board:

```bash
cpd
```

### Board Management

```bash
# List all detected CircuitPython boards
cpd --list-boards

# Deploy to a specific board
cpd --board E:\              # Windows
cpd --board /media/CIRCUITPY # Linux
cpd --board /Volumes/CIRCUITPY # macOS
```

### Backup and Safety

```bash
# Create backup before deployment
cpd --backup ./my-backup

# Preview what would be deployed (safe)
cpd --dry-run

# Verbose output for debugging
cpd --verbose

# Force deployment without confirmation
cpd --yes
```

### File Filtering

Create a `.cpdignore` file in your project root to exclude files:

```gitignore
# Ignore development files
*.test.py
test_*
docs/
.vscode/
__pycache__/

# Ignore backup directories
backups/*
*.bak

# Ignore temporary files
*.tmp
*.log
```

## 🏗️ Project Structure

A typical CircuitPython project structure that works well with `cpd`:

```
my-circuitpython-project/
├── code.py              # Main entry point
├── boot.py              # Boot configuration
├── settings.toml        # Configuration settings
├── lib/                 # Libraries
│   ├── sensors.py
│   └── display_utils.py
├── assets/              # Images, sounds, etc.
│   └── icon.bmp
├── .cpdignore          # Files to exclude
└── README.md           # Project documentation
```

## 🔧 Advanced Usage

### Multiple Boards

When multiple CircuitPython boards are connected:

```bash
# List boards and select interactively
cpd

# Or specify the board directly
cpd --board /dev/sdb1
```

### Integration with Development Workflow

```bash
# Quick deploy during development
cpd --dry-run && cpd --yes

# Deploy with backup for important updates
cpd --backup "./backups/$(date +%Y%m%d_%H%M%S)"

# Check what would be deployed
cpd --verbose --dry-run
```

### Configuration

No configuration file needed! `cpd` works out of the box with sensible defaults:

- Automatically excludes `.git/`, `__pycache__/`, `node_modules/`, etc.
- Respects `.cpdignore` and `.gitignore` patterns
- Preserves file timestamps
- Shows progress for operations taking more than a second

## 🐛 Troubleshooting

### Board Not Detected

If your CircuitPython board isn't detected:

1. **Check USB connection** - Ensure the board is properly connected
2. **Verify CircuitPython** - Make sure CircuitPython is installed (not MicroPython)
3. **Check mount point** - The board should appear as a removable drive
4. **Try reset** - Press the RESET button on your board
5. **Manual specification** - Use `--board <path>` to specify manually

### Common Issues

**Permission Denied**
```bash
# Linux/macOS: You might need to be in the dialout group
sudo usermod -a -G dialout $USER
# Then log out and back in
```

**No Files Deployed**
```bash
# Check what files would be included
cpd --verbose --dry-run

# Common causes:
# - All files excluded by .cpdignore
# - Empty project directory
# - All files already up to date
```

**Slow Performance**
```bash
# For very large projects, consider:
# - Adding more patterns to .cpdignore
# - Using --yes to skip confirmation
# - Excluding documentation/test directories
```

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/yourusername/circuitpython-deploy.git
cd circuitpython-deploy

# Run tests
cargo test

# Run integration tests
cargo test --test integration_test

# Build release
cargo build --release
```

## 📋 Requirements

- **Rust 1.75+** (for building from source)
- **CircuitPython board** with CircuitPython firmware installed
- **USB connection** or network access to the board

### Supported Boards

`cpd` works with any CircuitPython-compatible board including:

- Adafruit Feather series (RP2040, ESP32-S2, ESP32-S3, etc.)
- Raspberry Pi Pico with CircuitPython
- Adafruit Metro series
- Adafruit ItsyBitsy series
- Adafruit QT Py series
- And many more!

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Adafruit](https://adafruit.com) for CircuitPython
- [CircuitPython Community](https://circuitpython.org) for the amazing ecosystem
- All the Rust crate authors whose work made this tool possible

## 🔗 Related Projects

- [CircuitPython](https://circuitpython.org) - The Python implementation for microcontrollers
- [Adafruit CircuitPython Bundle](https://github.com/adafruit/Adafruit_CircuitPython_Bundle) - Library collection
- [circup](https://github.com/adafruit/circup) - CircuitPython library updater

---

Made with ❤️ for the CircuitPython community
