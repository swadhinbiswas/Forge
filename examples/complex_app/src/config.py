"""
Configuration module.
"""

import os
from pathlib import Path

APP_NAME = "examples/complex_app"
DEBUG = os.environ.get("DEBUG", "1") == "1"

# Add any additional constants or config classes here
