# Basic LED Blink Example

A simple CircuitPython project that blinks the onboard LED. Perfect for testing your board and learning how to use `cpd`.

## 🔧 Hardware Required

- Any CircuitPython-compatible board with an onboard LED
- USB cable for connection

## 🚀 Quick Start

1. **Deploy to your board:**
   ```bash
   cd examples/basic-led-blink
   cpd
   ```

2. **The LED should start blinking immediately!**

## 📁 Project Structure

```
basic-led-blink/
├── code.py      # Main program (blinks LED)
├── .cpdignore   # Excludes README.md from deployment
└── README.md    # This file (not deployed)
```

## 💡 What You'll Learn

- Basic CircuitPython GPIO control
- How to use `cpd` for deployment
- Simple `.cpdignore` patterns
- Serial output monitoring

## 🔍 Deployment Details

When you run `cpd`, it will:

1. **Detect your CircuitPython board** automatically
2. **Copy only `code.py`** (README.md is excluded by .cpdignore)
3. **Show progress** with a visual progress bar
4. **Complete in milliseconds** for this simple project

## 🧪 Testing

```bash
# Preview what will be deployed
cpd --dry-run

# Deploy with verbose output
cpd --verbose

# Create backup before deployment
cpd --backup ./backup
```

## 🔧 Customization

Try modifying the code:

```python
# Change blink speed
time.sleep(0.1)  # Faster blink

# Add more LEDs if your board has them
# Check your board's pinout guide
```

## 📊 Expected Output

After deployment, you should see output like:
```
Starting LED blink example...
Press Ctrl+C to stop
LED ON
LED OFF
LED ON
LED OFF
...
```

## 🐛 Troubleshooting

**LED not blinking?**
- Check that CircuitPython is installed (not MicroPython)
- Verify the board has an onboard LED at `board.LED`
- Try pressing the RESET button

**cpd not finding board?**
- Run `cpd --list-boards` to see detected boards
- Check USB connection
- Make sure board appears as removable drive

**Permission errors?**
- On Linux/macOS: Add yourself to dialout group
- Try running with appropriate permissions

## ➡️ Next Steps

Once this works, try:
- [Sensor Dashboard Example](../sensor-dashboard/) - More complex project
- [WiFi Weather Station](../wifi-weather/) - Network-enabled project
- Creating your own custom project
