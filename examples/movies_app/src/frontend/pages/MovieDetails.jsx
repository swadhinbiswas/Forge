import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import forge from '../forge-api'

export default function MovieDetails({ token }) {
  const { id } = useParams()
  const navigate = useNavigate()
  const [movie, setMovie] = useState(null)
  const [inWatchlist, setInWatchlist] = useState(false)

  useEffect(() => {
    forge.get_movie_details(parseInt(id)).then(m => {
      if(!m.error) setMovie(m)
    })
    
    forge.get_watchlist(token).then(list => {
      if (list.find(item => item.id == id)) setInWatchlist(true)
    })
  }, [id, token])

  const toggleWatchlist = async () => {
    const result = await forge.toggle_watchlist(token, movie)
    setInWatchlist(result)
  }

  if (!movie) return <div className="loading">Loading movie details...</div>
  const heroImage = movie.backdrop_path ? `https://image.tmdb.org/t/p/original${movie.backdrop_path}` : ''

  return (
    <div>
       <button onClick={() => navigate(-1)} style={{background: 'none', border: 'none', color: '#fff', cursor: 'pointer', marginBottom: 20}}>← Back</button>
       <div className="hero" style={{ backgroundImage: `linear-gradient(to top, #0f0f11 5%, rgba(15,15,17,0.4)), url(${heroImage})`, minHeight: 450 }}>
          <div className="hero-content" style={{alignSelf: 'flex-end', maxWidth: '70%'}}>
            <h1 style={{fontSize: '36px'}}>{movie.title}</h1>
            <div className="hero-meta">
              <span className="badge rating">⭐ {movie.vote_average?.toFixed(1)}</span>
              <span className="badge lang">{movie.release_date?.substring(0, 4)}</span>
            </div>
            <p>{movie.overview}</p>
            <div className="hero-actions" style={{marginTop: 20}}>
              <button className="btn-primary" onClick={toggleWatchlist}>
                {inWatchlist ? '- Remove from Watchlist' : '+ Add to Watchlist'}
              </button>
            </div>
          </div>
        </div>

        <div className="section-header" style={{marginTop: 40}}><h2>Cast</h2></div>
        <div className="horizontal-list">
          <div className="list-container party-card-list" style={{display: 'flex', gap: 15, overflowX: 'auto'}}>
            {movie.credits?.cast?.slice(0, 10).map(actor => (
              <div key={actor.cast_id} style={{minWidth: 120, textAlign: 'center'}}>
                <img 
                  src={actor.profile_path ? `https://image.tmdb.org/t/p/w200${actor.profile_path}` : 'https://via.placeholder.com/120x180?text=No+Image'} 
                  alt={actor.name} 
                  style={{width: 100, height: 150, objectFit: 'cover', borderRadius: 8, marginBottom: 10}} 
                />
                <h4 style={{fontSize: 14, margin: '0 0 4px 0'}}>{actor.name}</h4>
                <p style={{fontSize: 12, color: '#888', margin: 0}}>{actor.character}</p>
              </div>
            ))}
          </div>
        </div>
    </div>
  )
}
