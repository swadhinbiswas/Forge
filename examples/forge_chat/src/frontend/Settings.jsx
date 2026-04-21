import { useState, useEffect } from 'react'

export default function Settings() {
  const [theme, setTheme] = useState('dark')
  
  useEffect(() => {
    document.body.className = `theme-${theme}`
  }, [theme])

  const handleThemeChange = async (newTheme) => {
    setTheme(newTheme)
    try {
      // Broadcast this config change to all windows via Forge IPC!
      await window.__forge__.window.broadcast('theme:changed', { theme: newTheme })
    } catch (e) {
      console.error("Failed to broadcast", e)
    }
  }

  return (
    <div className="settings-container">
      <h2>App Settings</h2>
      
      <div className="setting-row">
        <label>Global Theme</label>
        <select value={theme} onChange={e => handleThemeChange(e.target.value)}>
          <option value="dark">Dark Mode</option>
          <option value="light">Light Mode</option>
          <option value="cyberpunk">Cyberpunk</option>
        </select>
      </div>

      <div className="setting-footer">
        <p>This is a secondary window spawned natively via Forge IPC.</p>
        <button onClick={() => window.close()}>Close Settings</button>
      </div>
    </div>
  )
}
