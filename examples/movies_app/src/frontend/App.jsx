import { useState, useEffect } from 'react'
import { Routes, Route, useNavigate, Navigate } from 'react-router-dom'
import forge from './forge-api'
import './App.css'

import Login from './pages/Login'
import Dashboard from './pages/Dashboard'
import MovieDetails from './pages/MovieDetails'
import Watchlist from './pages/Watchlist'

function App() {
  const [token, setToken] = useState(localStorage.getItem('token') || null)
  const [tmdbKey, setTmdbKey] = useState('')
  const [keyValid, setKeyValid] = useState(false)
  const [setupDone, setSetupDone] = useState(false)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    async function init() {
      const savedKey = await forge.get_setting("tmdb_key")
      if (savedKey) {
        const valid = await forge.check_api_key(savedKey)
        if (valid) {
          setTmdbKey(savedKey)
          setKeyValid(true)
          setSetupDone(true)
        }
      }
      setLoading(false)
    }
    init()
  }, [])

  const handleSetup = async () => {
    const valid = await forge.check_api_key(tmdbKey)
    if (valid) {
      await forge.save_setting("tmdb_key", tmdbKey)
      setKeyValid(true)
      setSetupDone(true)
    } else {
      alert("Invalid TMDB Key")
    }
  }

  const handleLogin = (newToken) => {
    localStorage.setItem('token', newToken)
    setToken(newToken)
  }

  const handleLogout = () => {
    localStorage.removeItem('token')
    setToken(null)
  }

  if (loading) return <div className="loading">Loading...</div>

  if (!setupDone) {
    return (
      <div className="setup-container">
        <div className="setup-box">
          <h2>Configure TMDB</h2>
          <p>Please enter your TMDB API Key</p>
          <input 
            type="text" 
            value={tmdbKey} 
            onChange={e => setTmdbKey(e.target.value)} 
            placeholder="TMDB API Key"
          />
          <button onClick={handleSetup}>Save & Continue</button>
        </div>
      </div>
    )
  }

  return (
    <div className="app-container">
      {token && (
        <div className="sidebar">
          <div className="logo"><span>Forge</span>Movies</div>
          <div className="menu-group">
            <h4>Menu</h4>
            <ul>
              <li onClick={() => window.location.href = '/'}>Home</li>
              <li onClick={() => window.location.href = '/watchlist'}>Watchlist</li>
            </ul>
          </div>
          <div className="logout">
            <button onClick={handleLogout}>Log out</button>
          </div>
        </div>
      )}
      <div className="main-content">
        <Routes>
          <Route path="/login" element={!token ? <Login onLogin={handleLogin} /> : <Navigate to="/" />} />
          <Route path="/" element={token ? <Dashboard token={token} /> : <Navigate to="/login" />} />
          <Route path="/movie/:id" element={token ? <MovieDetails token={token} /> : <Navigate to="/login" />} />
          <Route path="/watchlist" element={token ? <Watchlist token={token} /> : <Navigate to="/login" />} />
        </Routes>
      </div>
    </div>
  )
}

export default App
