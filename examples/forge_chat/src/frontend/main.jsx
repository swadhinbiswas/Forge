import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App'
import Settings from './Settings'
import './index.css'

const isSettings = window.location.hash.includes('/settings')

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    {isSettings ? <Settings /> : <App />}
  </React.StrictMode>,
)
