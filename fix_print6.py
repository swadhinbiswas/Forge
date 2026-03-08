with open("src/lib.rs", "r") as f:
    text = f.read()

start = text.find("Event::UserEvent(UserEvent::RegisterShortcut(accelerator, tx)) => {")
end = text.find("Event::UserEvent(UserEvent::Print(label)) => {")

new_text = """Event::UserEvent(UserEvent::RegisterShortcut(accelerator, tx)) => {
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
                Event::UserEvent(UserEvent::UnregisterShortcut(accelerator, tx)) => {
                    if let Some(hotkey) = registered_hotkeys.remove(&accelerator) {
                        hotkey_id_to_string.remove(&hotkey.id());
                        let _ = hotkey_manager.unregister(hotkey);
                        let _ = tx.send(true);
                    } else {
                        let _ = tx.send(false);
                    }
                }
                """

text = text[:start] + new_text + text[end:]

with open("src/lib.rs", "w") as f:
    f.write(text)
