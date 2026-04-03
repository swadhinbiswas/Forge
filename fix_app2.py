with open("forge/app.py", "r") as f:
    text = f.read()
text = text.replace("OsIntegrationAPI", "OSIntegrationAPI")
with open("forge/app.py", "w") as f:
    f.write(text)
