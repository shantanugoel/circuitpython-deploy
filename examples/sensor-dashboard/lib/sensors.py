# Sensor library for reading temperature and light levels

import board
import analogio
from config import CONFIG

class TemperatureSensor:
    """
    Temperature sensor using analog input.
    Assumes TMP36 or similar analog temperature sensor.
    """
    
    def __init__(self, pin):
        self.analog_in = analogio.AnalogIn(pin)
        self.offset = CONFIG.get('temp_offset', 0.0)
    
    def read_raw(self):
        """Read raw analog value (0-65535)"""
        return self.analog_in.value
    
    def read_voltage(self):
        """Convert raw reading to voltage"""
        return (self.analog_in.value * 3.3) / 65536
    
    def read_celsius(self):
        """Read temperature in Celsius"""
        voltage = self.read_voltage()
        # TMP36: 10mV/°C with 500mV offset at 0°C
        temp_c = (voltage - 0.5) * 100 + self.offset
        return temp_c
    
    def read_fahrenheit(self):
        """Read temperature in Fahrenheit"""
        return self.read_celsius() * 9/5 + 32

class LightSensor:
    """
    Light sensor using analog input.
    Can be used with photoresistors or photodiodes.
    """
    
    def __init__(self, pin):
        self.analog_in = analogio.AnalogIn(pin)
        self.calibration = CONFIG.get('light_calibration', 1.0)
    
    def read_raw(self):
        """Read raw analog value (0-65535)"""
        return self.analog_in.value
    
    def read_voltage(self):
        """Convert raw reading to voltage"""
        return (self.analog_in.value * 3.3) / 65536
    
    def read_percent(self):
        """Read light level as percentage (0-100%)"""
        raw_percent = (self.analog_in.value / 65536) * 100
        return raw_percent * self.calibration
    
    def read_lux(self, r_fixed=10000):
        """
        Estimate lux value for photoresistor setup.
        r_fixed: value of fixed resistor in voltage divider (ohms)
        """
        voltage = self.read_voltage()
        if voltage == 0:
            return 0
        
        # Calculate resistance of photoresistor
        r_photo = r_fixed * (3.3 - voltage) / voltage
        
        # Convert to approximate lux (very rough estimation)
        # This would need calibration for accurate readings
        if r_photo > 0:
            lux = 500000 / r_photo  # Rough approximation
        else:
            lux = 0
        
        return lux

class StatusLED:
    """
    Simple status LED controller
    """
    
    def __init__(self, pin):
        import digitalio
        self.led = digitalio.DigitalInOut(pin)
        self.led.direction = digitalio.Direction.OUTPUT
        self.state = False
    
    def on(self):
        """Turn LED on"""
        self.led.value = True
        self.state = True
    
    def off(self):
        """Turn LED off"""
        self.led.value = False
        self.state = False
    
    def toggle(self):
        """Toggle LED state"""
        if self.state:
            self.off()
        else:
            self.on()
    
    def blink(self, duration=0.1):
        """Quick blink"""
        self.on()
        import time
        time.sleep(duration)
        self.off()
