with open('/home/swadhin/Forge/forge-framework/missing.md', 'r') as f:
    text = f.read()

text = text.replace(
    '### 9. Hardware & Device Interfaces (WebUSB/WebSerial emulation or Native pass-through)\n- **Missing feature:**',
    '### 9. Hardware & Device Interfaces (WebUSB/WebSerial emulation or Native pass-through)\n✅ **STATUS: IMPLEMENTED**\n- **Feature integrated via pyserial proxy:**'
)
with open('/home/swadhin/Forge/forge-framework/missing.md', 'w') as f:
    f.write(text)
