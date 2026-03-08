with open("src/lib.rs", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "UserEvent::RegisterShortcut" in line:
        start_idx = i
        break

for i in range(start_idx, len(lines)):
    if "UserEvent::UnregisterShortcut" in lines[i]:
        mid_idx = i
        break

for i in range(mid_idx, len(lines)):
    if "UserEvent::UnregisterAllShortcuts" in lines[i]:
        end_idx = i
        break

new_register = """                Event::UserEvent(UserEvent::RegisterShortcut(accelerator, tx)) => {
                    use std::str::FromStr;
                    match global_hotkey::hotkey::HotKey::from_str(&accelerator) {
                        Ok(hotkey) => {
                            if hotkey_manager.register(hotkey).is_ok() {
                                registered_hotkeys.insert(accelerator.clone(), hotkey);
                                hotkey_id_to_string.insert(hotkey.id(), accelerator.clone());
                                let _ = tx.send(true);
                            } else {
                                let _ = tx.send(false);
                            }
                        }
                        Err(_) => {
                            let _ = tx.send(false);
                        }
                    }
                }
"""

new_unregister = """                Event::UserEvent(UserEvent::UnregisterShortcut(accelerator, tx)) => {
                    if let Some(hotkey) = registered_hotkeys.remove(&accelerator) {
                        hotkey_id_to_string.remove(&hotkey.id());
                        let _ = hotkey_manager.unregister(hotkey);
                        let _ = tx.send(true);
                    } else {
                        let _ = tx.send(false);
                    }
                }
"""

lines = lines[:start_idx] + [new_register, new_unregister] + lines[end_idx:]

with open("src/lib.rs", "w") as f:
    f.writelines(lines)
