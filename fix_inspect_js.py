with open("forge/js/forge.js", "r") as f:
    text = f.read()

import re
text = re.sub(
    r'(\s*if \(window\.__FORGE_INSPECT__\) {\s*console\.debug\("\[IPC REQ\]", jsonStr\);\s*}){2,}',
    r'\n        if (window.__FORGE_INSPECT__) {\n            console.debug("[IPC REQ]", jsonStr);\n        }',
    text
)

text = re.sub(
    r'(\s*if \(window\.__FORGE_INSPECT__\) {\s*console\.debug\("\[IPC RES\]", msg\);\s*}){2,}',
    r'\n        if (window.__FORGE_INSPECT__) {\n            console.debug("[IPC RES]", msg);\n        }',
    text
)

with open("forge/js/forge.js", "w") as f:
    f.write(text)
