# Sensor library
import board
import analogio

class TemperatureSensor:
    def __init__(self, pin):
        self.analog_in = analogio.AnalogIn(pin)
    
    def read_celsius(self):
        # Convert analog reading to temperature
        voltage = (self.analog_in.value * 3.3) / 65536
        temp_c = (voltage - 0.5) * 100
        return temp_c
    
    def read_fahrenheit(self):
        return self.read_celsius() * 9/5 + 32

class LightSensor:
    def __init__(self, pin):
        self.analog_in = analogio.AnalogIn(pin)
    
    def read_raw(self):
        return self.analog_in.value
    
    def read_percent(self):
        return (self.analog_in.value / 65536) * 100
