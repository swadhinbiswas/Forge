import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'
import forge from '../forge-api'

export default function Dashboard({ token }) {
  const [data, setData] = useState(null)
  const navigate = useNavigate()

  useEffect(() => {
    forge.get_dashboard_data().then(setData)
  }, [])

  if (!data) return <div className="loading">Loading Dashboard...</div>

  const heroImage = data.hero?.backdrop_path ? `https://image.tmdb.org/t/p/original${data.hero.backdrop_path}` : ''

  return (
    <div>
      {data.hero && (
        <div className="hero" style={{ backgroundImage: `linear-gradient(to right, #0f0f11 20%, rgba(15,15,17,0)), url(${heroImage})` }}>
          <div className="hero-content">
            <h1>{data.hero.title}</h1>
            <div className="hero-meta">
              <span className="badge rating">IMDB {data.hero.vote_average.toFixed(1)}</span>
            </div>
            <p>{data.hero.overview}</p>
            <div className="hero-actions" style={{marginTop: 20}}>
              <button className="btn-primary" onClick={() => navigate(`/movie/${data.hero.id}`)}>View Details</button>
            </div>
          </div>
        </div>
      )}

      <div className="section-header"><h2>Popular Movies</h2></div>
      <div className="horizontal-list">
        <div className="list-container continue-watching">
          {data.popular?.slice(0, 10).map(m => (
            <div key={m.id} className="movie-card" onClick={() => navigate(`/movie/${m.id}`)} style={{cursor: 'pointer'}}>
              <img src={`https://image.tmdb.org/t/p/w500${m.poster_path}`} alt={m.title} />
            </div>
          ))}
        </div>
      </div>
      
      <div className="section-header"><h2>Top Rated</h2></div>
      <div className="horizontal-list">
        <div className="list-container continue-watching">
          {data.continue_watching?.map(m => (
            <div key={m.id} className="movie-card" onClick={() => navigate(`/movie/${m.id}`)} style={{cursor: 'pointer'}}>
              <img src={`https://image.tmdb.org/t/p/w500${m.poster_path}`} alt={m.title} />
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
