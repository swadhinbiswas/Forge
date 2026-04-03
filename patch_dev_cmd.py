import re

with open("forge_cli/main.py", "r") as f:
    code = f.read()

# Add inspect param
new_param = """    watch: bool = typer.Option(
        True,
        "--watch/--no-watch",
        help="Restart the Python app when project files change",
    ),
    inspect: bool = typer.Option(
        False,
        "--inspect/--no-inspect",
        help="Enable IPC traffic inspection",
    ),"""
code = code.replace("""    watch: bool = typer.Option(
        True,
        "--watch/--no-watch",
        help="Restart the Python app when project files change",
    ),""", new_param)

# Check inspect inside dev_mode
if_inspect = """    _print_note("Launching dev process", level="ok")
    if inspect:
        os.environ["FORGE_INSPECT"] = "1"
    _run_dev_loop(project_dir, config, hot_reload and watch)"""
code = code.replace("""    _print_note("Launching dev process", level="ok")
    _run_dev_loop(project_dir, config, hot_reload and watch)""", if_inspect)

with open("forge_cli/main.py", "w") as f:
    f.write(code)

