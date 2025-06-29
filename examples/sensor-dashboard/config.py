# Configuration for Sensor Dashboard

CONFIG = {
    # Sensor settings
    'update_interval': 2.0,  # seconds between readings
    'temperature_unit': 'celsius',  # 'celsius' or 'fahrenheit' for primary display
    
    # Display settings
    'brightness': 0.8,
    'text_color': 0xFFFFFF,  # White
    'error_color': 0xFF0000,  # Red
    'accent_color': 0x00FF00,  # Green
    
    # Sensor calibration
    'temp_offset': 0.0,  # Temperature offset in celsius
    'light_calibration': 1.0,  # Light sensor multiplier
    
    # Debug settings
    'debug_mode': True,
    'serial_output': True,
}

# Display layout configuration
DISPLAY_LAYOUT = {
    'title_y': 10,
    'temp_y': 40,
    'light_y': 70,
    'status_y': 100,
    'margin_x': 10,
}

# Sensor pin assignments (board-specific)
# Modify these based on your board
SENSOR_PINS = {
    'temperature': 'A0',
    'light': 'A1',
    'led': 'D13',  # Status LED
}
