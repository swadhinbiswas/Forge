import { HashRouter as Router, Routes, Route, Link } from 'react-router-dom';
import { LayoutDashboard, Coins, Briefcase } from 'lucide-react';
import Dashboard from './pages/Dashboard';
import CoinData from './pages/CoinData';
import Portfolio from './pages/Portfolio';

function App() {
  return (
    <Router>
      <div className="flex h-screen bg-gray-900 text-white">
        {/* Sidebar */}
        <aside className="w-64 bg-gray-800 border-r border-gray-700 flex flex-col">
          <div className="p-6">
            <h1 className="text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">Crypto Analyzer</h1>
          </div>
          <nav className="flex-1 px-4 space-y-2">
            <Link to="/" className="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-gray-700 transition-colors">
              <LayoutDashboard size={20} />
              <span>Dashboard</span>
            </Link>
            <Link to="/coins" className="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-gray-700 transition-colors">
              <Coins size={20} />
              <span>Coin Data</span>
            </Link>
            <Link to="/portfolio" className="flex items-center space-x-3 px-4 py-3 rounded-lg hover:bg-gray-700 transition-colors">
              <Briefcase size={20} />
              <span>Portfolio</span>
            </Link>
          </nav>
        </aside>

        {/* Main Content */}
        <main className="flex-1 overflow-y-auto p-8">
          <Routes>
            <Route path="/" element={<Dashboard />} />
            <Route path="/coins" element={<CoinData />} />
            <Route path="/portfolio" element={<Portfolio />} />
          </Routes>
        </main>
      </div>
    </Router>
  );
}

export default App;