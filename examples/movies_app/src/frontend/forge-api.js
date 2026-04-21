function ensureForgeRuntime() {
  if (!window.__forge__) {
    throw new Error('Forge runtime is not initialized. Ensure forge.js is loaded before app startup.')
  }
  return window.__forge__
}

async function invoke(cmd, args = {}) {
  const runtime = ensureForgeRuntime()
  return runtime.invoke(cmd, args)
}

const forgeApi = {
  get_setting(key) {
    return invoke('get_setting', { key })
  },
  save_setting(key, value) {
    return invoke('save_setting', { key, value })
  },
  check_api_key(key) {
    return invoke('check_api_key', { key })
  },
  login(username, password) {
    return invoke('login', { username, password })
  },
  get_dashboard_data() {
    return invoke('get_dashboard_data')
  },
  get_movie_details(movie_id) {
    return invoke('get_movie_details', { movie_id })
  },
  toggle_watchlist(token, movie) {
    return invoke('toggle_watchlist', { token, movie })
  },
  get_watchlist(token) {
    return invoke('get_watchlist', { token })
  }
}

export default forgeApi
