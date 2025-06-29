# Sensor Dashboard Example

A comprehensive CircuitPython project that reads temperature and light sensors and displays the data on screen. This example demonstrates advanced project organization, error handling, and configuration management.

## 🔧 Hardware Required

- CircuitPython board with display (e.g., Adafruit PyPortal, Matrix Portal, or board + separate display)
- TMP36 temperature sensor (or similar analog temperature sensor)
- Photoresistor and 10kΩ resistor (for light sensing)
- Breadboard and jumper wires

## 🔌 Wiring

```
Temperature Sensor (TMP36):
- VCC → 3.3V
- GND → GND  
- OUT → A0

Light Sensor (Photoresistor):
- One leg → 3.3V
- Other leg → A1 and one leg of 10kΩ resistor
- Other leg of 10kΩ resistor → GND
```

## 🚀 Quick Start

1. **Wire up your sensors** according to the diagram above
2. **Deploy the project:**
   ```bash
   cd examples/sensor-dashboard
   cpd --verbose
   ```
3. **Watch the dashboard update** with live sensor readings!

## 📁 Project Structure

```
sensor-dashboard/
├── code.py                 # Main application
├── config.py              # Configuration settings
├── lib/                   # Custom libraries
│   ├── sensors.py         # Sensor reading classes
│   └── display_manager.py # Display management
├── .cpdignore            # Deployment exclusions
└── README.md             # This file (not deployed)
```

## ⚙️ Configuration

Edit `config.py` to customize:

```python
CONFIG = {
    'update_interval': 2.0,        # Seconds between readings
    'temperature_unit': 'celsius', # 'celsius' or 'fahrenheit'
    'brightness': 0.8,             # Display brightness
    'temp_offset': 0.0,            # Temperature calibration
}
```

## 📊 Features

- **Real-time sensor readings** updated every 2 seconds
- **Temperature display** in Celsius or Fahrenheit
- **Light level percentage** from photoresistor
- **Error handling** with on-screen error messages
- **Status display** showing system state
- **Modular code** organized into libraries
- **Configurable settings** for easy customization

## 🧪 Testing Deployment

```bash
# Preview what will be deployed
cpd --dry-run --verbose

# Deploy with backup (recommended for first time)
cpd --backup ./backup

# Quick deployment during development
cpd --yes
```

## 📖 Code Organization

### Main Application (`code.py`)
- Initializes hardware and display
- Main sensor reading loop
- Error handling and recovery

### Configuration (`config.py`)
- Centralized settings
- Pin assignments
- Display layout configuration

### Sensor Library (`lib/sensors.py`)
- `TemperatureSensor` class for analog temperature reading
- `LightSensor` class for photoresistor readings
- `StatusLED` class for visual feedback

### Display Manager (`lib/display_manager.py`)
- Screen layout management
- Text label updates
- Error message display

## 🔧 Customization

### Adding New Sensors

1. **Add sensor class to `lib/sensors.py`:**
   ```python
   class HumiditySensor:
       def __init__(self, pin):
           # Sensor initialization
       
       def read_percent(self):
           # Reading logic
   ```

2. **Update display in `lib/display_manager.py`**
3. **Modify main loop in `code.py`**

### Changing Pin Assignments

Edit the `SENSOR_PINS` section in `config.py`:

```python
SENSOR_PINS = {
    'temperature': 'A2',  # Changed from A0
    'light': 'A3',        # Changed from A1
    'led': 'D13',
}
```

### Customizing Display Layout

Modify `DISPLAY_LAYOUT` in `config.py`:

```python
DISPLAY_LAYOUT = {
    'title_y': 10,
    'temp_y': 40,
    'light_y': 70,
    'status_y': 100,
    'margin_x': 10,
}
```

## 🐛 Troubleshooting

### No Sensor Readings
- Check wiring connections
- Verify sensors are getting power
- Test with multimeter if available

### Display Issues
- Ensure board has built-in display or display is connected
- Check `board.DISPLAY` is available
- Verify display library imports

### Deployment Issues
- Run `cpd --list-boards` to verify board detection
- Check that required libraries are available
- Use `cpd --verbose --dry-run` to see what will be deployed

### Temperature Readings Seem Wrong
- Adjust `temp_offset` in config.py
- Verify TMP36 wiring (curved side facing you: +3.3V, signal, GND)
- Check if sensor needs different conversion formula

### Light Readings Not Changing
- Verify photoresistor and resistor values
- Check voltage divider wiring
- Adjust `light_calibration` in config.py

## 📈 Expected Output

Serial monitor should show:
```
Sensors initialized successfully
Starting sensor dashboard...
Temp: 23.5°C (74.3°F), Light: 45%
Temp: 23.7°C (74.7°F), Light: 43%
Temp: 23.6°C (74.5°F), Light: 47%
```

Display should show:
```
Sensor Dashboard
Temp: 23.6°C
Light: 47%
Running
```

## ➡️ Next Steps

- Try the [WiFi Weather Station](../wifi-weather/) example
- Add data logging to a file
- Implement sensor alerts/thresholds
- Add network connectivity for remote monitoring
- Create custom sensor calibration routines

## 🔗 Related Libraries

This example uses several CircuitPython libraries:
- `adafruit_display_text` - Text rendering
- `displayio` - Display management
- Built-in `analogio` - Analog sensor reading

Make sure these are installed on your board via `circup` or manual installation.
