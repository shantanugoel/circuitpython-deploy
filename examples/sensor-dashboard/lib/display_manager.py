# Display management for sensor dashboard

import displayio
import terminalio
from adafruit_display_text import label
from config import CONFIG, DISPLAY_LAYOUT

class DisplayManager:
    """
    Manages the display layout and updates for the sensor dashboard
    """
    
    def __init__(self, display):
        self.display = display
        self.main_group = displayio.Group()
        
        # Create text labels
        self.title_label = self._create_label("Sensor Dashboard", 
                                              DISPLAY_LAYOUT['title_y'], 
                                              CONFIG['accent_color'])
        
        self.temp_label = self._create_label("Temp: --°C", 
                                             DISPLAY_LAYOUT['temp_y'], 
                                             CONFIG['text_color'])
        
        self.light_label = self._create_label("Light: --%", 
                                              DISPLAY_LAYOUT['light_y'], 
                                              CONFIG['text_color'])
        
        self.status_label = self._create_label("Initializing...", 
                                               DISPLAY_LAYOUT['status_y'], 
                                               CONFIG['accent_color'])
        
        # Add all labels to the group
        self.main_group.append(self.title_label)
        self.main_group.append(self.temp_label)
        self.main_group.append(self.light_label)
        self.main_group.append(self.status_label)
        
        # Show the group
        self.display.show(self.main_group)
        
        # Track last update time for status
        import time
        self.last_update = time.monotonic()
    
    def _create_label(self, text, y_pos, color):
        """Create a text label with consistent styling"""
        return label.Label(
            terminalio.FONT, 
            text=text, 
            color=color, 
            x=DISPLAY_LAYOUT['margin_x'], 
            y=y_pos
        )
    
    def update_readings(self, temperature_c=None, temperature_f=None, light_percent=None):
        """Update sensor readings on display"""
        import time
        
        # Update temperature display
        if temperature_c is not None:
            if CONFIG['temperature_unit'] == 'fahrenheit' and temperature_f is not None:
                self.temp_label.text = f"Temp: {temperature_f:.1f}°F"
            else:
                self.temp_label.text = f"Temp: {temperature_c:.1f}°C"
        
        # Update light display
        if light_percent is not None:
            self.light_label.text = f"Light: {light_percent:.0f}%"
        
        # Update status
        self.status_label.text = "Running"
        self.status_label.color = CONFIG['accent_color']
        self.last_update = time.monotonic()
    
    def show_error(self, error_message):
        """Display error message"""
        self.status_label.text = f"Error: {error_message[:20]}"
        self.status_label.color = CONFIG['error_color']
    
    def show_startup(self):
        """Display startup message"""
        self.status_label.text = "Starting up..."
        self.status_label.color = CONFIG['accent_color']
    
    def show_no_sensors(self):
        """Display message when sensors are not available"""
        self.temp_label.text = "Temp: No sensor"
        self.light_label.text = "Light: No sensor"
        self.status_label.text = "Check wiring"
        self.status_label.color = CONFIG['error_color']
    
    def update_status(self, message, is_error=False):
        """Update just the status line"""
        self.status_label.text = message
        if is_error:
            self.status_label.color = CONFIG['error_color']
        else:
            self.status_label.color = CONFIG['accent_color']
    
    def set_brightness(self, brightness):
        """Set display brightness (0.0 to 1.0)"""
        if hasattr(self.display, 'brightness'):
            self.display.brightness = brightness
    
    def clear_display(self):
        """Clear the display"""
        self.main_group = displayio.Group()
        self.display.show(self.main_group)
    
    def get_uptime_string(self):
        """Get formatted uptime string"""
        import time
        uptime = time.monotonic()
        
        hours = int(uptime // 3600)
        minutes = int((uptime % 3600) // 60)
        seconds = int(uptime % 60)
        
        if hours > 0:
            return f"{hours:02d}:{minutes:02d}:{seconds:02d}"
        else:
            return f"{minutes:02d}:{seconds:02d}"
