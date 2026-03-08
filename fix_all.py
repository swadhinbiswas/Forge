with open("src/lib.rs", "r") as f:
    text = f.read()

# Let's cleanly replace the WindowProxy and NativeWindow methods and the event loop
start_idx = text.find("impl WindowProxy {")
if start_idx == -1:
    print("WindowProxy not found")

end_idx = text.find("#[pyclass]\nstruct AutoLaunchManager {")
if end_idx == -1:
    print("AutoLaunchManager not found")

with open("fix_all_backup.rs", "w") as f:
    f.write(text)

