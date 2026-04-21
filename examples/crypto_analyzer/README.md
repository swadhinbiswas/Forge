# Forge Crypto Analyzer

An enterprise-grade cryptocurrency market analysis tool, built to demonstrate the immense power of the **Forge Desktop Framework**. This application seamlessly blends a multi-threaded Python backend with a reactive, highly-styled React dashboard.

## 🌟 Key Features

- **Real-Time Data Streaming**: A Python background thread securely polls live prices from CoinGecko every 30 seconds and pipes them natively to the React frontend using zero-overhead Forge IPC Events (`app.emit(...)`).
- **Smart OS Notifications**: Detects >5% market swings and triggers native operating system notifications silently via Python without any frontend interaction.
- **Persistent Portfolio Management**: A lightweight database wrapper exposing CRUD operations to React strictly through synchronous & asynchronous Remote Procedure Calls (RPC/IPC).
- **Pro-Grade Analytics Dashboard**: Beautiful UI utilizing `shadcn/ui` components and the `recharts` library for deep, dark-mode SVG gradient data visualizations that react dynamically to CSS custom variables in Tailwind.
- **Custom IPC Wrapper**: Implementation of the `forge-api.js` abstraction pattern, proving how to securely bind complex desktop runtime endpoints to standard React hooks.

## 🛠 Tech Stack

### Backend (Python Core)
- **Forge Framework App**: Initializes the window, permissions, plugins, and background thread logic.
- **Python 3.14+ (Free-Threaded)**: Takes advantage of true multi-threading for uninterrupted API polling.
- **Requests**: Handing HTTP IO with CoinGecko's external APIs.

### Frontend (Desktop View)
- **Vite + React (JSX)**: Lightning-fast hot-reloading desktop development.
- **Shadcn UI & TailwindCSS**: Professional, accessible, atomic class styling with CSS Variables binding.
- **Recharts**: Modular SVG charting tied into responsive dashboard grids.
- **React Router DOM**: Client-side single-page architecture switching (Dashboard, Portfolio, Market Data).

## 📂 Architecture Breakdown

The project follows a scalable, decoupled architecture perfect for larger apps:

```text
crypto_analyzer/
├── forge.toml               # Engine config (port 5205, window size, entry)
├── package.json             # Node deps (Vite, React, Recharts, Tailwind)
├── tailwind.config.js       # Unified theme config mirroring shadcn
└── src/
    ├── main.py              # Application Entry / Initializer
    ├── api.py               # @app.command endpoints (Portfolio, Snapshots)
    ├── background.py        # Independent polling thread & OS Notifications
    ├── db.py                # Local SQLite persistence engine
    ├── App.jsx              # Main React Router
    ├── forge-api.js         # The critical bridge wrapping `window.__forge__`
    ├── index.html           # Injects the Forge runtime script globally
    ├── components/          # Reusable UI (Cards, Badges, Tables, Inputs)
    └── pages/               # Functional views (Dashboard, Portfolio, CoinData)
```

## 🚀 Getting Started

Ensure you have your global Python Environment and `uv` setup within the monorepo. Then simply:

1. **Install Frontend Dependencies**
   ```bash
   npm install
   ```

2. **Boot the Developer Environment**
   Launch both the Vite server (port 5205) and the Python runtime securely using `uv`:
   ```bash
   uv run python -m forge dev
   ```

3. **Enjoy Hot-Reloading**
   Modify any Python file (`.py`) to see the backend automatically restart (`os.execv` strategy), or modify any `.jsx` file to see Vite hot-swap the interface natively without losing state.

## 🧠 Masterclass in IPC (Inter-Process Communication)

### 1. RPC Commands (Frontend -> Backend)
Instead of creating cumbersome REST APIs, React securely requests data using decorators in Python:

**Backend (`src/api.py`):**
```python
@app.command("get_portfolio")
def get_portfolio():
    return db.get_portfolio_items()
```

**Frontend (`src/forge-api.js` wrapper):**
```javascript
// Safely checks context before invocation
export const invoke = async (command, payload = {}) => {
  if (window.__forge__) return await window.__forge__.invoke(command, payload);
  console.warn(`Forge runtime not found. Mocking: ${command}`);
  return null;
};
```

### 2. Live Events (Backend -> Frontend)
To stream data to the dashboard without React spam-polling, the Python thread pipes events downward:

**Backend Sub-Thread (`src/background.py`):**
```python
def poll_markets(app):
    while True:
        data = fetch_snapshot()
        app.emit("market-update", data) # Fires to React
        time.sleep(30)
```

**Frontend Hook (`src/pages/Dashboard.jsx`):**
```javascript
useEffect(() => {
  const unsubscribe = window.__forge__?.on("market-update", (data) => {
    setMarketPrices(data.payload);
  });
  return () => unsubscribe?.();
}, []);
```

## 🔐 Security Context
This demo explicitly maps Vite's port via the `forge.toml` (`dev_server_url = "http://127.0.0.1:5205"`). Forge strictly validates origins across IPC calls, ensuring that standard browser tabs cannot connect into your system-level backend. All OS operations (like Notifications) execute natively out-of-band in the Python runtime.
