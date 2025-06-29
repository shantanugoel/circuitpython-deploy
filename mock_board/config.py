# Configuration settings
CONFIG = {
    'sensor_pin': 'A0',
    'light_sensor_pin': 'A1',
    'led_pin': 'D13',
    'update_interval': 1.0,
    'display_brightness': 0.8,
    'debug_mode': True,
    'wifi_enabled': False,
    'temperature_unit': 'celsius',  # 'celsius' or 'fahrenheit'
    'data_logging': False,
    'max_log_entries': 100,
}

# Sensor calibration values
SENSOR_CALIBRATION = {
    'temperature_offset': 0.0,
    'light_sensitivity': 1.0,
    'temperature_scale': 1.0,
}

# Display settings
DISPLAY_CONFIG = {
    'show_temperature': True,
    'show_light': True,
    'show_time': False,
    'auto_brightness': True,
    'theme': 'dark',  # 'dark' or 'light'
}
