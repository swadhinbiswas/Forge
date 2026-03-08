with open("src/lib.rs", "r") as f:
    text = f.read()

text = text.replace(
"""                        Err(_) => {
                            let _ = tx.send(false);
                    }
                }
                Event::UserEvent(UserEvent::UnregisterAllShortcuts(tx)) => {""",
"""                        Err(_) => {
                            let _ = tx.send(false);
                        }
                    }
                }
                Event::UserEvent(UserEvent::UnregisterAllShortcuts(tx)) => {"""
)
with open("src/lib.rs", "w") as f:
    f.write(text)
