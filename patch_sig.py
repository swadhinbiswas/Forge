import re
with open("forge_cli/main.py", "r") as f:
    text = f.read()

text = text.replace(
"""                console.print("[cyan]Change detected[/] Restarting application...")
                process.terminate()
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait(timeout=5)
                process = _launch_dev_process_with_env(entry_path, project_dir, extra_env=extra_env)""",
"""                console.print("[cyan]Change detected[/] Reloading backend...")
                import signal
                if hasattr(signal, "SIGUSR1"):
                    process.send_signal(signal.SIGUSR1)
                else:
                    process.terminate()
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=5)
                    process = _launch_dev_process_with_env(entry_path, project_dir, extra_env=extra_env)"""
)
with open("forge_cli/main.py", "w") as f:
    f.write(text)
