import sys
from pathlib import Path
import re

file_path = Path("/home/swadhin/Forge/forge-framework/forge_cli/main.py")
content = file_path.read_text()

# Update signature
content = content.replace(
    "def _write_frontend_workspace_files(project_dir: Path, template: str, name: str) -> None:",
    "def _write_frontend_workspace_files(project_dir: Path, template: str, name: str, tailwind: bool = False) -> None:"
)

# Update _write_frontend_workspace_files body
injection_code = """
    if tailwind:
        dev_dependencies.update({
            "tailwindcss": "^3.4.1",
            "postcss": "^8.4.38",
            "autoprefixer": "^10.4.19"
        })
        
        # Write Tailwind config
        (project_dir / "tailwind.config.js").write_text(
            "/** @type {import('tailwindcss').Config} */\\n"
            "export default {\\n"
            "  content: [\\n"
            "    \\"./src/frontend/**/*.{js,ts,jsx,tsx,vue,svelte,html}\\",\\n"
            "  ],\\n"
            "  theme: {\\n"
            "    extend: {},\\n"
            "  },\\n"
            "  plugins: [],\\n"
            "}\\n",
            encoding="utf-8"
        )
        
        # Write PostCSS config
        (project_dir / "postcss.config.js").write_text(
            "export default {\\n"
            "  plugins: {\\n"
            "    tailwindcss: {},\\n"
            "    autoprefixer: {},\\n"
            "  },\\n"
            "}\\n",
            encoding="utf-8"
        )
        
        # Inject tailwind directives into main css file
        css_file = project_dir / "src" / "frontend" / "index.css"
        if template in ["vue", "svelte", "plain"]:
            css_file = project_dir / "src" / "frontend" / "style.css"
        
        if css_file.exists():
            original_css = css_file.read_text("utf-8")
            css_file.write_text(
                "@tailwind base;\\n@tailwind components;\\n@tailwind utilities;\\n\\n" + original_css,
                encoding="utf-8"
            )

    package_json = {
"""
content = re.sub(
    r"    package_json = \{",
    injection_code,
    content,
    count=1
)

# Update _inject_dev_server_defaults
content = content.replace(
    "def _inject_dev_server_defaults(config_path: Path) -> None:",
    "def _inject_dev_server_defaults(config_path: Path, package_manager: str = \"npm\") -> None:"
)

content = content.replace(
    "        + 'dev_server_command = \"npm run dev\"\\n'",
    "        + f'dev_server_command = \"{package_manager} run dev\"\\n'"
)

file_path.write_text(content)
