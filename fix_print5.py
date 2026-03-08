with open("src/lib.rs", "r") as f:
    lines = f.readlines()

new_struct = """                Event::UserEvent(UserEvent::Print(label)) => {
                    if let Some(window_id) = labels_to_id.get(&label) {
                        if let Some(runtime_window) = windows_by_id.get(window_id) {
                            let _ = runtime_window.webview.print();
                        }
                    }
                }
"""

for i in range(len(lines)):
    if "Event::UserEvent(UserEvent::UnregisterAllShortcuts" in lines[i]:
        insert_idx = i
        break

lines.insert(insert_idx, new_struct)
with open("src/lib.rs", "w") as f:
    f.writelines(lines)
