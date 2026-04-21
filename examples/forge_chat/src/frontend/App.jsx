import { useState, useEffect, useRef } from 'react'

export default function App() {
  const [messages, setMessages] = useState([])
  const [inputVal, setInputVal] = useState('')
  const [connId, setConnId] = useState(null)
  const [downloads, setDownloads] = useState({})
  
  useEffect(() => {
    // 1. Listen for cross-window events (e.g. from Settings window)
    const handleTheme = (e) => {
      document.body.className = `theme-${e.theme}`
    };
    window.__forge__.on('theme:changed', handleTheme)

    // 2. Center window natively just to show off Positioner API
    const centerWindow = async () => {
      try {
        await window.__forge__.positioner.center()
      } catch (e) {
        console.warn("Could not center window", e)
      }
    }
    centerWindow()

    return () => {
      window.__forge__.off('theme:changed', handleTheme)
    }
  }, [])

  // Listen to WebSocket messages securely via Forge generic event bus
  useEffect(() => {
    const handleMsg = (payload) => {
      setMessages(prev => [...prev, { text: payload.data, id: Date.now(), isSelf: false }])
    };
    if (connId) {
      window.__forge__.on(`ws:message:${connId}`, handleMsg)
    }
    return () => {
      if (connId) {
        window.__forge__.off(`ws:message:${connId}`, handleMsg)
      }
    }
  }, [connId])

  const connectToWebSocket = async () => {
    if (connId) {
      await window.__forge__.ws.close(connId)
      setConnId(null)
      return
    }
    try {
      const res = await window.__forge__.ws.connect('wss://echo.websocket.events')
      setConnId(res.connection_id)
      setMessages(prev => [...prev, { text: "Connected to echo server!", id: Date.now(), isSystem: true }])
    } catch (e) {
      setMessages(prev => [...prev, { text: `WS Error: ${e.message}`, id: Date.now(), isSystem: true }])
    }
  }

  const sendMessage = async (e) => {
    e.preventDefault()
    if (!inputVal.trim() || !connId) return

    setMessages(prev => [...prev, { text: inputVal, id: Date.now(), isSelf: true }])
    await window.__forge__.ws.send(connId, inputVal)
    setInputVal('')
  }
  
  const simulateDownload = async () => {
    const filename = `file_${Math.floor(Math.random()*1000)}.bin`
    try {
      // Start background task in Python
      const res = await window.__forge__.invoke('simulate_file_download', { filename })
      
      // Wait for Python to broadcast its stream channel ID
      const unlistenStarted = await window.__forge__.on('download:started', async (evt) => {
        if (evt.payload.filename === filename) {
          const streamId = evt.payload.channel_id
          
          setDownloads(prev => ({...prev, [filename]: 0}))
          
          // Listen on the dedicated channel for progress dots
          await window.__forge__.channel.on(streamId, (chunk) => {
            setDownloads(prev => ({...prev, [filename]: chunk.percent}))
          })
        }
      })
      
    } catch (exc) {
       console.error("Task error", exc)
    }
  }

  const openSettings = async () => {
    await window.__forge__.invoke("open_settings")
  }

  const revealFiles = async () => {
    await window.__forge__.opener.openPath(".")
  }

  return (
    <div className="app-layout">
      <aside className="sidebar.glass-panel">
        <h1>ForgeChat</h1>
        <div className="status-indicator">
           <span className={connId ? 'online' : 'offline'}></span>
           {connId ? 'Connected' : 'Offline'}
        </div>
        
        <button onClick={connectToWebSocket}>
          {connId ? 'Disconnect WS' : 'Connect to Echo Server'}
        </button>
        
        <button onClick={simulateDownload}>
          Simulate Background Download
        </button>
        
        <button className="secondary" onClick={openSettings}>Open Settings</button>
        
        <div className="downloads">
          {Object.entries(downloads).map(([file, pct]) => (
            <div key={file} className="dl-item">
              <span>{file}</span>
              <div className="progress-bar">
                 <div className="progress-fill" style={{ width: `${pct}%` }}></div>
              </div>
            </div>
          ))}
        </div>
      </aside>
      
      <main className="chat-area.glass-panel">
        <div className="msg-list">
          {messages.map(m => (
            <div key={m.id} className={`msg ${m.isSystem ? 'system' : m.isSelf ? 'self' : 'remote'}`}>
              <div className="msg-bubble">{m.text}</div>
            </div>
          ))}
        </div>
        
        <form onSubmit={sendMessage} className="chat-input-bar">
           <input 
             disabled={!connId}
             value={inputVal}
             onChange={e => setInputVal(e.target.value)}
             placeholder={connId ? "Type a message..." : "Connect first"} 
           />
           <button type="submit" disabled={!connId}>Send</button>
        </form>
      </main>
    </div>
  )
}
