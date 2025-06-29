# Large CircuitPython project
import board
import digitalio
import time
import busio
import displayio
import terminalio
from adafruit_display_text import label

# Initialize display
display = board.DISPLAY

# Create a group to hold everything
main_group = displayio.Group()

# Create text labels
title = label.Label(terminalio.FONT, text="CircuitPython Demo", color=0xFFFFFF, x=10, y=20)
status = label.Label(terminalio.FONT, text="Status: Running", color=0x00FF00, x=10, y=40)

main_group.append(title)
main_group.append(status)

# Show the group
display.show(main_group)

# Main loop
while True:
    print("Large project running...")
    time.sleep(1)
