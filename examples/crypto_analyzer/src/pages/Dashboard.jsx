import { useState, useEffect } from 'react';
import { invoke, onEvent, removeEvent } from '../forge-api';
import { ArrowUpRight, ArrowDownRight, TrendingUp, Activity } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';
import { Badge } from '../components/ui/badge';
import { 
  AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer 
} from 'recharts';

export default function Dashboard() {
  const [marketData, setMarketData] = useState({});
  const [loading, setLoading] = useState(true);

  const [historicalData, setHistoricalData] = useState([]);

  useEffect(() => {
    // Generate some mock historical data for the chart to showcase beautiful recharts ui
    const generateMockData = () => {
      const data = [];
      let currentPrice = 65000;
      for (let i = 0; i < 24; i++) {
        currentPrice = currentPrice + (Math.random() - 0.45) * 1000;
        data.push({
          time: `${i}:00`,
          price: currentPrice
        });
      }
      return data;
    };
    setHistoricalData(generateMockData());

    const fetchMarket = async () => {
      try {
        const data = await invoke('get_market_snapshot');
        if (data && !data.error) {
          setMarketData(data);
        }
      } catch (err) {
        console.error("Failed to fetch market snapshot:", err);
      } finally {
        setLoading(false);
      }
    };
    
    fetchMarket();

    const handleUpdate = (updatedData) => {
      if (updatedData && typeof updatedData === 'object') {
        setMarketData(prev => ({...prev, ...updatedData}));
        
        // Also add a slight wobble to historical data for live effect
        setHistoricalData(prev => {
          const newData = [...prev];
          const lastPrice = newData[newData.length - 1].price;
          newData.shift();
          newData.push({
            time: 'Now',
            price: lastPrice + (Math.random() - 0.5) * 200
          });
          return newData;
        });
      }
    };
    
    onEvent('market-update', handleUpdate);
    return () => removeEvent('market-update', handleUpdate);
  }, []);

  return (
    <div className="space-y-6">
      <header className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-3xl font-bold tracking-tight">Market Dashboard</h2>
          <p className="text-muted-foreground">Real-time crypto market analytics</p>
        </div>
        <Badge variant="outline" className="flex items-center gap-2 px-3 py-1 bg-secondary/50">
          <span className="relative flex h-3 w-3">
            <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
            <span className="relative inline-flex rounded-full h-3 w-3 bg-green-500"></span>
          </span>
          Live Updates Active
        </Badge>
      </header>

      {loading ? (
        <div className="animate-pulse space-y-4">
          <div className="h-[120px] bg-muted rounded-xl" />
          <div className="h-[400px] bg-muted rounded-xl" />
        </div>
      ) : (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {Object.entries(marketData).map(([coin, data]) => {
              const isPositive = data.usd_24h_change >= 0;
              return (
                <Card key={coin} className="overflow-hidden group hover:border-primary/50 transition-colors">
                  <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                    <CardTitle className="text-lg font-medium capitalize flex items-center gap-2">
                      <Activity className="h-4 w-4 text-muted-foreground group-hover:text-primary transition-colors" />
                      {coin}
                    </CardTitle>
                    <Badge variant={isPositive ? "success" : "danger"} className="flex gap-1 items-center">
                      {isPositive ? <ArrowUpRight className="h-3 w-3" /> : <ArrowDownRight className="h-3 w-3" />}
                      {Math.abs(data.usd_24h_change || 0).toFixed(2)}%
                    </Badge>
                  </CardHeader>
                  <CardContent>
                    <div className="text-3xl font-bold tracking-tight">
                      ${data.usd?.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 6 })}
                    </div>
                  </CardContent>
                </Card>
              );
            })}
          </div>

          <Card className="border-border">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <TrendingUp className="h-5 w-5 text-primary" />
                Market Overview (24h)
              </CardTitle>
              <CardDescription>
                Live simulated mock view of market capitalization.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="h-[350px] w-full mt-4">
                <ResponsiveContainer width="100%" height="100%">
                  <AreaChart data={historicalData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                    <defs>
                      <linearGradient id="colorPrice" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="5%" stopColor="hsl(var(--primary))" stopOpacity={0.3}/>
                        <stop offset="95%" stopColor="hsl(var(--primary))" stopOpacity={0}/>
                      </linearGradient>
                    </defs>
                    <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" vertical={false} />
                    <XAxis 
                      dataKey="time" 
                      stroke="hsl(var(--muted-foreground))"
                      fontSize={12}
                      tickLine={false}
                      axisLine={false}
                    />
                    <YAxis 
                      stroke="hsl(var(--muted-foreground))"
                      fontSize={12}
                      tickLine={false}
                      axisLine={false}
                      tickFormatter={(value) => `$${value.toLocaleString()}`}
                    />
                    <Tooltip 
                      contentStyle={{ 
                        backgroundColor: 'hsl(var(--card))', 
                        borderColor: 'hsl(var(--border))',
                        borderRadius: '0.5rem',
                        boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
                      }}
                      itemStyle={{ color: 'hsl(var(--foreground))' }}
                      formatter={(value) => [`$${value.toFixed(2)}`, 'Price']}
                    />
                    <Area 
                      type="monotone" 
                      dataKey="price" 
                      stroke="hsl(var(--primary))" 
                      strokeWidth={2}
                      fillOpacity={1} 
                      fill="url(#colorPrice)" 
                    />
                  </AreaChart>
                </ResponsiveContainer>
              </div>
            </CardContent>
          </Card>
        </>
      )}
    </div>
  );
}
