import re

with open("tests/test_release_artifacts.py", "r") as f:
    text = f.read()

# Fix the dict missing 'provenance'
# Let's just find the dictionary values around line 64 and fix
# Replace '    "target": "desktop",\n        "source_commit": "abc123",' with '    "target": "desktop",' and remove the block
block_to_remove = '''        "source_commit": "abc123",
        "workspace_root": str(workspace),
        "python_version": "3.14.3",
        "package_versions": [{"name": "@forge/api", "version": "2.0.0", "path": "packages/api/package.json"}],
    },
'''
text = text.replace(block_to_remove, '')

# Also remove assertions at the end? Let's just write the code out
with open("tests/test_release_artifacts.py", "w") as f:
    f.write(text)

