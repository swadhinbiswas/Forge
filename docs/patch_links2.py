import re
import glob

for filename in glob.glob('src/content/docs/**/*.md*', recursive=True):
    with open(filename, 'r') as f:
        content = f.read()

    new_content = re.sub(r'\]\(([^)]+)\.md\)', r'](\1/)', content)

    if new_content != content:
        with open(filename, 'w') as f:
            f.write(new_content)
