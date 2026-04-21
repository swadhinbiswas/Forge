import sys
from pathlib import Path
import re

file_path = Path("/home/swadhin/Forge/forge-framework/forge_cli/main.py")
content = file_path.read_text()

# 1. Update def create_project parameters
content = re.sub(
    r"""    author: Optional\[str\] = typer.Option\(\n        None,\n        "-a",\n        "--author",\n        help="Author name",\n    \),\n\) -> None:""",
    r'''    author: Optional[str] = typer.Option(
        None,
        "-a",
        "--author",
        help="Author name",
    ),
    package_manager: Optional[str] = typer.Option(
        None,
        "-p",
        "--package-manager",
        help="Package manager (npm, pnpm, bun, yarn, skip)",
    ),
    tailwind: Optional[bool] = typer.Option(
        None,
        "--tailwind/--no-tailwind",
        help="Include Tailwind CSS",
    ),
) -> None:''',
    content
)

# 2. Add prompts for package_manager and tailwind
prompt_code = """
    # Get author name if not provided
    if not author:
        default_author = os.environ.get("USER", os.environ.get("USERNAME", "Developer"))
        try:
            import questionary
            author = questionary.text("Author name:", default=default_author).ask()
        except ImportError:
            author = Prompt.ask("Author name", default=default_author)
        if not author: raise typer.Exit(1)

    if package_manager is None:
        try:
            import questionary
            package_manager = questionary.select(
                "Choose a package manager:",
                choices=["npm", "pnpm", "bun", "yarn", "skip"],
                default="npm"
            ).ask()
        except ImportError:
            package_manager = Prompt.ask("Package manager", choices=["npm", "pnpm", "bun", "yarn", "skip"], default="npm")
        if not package_manager: raise typer.Exit(1)

    if tailwind is None:
        if template in ["react", "vue", "svelte", "plain", "complex"]:
            try:
                import questionary
                tailwind = questionary.confirm("Include Tailwind CSS?", default=True).ask()
            except ImportError:
                ans = Prompt.ask("Include Tailwind CSS? [y/N]", default="y")
                tailwind = ans.lower().startswith('y')
            if tailwind is None: raise typer.Exit(1)
        else:
            tailwind = False
"""
content = re.sub(
    r"""    # Get author name if not provided.*?if not author: raise typer\.Exit\(1\)""",
    prompt_code,
    content,
    flags=re.DOTALL
)

# 3. Pass new args to _write_frontend_workspace_files and run install
create_dir_code = """
    with console.status("[cyan]Scaffolding files...[/]"):
        _copy_template(templates_dir, project_dir, name, author, width, height)
        _write_frontend_workspace_files(project_dir, template, name, tailwind=tailwind)
        _inject_dev_server_defaults(project_dir / "forge.toml", package_manager)

    _print_note(f"Scaffolded {name}/", level="ok")
    _print_note("Created forge.toml configuration", level="ok")
    _print_note(f"Set up {template} template", level="ok")
    _print_note(f"Template contract schema v{template_contract['schema_version']} validated", level="ok")

    if tailwind:
        _print_note("Configured Tailwind CSS + PostCSS", level="ok")

    # Create a simple icon placeholder
    _create_placeholder_icon(assets_dir / "icon.png")
    _print_note("Created placeholder icon", level="ok")

    if package_manager != "skip":
        with console.status(f"[cyan]Running {package_manager} install...[/]"):
            import subprocess
            try:
                subprocess.run([package_manager, "install"], cwd=project_dir, check=True, capture_output=True)
                _print_note(f"Installed Node dependencies via {package_manager}", level="ok")
            except Exception as e:
                _print_note(f"Failed to install dependencies: {e}", level="warning")

    with console.status("[cyan]Setting up Python environment...[/]"):
        _setup_python_env(project_dir)
"""

content = re.sub(
    r"""    with console\.status\("\[cyan\]Scaffolding files\.\.\.\[/\]"\):.*?_setup_python_env\(project_dir\)""",
    create_dir_code,
    content,
    flags=re.DOTALL
)

file_path.write_text(content)
