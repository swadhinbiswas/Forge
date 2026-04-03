with open("tests/test_release_artifacts.py", "r") as f:
    text = f.read()

# find "notarization": {"status": "skipped"}, and inject provenance and version_alignment
text = text.replace(
    '"notarization": {"status": "skipped"},',
    '"notarization": {"status": "skipped"},\n            "provenance": {"workspace_root": str(workspace), "source_commit": "abc123"},\n            "version_alignment": {"aligned": True},'
)

with open("tests/test_release_artifacts.py", "w") as f:
    f.write(text)
