import { useState } from 'react'
import forge from '../forge-api'

export default function Login({ onLogin }) {
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')

  const submit = async (e) => {
    e.preventDefault()
    const res = await forge.login(username, password)
    if (res.error) alert(res.error)
    else onLogin(res.token)
  }

  return (
    <div className="setup-container">
      <div className="setup-box">
        <h2>Login or Sign Up</h2>
        <form onSubmit={submit}>
          <input placeholder="Username" value={username} onChange={e => setUsername(e.target.value)} required />
          <input type="password" placeholder="Password" value={password} onChange={e => setPassword(e.target.value)} required />
          <button type="submit">Login</button>
        </form>
      </div>
    </div>
  )
}
