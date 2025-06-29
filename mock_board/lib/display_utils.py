# Display utility functions
import displayio
import terminalio
from adafruit_display_text import label

def create_text_group(text, x=0, y=20, color=0xFFFFFF):
    """Create a display group with text"""
    group = displayio.Group()
    text_label = label.Label(terminalio.FONT, text=text, color=color, x=x, y=y)
    group.append(text_label)
    return group

def create_centered_text(display, text, y=None, color=0xFFFFFF):
    """Create centered text on display"""
    if y is None:
        y = display.height // 2
    
    # Approximate text width calculation
    text_width = len(text) * 6  # Rough estimate
    x = (display.width - text_width) // 2
    
    return create_text_group(text, x, y, color)

def update_status_display(display, status_text, line=0):
    """Update a status line on the display"""
    y_position = 20 + (line * 20)
    group = create_text_group(status_text, 10, y_position, 0x00FF00)
    return group
