# Basic LED Blink Example
# Perfect for getting started with CircuitPython and cpd

import board
import digitalio
import time

# Set up the onboard LED
led = digitalio.DigitalInOut(board.LED)
led.direction = digitalio.Direction.OUTPUT

print("Starting LED blink example...")
print("Press Ctrl+C to stop")

# Main loop
try:
    while True:
        # Turn LED on
        led.value = True
        print("LED ON")
        time.sleep(0.5)
        
        # Turn LED off
        led.value = False
        print("LED OFF")
        time.sleep(0.5)

except KeyboardInterrupt:
    print("\nStopping LED blink")
    led.value = False
