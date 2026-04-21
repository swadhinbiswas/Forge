import { BarChart, Bar, XAxis, YAxis, Tooltip, Legend, ResponsiveContainer, CartesianGrid } from 'recharts';
import { useState, useEffect } from 'react';
import { invoke } from '../forge-api';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';

export default function CoinData() {
  const [data, setData] = useState([]);

  useEffect(() => {
    const fetchMarket = async () => {
      try {
        const snapshot = await invoke('get_market_snapshot');
        if (snapshot && !snapshot.error) {
          const formatted = Object.entries(snapshot).map(([coin, info]) => ({
            name: coin.charAt(0).toUpperCase() + coin.slice(1),
            price: info.usd,
            change: info.usd_24h_change
          }));
          setData(formatted);
        }
      } catch (err) {
        console.error(err);
      }
    };
    fetchMarket();
  }, []);

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-3xl font-bold tracking-tight">Market Analysis</h2>
        <p className="text-muted-foreground">Compare 24-hour performance across cryptocurrencies</p>
      </div>
      
      <Card className="h-[600px] flex flex-col">
        <CardHeader>
          <CardTitle>24h Price Change (%)</CardTitle>
          <CardDescription>Visualizing market momentum and sentiment over the last day.</CardDescription>
        </CardHeader>
        <CardContent className="flex-1 min-h-0">
          {data.length > 0 ? (
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={data} margin={{ top: 20, right: 30, left: 0, bottom: 5 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="hsl(var(--border))" vertical={false} />
                <XAxis 
                  dataKey="name" 
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
                  tickFormatter={(value) => `${value}%`}
                />
                <Tooltip 
                  cursor={{fill: 'hsl(var(--muted))', opacity: 0.4}} 
                  contentStyle={{ 
                    backgroundColor: 'hsl(var(--card))', 
                    borderColor: 'hsl(var(--border))', 
                    borderRadius: '0.5rem',
                    color: 'hsl(var(--foreground))',
                    boxShadow: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)'
                  }}
                  itemStyle={{ color: 'hsl(var(--foreground))' }}
                />
                <Legend iconType="circle" />
                <Bar 
                  dataKey="change" 
                  name="24h Change (%)"
                  fill={(entry) => (entry.change >= 0 ? "hsl(var(--primary))" : "hsl(var(--destructive))")} 
                  radius={[4, 4, 0, 0]}
                  barSize={40}
                />
              </BarChart>
            </ResponsiveContainer>
          ) : (
            <div className="h-full flex items-center justify-center text-muted-foreground animate-pulse">
              Loading chart data...
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
