# Sensor Dashboard Example
# Displays sensor readings on screen with error handling

import board
import time
import displayio
import terminalio
from adafruit_display_text import label
import analogio

# Import our custom libraries
from lib.sensors import TemperatureSensor, LightSensor
from lib.display_manager import DisplayManager
from config import CONFIG

# Initialize display
display = board.DISPLAY
display_manager = DisplayManager(display)

# Initialize sensors
try:
    temp_sensor = TemperatureSensor(board.A0)
    light_sensor = LightSensor(board.A1)
    print("Sensors initialized successfully")
except Exception as e:
    print(f"Failed to initialize sensors: {e}")
    temp_sensor = None
    light_sensor = None

# Main loop
print("Starting sensor dashboard...")
last_update = 0

while True:
    current_time = time.monotonic()
    
    # Update readings every interval
    if current_time - last_update >= CONFIG['update_interval']:
        try:
            # Read sensors
            if temp_sensor:
                temp_c = temp_sensor.read_celsius()
                temp_f = temp_sensor.read_fahrenheit()
            else:
                temp_c = temp_f = 0
                
            if light_sensor:
                light_percent = light_sensor.read_percent()
            else:
                light_percent = 0
            
            # Update display
            display_manager.update_readings(
                temperature_c=temp_c,
                temperature_f=temp_f,
                light_percent=light_percent
            )
            
            # Serial output
            print(f"Temp: {temp_c:.1f}°C ({temp_f:.1f}°F), Light: {light_percent:.0f}%")
            
            last_update = current_time
            
        except Exception as e:
            print(f"Error reading sensors: {e}")
            display_manager.show_error(str(e))
    
    # Small delay to prevent overwhelming the system
    time.sleep(0.1)
