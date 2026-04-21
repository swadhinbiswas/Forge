import logging
logging.basicConfig(level=logging.DEBUG)
from src.main import get_dashboard_data, get_setting
print("testing dashboard")
print("Key:", get_setting('tmdb_key'))
print("Dash:", get_dashboard_data())
