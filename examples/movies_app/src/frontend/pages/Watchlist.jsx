import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import forge from '../forge-api'

export default function Watchlist({ token }) {
  const [watchlist, setWatchlist] = useState([])
  const navigate = useNavigate()

  useEffect(() => {
    forge.get_watchlist(token).then(setWatchlist)
  }, [token])

  return (
    <div>
      <h1 style={{marginBottom: 30}}>My Watchlist</h1>
      {watchlist.length === 0 ? <p>Your watchlist is empty.</p> : (
        <div style={{display: 'flex', flexWrap: 'wrap', gap: 20}}>
          {watchlist.map(m => (
            <div key={m.id} onClick={() => navigate(`/movie/${m.id}`)} style={{width: 180, cursor: 'pointer', background: '#1c1c21', borderRadius: 12, overflow: 'hidden'}}>
              <img src={`https://image.tmdb.org/t/p/w300${m.poster_path}`} alt={m.title} style={{width: '100%', height: 270, objectFit: 'cover'}}/>
              <div style={{padding: 10}}>
                <h4 style={{margin: '0 0 5px 0', fontSize: 14, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis'}}>{m.title}</h4>
                <div style={{fontSize: 12, color: '#f5c518'}}>⭐ {m.vote_average?.toFixed(1)}</div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
