#!/usr/bin/env python3
"""
Security verification tests for Forge Framework.
Run this to verify all security fixes are working.
"""

import sys
import tempfile
import os
from pathlib import Path
from unittest.mock import MagicMock

# Import modules directly, bypassing forge/__init__.py
import importlib.util


def load_module(name, path):
    """Load a module from a file path."""
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod  # Register so dataclasses can resolve __module__
    spec.loader.exec_module(mod)
    return mod


def main():
    print("Loading Forge modules...")

    # Load modules
    config_mod = load_module("config", "forge/config.py")
    ForgeConfig = config_mod.ForgeConfig
    ConfigValidationError = config_mod.ConfigValidationError

    events_mod = load_module("events", "forge/events.py")
    EventEmitter = events_mod.EventEmitter

    fs_mod = load_module("fs", "forge/api/fs.py")
    FileSystemAPI = fs_mod.FileSystemAPI

    bridge_mod = load_module("bridge", "forge/bridge.py")
    IPCBridge = bridge_mod.IPCBridge

    print("✓ All core modules loaded successfully\n")

    # Test 1: Config validation
    print("Test 1: Configuration loading")
    with tempfile.TemporaryDirectory() as tmp:
        config_file = Path(tmp) / "forge.toml"
        config_file.write_text('[app]\nname = "Test App"\n')
        config = ForgeConfig.from_file(config_file)
        print(f"  ✓ Config loading works: {config.app.name}")

    # Test 2: Event emitter
    print("\nTest 2: Event emitter")
    emitter = EventEmitter()
    received = []
    emitter.on("test", lambda d: received.append(d))
    emitter.emit("test", {"value": 42})
    print(f"  ✓ Event emitter works: received={received}")

    # Test 3: File System Security
    print("\nTest 3: File System Security")
    with tempfile.TemporaryDirectory() as tmp:
        fs = FileSystemAPI(base_path=Path(tmp))

        # Test path traversal
        try:
            fs.read("../../../etc/passwd")
            print("  ✗ SECURITY FAIL: path traversal allowed")
            return 1
        except ValueError:
            print("  ✓ Path traversal blocked")

        # Test null byte
        try:
            fs.read("test\x00.txt")
            print("  ✗ SECURITY FAIL: null byte allowed")
            return 1
        except ValueError:
            print("  ✓ Null byte injection blocked")

        # Test empty path
        try:
            fs.read("")
            print("  ✗ SECURITY FAIL: empty path allowed")
            return 1
        except ValueError:
            print("  ✓ Empty path blocked")

    # Test 4: Bridge Security
    print("\nTest 4: IPC Bridge Security")
    bridge = IPCBridge(MagicMock(), {})

    # Valid names
    assert bridge._validate_command_name("greet") == True
    assert bridge._validate_command_name("get_system_info") == True
    assert bridge._validate_command_name("_private") == True
    print("  ✓ Valid command names accepted")

    # Invalid names
    assert bridge._validate_command_name("") == False
    assert bridge._validate_command_name("123start") == False
    assert bridge._validate_command_name("cmd-name") == False
    assert bridge._validate_command_name("../etc/passwd") == False
    assert bridge._validate_command_name("cmd;rm -rf /") == False
    assert bridge._validate_command_name("cmd$name") == False
    print("  ✓ Invalid command names blocked")

    # Test error sanitization
    error_msg = f"Error: {os.getcwd()}/secret/key.txt"
    sanitized = bridge._sanitize_error(Exception(error_msg))
    assert os.getcwd() not in sanitized
    print("  ✓ Error sanitization removes paths")

    # Test invoke with invalid JSON
    result = bridge.invoke_command("not json")
    assert "error" in result.lower()
    print("  ✓ Invalid JSON handled gracefully")

    # Test invoke with missing command
    result = bridge.invoke_command('{"args": {}, "id": 1}')
    assert "error" in result.lower()
    print("  ✓ Missing command field handled")

    # Test invoke with invalid command name
    result = bridge.invoke_command('{"command": "../../../etc/passwd", "args": {}}')
    assert "error" in result.lower()
    print("  ✓ Malicious command name blocked")

    # Test 5: Request size limit
    print("\nTest 5: DoS Prevention")
    large_request = '{"command": "test", "args": {"data": "' + "x" * (11 * 1024 * 1024) + '"}}'
    result = bridge.invoke_command(large_request)
    assert "error" in result.lower()
    print("  ✓ Large requests rejected")

    print("\n" + "=" * 50)
    print("ALL SECURITY CHECKS PASSED!")
    print("=" * 50)
    return 0


if __name__ == "__main__":
    sys.exit(main())
