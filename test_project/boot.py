# Boot script for CircuitPython
import board
print("Boot script running...")

# Configure USB and console
import usb_cdc
usb_cdc.enable(console=True, data=False)
