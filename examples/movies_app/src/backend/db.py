import sqlite3
import threading
import contextlib
import logging
from pathlib import Path
import os
import time

BASE_DIR = Path(__file__).parent.parent.parent
DB_PATH = (BASE_DIR / "app_data.db").resolve()

db_lock = threading.Lock()

def init_db():
    logging.info(f"Initializing DB at {DB_PATH}")
    with db_lock:
        with contextlib.closing(sqlite3.connect(DB_PATH, timeout=10, check_same_thread=False)) as conn:
            with conn:
                conn.execute('CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT)')
                # Watchlist bound to user
                conn.execute('CREATE TABLE IF NOT EXISTS watchlist (id INTEGER, user_id INTEGER, title TEXT, poster TEXT, rating REAL, PRIMARY KEY (id, user_id))')
                # Users and Sessions
                conn.execute('CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE, password TEXT)')
                conn.execute('CREATE TABLE IF NOT EXISTS sessions (token TEXT PRIMARY KEY, user_id INTEGER, created_at REAL)')

