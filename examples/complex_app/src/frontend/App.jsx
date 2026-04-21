import { useState } from 'react'
import forge, { invoke } from '@forgedesk/api'
import './App.css'

function App() {
  const [name, setName] = useState('')
  const [greeting, setGreeting] = useState('')
  const [systemInfo, setSystemInfo] = useState(null)
  const [loadingGreet, setLoadingGreet] = useState(false)
  const [loadingInfo, setLoadingInfo] = useState(false)
  const [errorGreet, setErrorGreet] = useState(null)
  const [errorInfo, setErrorInfo] = useState(null)

  const handleGreet = async () => {
    setLoadingGreet(true)
    setErrorGreet(null)
    setGreeting('')
    try {
      const result = await invoke('greet', { name: name || 'Developer' })
      setGreeting(result)
      try { await forge.clipboard.write(result); } catch(e) {}
    } catch (err) {
      setErrorGreet(err.message || String(err))
    } finally {
      setLoadingGreet(false)
    }
  }

  const handleGetInfo = async () => {
    setLoadingInfo(true)
    setErrorInfo(null)
    setSystemInfo(null)
    try {
      const info = await invoke('get_system_info')
      setSystemInfo(info)
    } catch (err) {
      setErrorInfo(err.message || String(err))
    } finally {
      setLoadingInfo(false)
    }
  }

  return (
    <>
      <div id="noise"></div>
      <div className="glow-bg"></div>

      <div className="container">
        <header>
          <div className="logo-container">
            <div className="logo">⚡</div>
          </div>
          <h1>examples/complex_app</h1>
          <p className="tagline">Welcome to the future of Desktop Apps mapped with Python.</p>
        </header>

        <main>
          <section className="card glass">
            <div className="card-header">
              <div className="dot-group">
                <span className="dot red"></span>
                <span className="dot yellow"></span>
                <span className="dot green"></span>
              </div>
              <h2>Command Execution (Complex Demo)</h2>
            </div>
            <div className="card-body">
              <p className="desc">Enter a name to call the <code>greet</code> python command via IPC.</p>
              <div className="input-group">
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="Enter your name..."
                  autoComplete="off"
                />
                <button onClick={handleGreet} disabled={loadingGreet}>
                  {loadingGreet ? 'Running...' : 'Run'}
                </button>
              </div>
              {greeting && <div className="output-box success show">{greeting}</div>}
              {errorGreet && <div className="output-box error show">{errorGreet}</div>}
            </div>
          </section>

          <section className="card glass">
            <div className="card-header">
              <div className="dot-group">
                <span className="dot red"></span>
                <span className="dot yellow"></span>
                <span className="dot green"></span>
              </div>
              <h2>System Telemetry & Events</h2>
            </div>
            <div className="card-body">
              <p className="desc">Fetching live diagnostics seamlessly from Python.</p>
              <button className="secondary-btn" onClick={handleGetInfo} disabled={loadingInfo}>
                {loadingInfo ? 'Fetching...' : 'Fetch System Info'}
              </button>
              {systemInfo && (
                <div className="output-box info show">
                  {JSON.stringify(systemInfo, null, 2)}
                </div>
              )}
              {errorInfo && <div className="output-box error show">{errorInfo}</div>}
            </div>
          </section>
        </main>

        <footer>
          <p>Powered by <strong>Forge Framework</strong> • <em>Complex App Template</em></p>
        </footer>
      </div>
    </>
  )
}

export default App
