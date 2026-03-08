with open("src/lib.rs", "r") as f:
    text = f.read()

import re
text = re.sub(r'                Event::UserEvent\(UserEvent::Print\(label\)\) => \{\n                    if let Some\(window_id\) = labels_to_id\.get\(&label\) \{\n                        if let Some\(runtime_window\) = windows_by_id\.get\(window_id\) \{\n                            let _ = runtime_window\.webview\.print\(\);\n                        \}\n                    \}\n+', '', text)

text = text.replace(
"""                Event::UserEvent(UserEvent::UnregisterAllShortcuts(tx)) => {
                    let _ = hotkey_manager.unregister_all(&registered_hotkeys.values().cloned().collect::<Vec<_>>());
                    registered_hotkeys.clear();
                    hotkey_id_to_string.clear();
                    let _ = tx.send(true);
                }""",
"""                Event::UserEvent(UserEvent::UnregisterAllShortcuts(tx)) => {
                    let _ = hotkey_manager.unregister_all(&registered_hotkeys.values().cloned().collect::<Vec<_>>());
                    registered_hotkeys.clear();
                    hotkey_id_to_string.clear();
                    let _ = tx.send(true);
                }
                Event::UserEvent(UserEvent::Print(label)) => {
                    if let Some(window_id) = labels_to_id.get(&label) {
                        if let Some(runtime_window) = windows_by_id.get(window_id) {
                            let _ = runtime_window.webview.print();
                        }
                    }
                }"""
)

with open("src/lib.rs", "w") as f:
    f.write(text)
