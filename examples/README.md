# CircuitPython Deploy Examples

This directory contains example CircuitPython projects that demonstrate how to use `cpd` effectively.

## 📁 Example Projects

### [Basic LED Blink](basic-led-blink/)
A simple LED blinking project perfect for getting started with CircuitPython and `cpd`.

**Features:**
- Basic GPIO control
- Simple .cpdignore usage
- Minimal project structure

**Usage:**
```bash
cd examples/basic-led-blink
cpd --dry-run  # Preview deployment
cpd            # Deploy to board
```

### [Sensor Dashboard](sensor-dashboard/)
A more advanced project that reads sensor data and displays it on an OLED screen.

**Features:**
- Multiple library files
- Configuration management
- Asset files (fonts, images)
- Comprehensive .cpdignore

**Usage:**
```bash
cd examples/sensor-dashboard
cpd --backup ./backup  # Deploy with backup
```

### [WiFi Weather Station](wifi-weather/)
A complete weather station that connects to WiFi and fetches weather data.

**Features:**
- Network configuration
- API integration
- Data logging
- Error handling
- Settings management

**Usage:**
```bash
cd examples/wifi-weather
# Edit settings.toml with your WiFi credentials
cpd --verbose  # Deploy with detailed output
```

## 🔧 Common Patterns

### Project Structure
```
my-project/
├── code.py           # Main entry point
├── boot.py           # Boot configuration (optional)
├── settings.toml     # Configuration
├── lib/              # Custom libraries
│   ├── sensors.py
│   └── display.py
├── assets/           # Static assets
│   ├── fonts/
│   └── images/
├── .cpdignore       # Deployment exclusions
└── README.md        # Project documentation
```

### Effective .cpdignore Patterns
```gitignore
# Development files
*.test.py
test_*
docs/
examples/
.vscode/
.idea/

# Python artifacts
__pycache__/
*.pyc
.pytest_cache/

# Backup and temporary files
backups/
*.bak
*.tmp
*.log

# OS artifacts
.DS_Store
Thumbs.db
.Trash-*

# Large assets not needed on device
assets/raw/
assets/*.psd
```

### Deployment Workflows

#### Development Workflow
```bash
# Quick iteration during development
cpd --dry-run && cpd --yes
```

#### Production Deployment
```bash
# Safe deployment with backup
cpd --backup "./backups/$(date +%Y%m%d_%H%M%S)" --verbose
```

#### Testing Deployment
```bash
# Test deployment to specific board
cpd --board /dev/sdb1 --dry-run
```

## 🧪 Testing Your Project

Before deploying, you can test your project structure:

```bash
# Check what files would be deployed
cpd --dry-run --verbose

# Verify ignore patterns are working
cpd --list-boards  # Ensure board is detected
```

## 💡 Tips and Best Practices

### 1. Use Meaningful File Names
```python
# Good
sensor_manager.py
display_controller.py
network_handler.py

# Avoid
utils.py
helper.py
stuff.py
```

### 2. Organize Libraries Logically
```
lib/
├── hardware/
│   ├── sensors.py
│   └── display.py
├── network/
│   ├── wifi_manager.py
│   └── api_client.py
└── utils/
    ├── config.py
    └── logging.py
```

### 3. Use Configuration Files
```toml
# settings.toml
[wifi]
ssid = "YourNetwork"
password = "YourPassword"

[sensor]
update_interval = 5.0
calibration_offset = 0.5

[display]
brightness = 0.8
rotation = 90
```

### 4. Version Control Integration
```bash
# Add to .gitignore
echo "settings.toml" >> .gitignore
echo "*.log" >> .gitignore

# But include template
cp settings.toml settings.toml.example
git add settings.toml.example
```

### 5. Backup Important Configurations
```bash
# Before major updates
cpd --backup "./backups/before-update-$(date +%Y%m%d)"
```

## 🔍 Debugging Deployment Issues

### Check File Inclusion
```bash
# See exactly what files will be deployed
cpd --verbose --dry-run
```

### Verify Board Detection
```bash
# List all detected boards
cpd --list-boards

# Check if specific path is a valid board
cpd --board /path/to/board --dry-run
```

### Test Ignore Patterns
Create a simple test:
```bash
# Create test files
touch test_file.py
mkdir test_dir
touch test_dir/test.py

# Check if they're ignored
cpd --dry-run --verbose
```

## 🚀 Advanced Usage

### Multiple Board Management
```bash
# Deploy to different boards for testing
cpd --board /dev/board1 --dry-run
cpd --board /dev/board2 --dry-run

# Use different ignore patterns per board
cp .cpdignore .cpdignore.prod
# Edit .cpdignore.prod for production
```

### Automated Deployment
```bash
#!/bin/bash
# deploy.sh

# Backup current state
BACKUP_DIR="./backups/$(date +%Y%m%d_%H%M%S)"
cpd --backup "$BACKUP_DIR"

# Deploy new version
cpd --yes

# Verify deployment
if cpd --list-boards | grep -q "CIRCUITPY"; then
    echo "✅ Deployment successful!"
else
    echo "❌ Board not detected after deployment"
    exit 1
fi
```

## 📚 Learning Resources

- [CircuitPython Documentation](https://docs.circuitpython.org/)
- [Adafruit Learn System](https://learn.adafruit.com/category/circuitpython)
- [CircuitPython Community Discord](https://adafru.it/discord)
- [Awesome CircuitPython](https://github.com/adafruit/awesome-circuitpython)
