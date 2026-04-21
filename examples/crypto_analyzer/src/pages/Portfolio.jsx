import { useState, useEffect } from 'react';
import { invoke } from '../forge-api';
import { Plus, Trash2, Wallet, DollarSign } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '../components/ui/card';
import { Button } from '../components/ui/button';
import { Input } from '../components/ui/input';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '../components/ui/table';

export default function Portfolio() {
  const [items, setItems] = useState([]);
  const [symbol, setSymbol] = useState('');
  const [amount, setAmount] = useState('');
  const [buyPrice, setBuyPrice] = useState('');
  
  const totalValue = items.reduce((acc, item) => acc + (item.amount * item.buy_price), 0);

  const fetchPortfolio = async () => {
    const data = await invoke('get_portfolio');
    if (data) setItems(data);
  };

  useEffect(() => {
    fetchPortfolio();
  }, []);

  const handleAdd = async (e) => {
    e.preventDefault();
    if (!symbol || !amount || !buyPrice) return;
    
    await invoke('add_portfolio_item', {
      symbol: symbol.trim().toUpperCase(),
      amount: parseFloat(amount),
      buy_price: parseFloat(buyPrice)
    });
    
    setSymbol('');
    setAmount('');
    setBuyPrice('');
    fetchPortfolio();
  };

  const handleDelete = async (id) => {
    await invoke('remove_portfolio_item', { id });
    fetchPortfolio();
  };

  return (
    <div className="space-y-6">
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-3xl font-bold tracking-tight">My Portfolio</h2>
          <p className="text-muted-foreground">Manage and track your crypto investments</p>
        </div>
        <Card className="bg-primary text-primary-foreground py-2 px-4 shadow-md">
          <div className="text-sm font-medium opacity-80">Total Invested</div>
          <div className="text-2xl font-bold">${totalValue.toLocaleString(undefined, { minimumFractionDigits: 2 })}</div>
        </Card>
      </div>
      
      <Card>
        <CardHeader>
          <CardTitle>Add Asset</CardTitle>
          <CardDescription>Record a new purchase to your portfolio.</CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleAdd} className="flex flex-col sm:flex-row gap-4 items-end">
            <div className="flex-1 w-full space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Symbol
              </label>
              <div className="relative">
                <div className="absolute left-2.5 top-2.5 text-muted-foreground">
                  <Wallet size={16} />
                </div>
                <Input 
                  type="text" 
                  value={symbol} 
                  onChange={e => setSymbol(e.target.value)} 
                  placeholder="e.g. BTC" 
                  className="pl-9"
                />
              </div>
            </div>
            <div className="flex-1 w-full space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Amount
              </label>
              <Input 
                type="number" 
                step="any" 
                value={amount} 
                onChange={e => setAmount(e.target.value)} 
                placeholder="0.0" 
              />
            </div>
            <div className="flex-1 w-full space-y-2">
              <label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Buy Price (USD)
              </label>
              <div className="relative">
                <div className="absolute left-2.5 top-2.5 text-muted-foreground">
                  <DollarSign size={16} />
                </div>
                <Input 
                  type="number" 
                  step="any" 
                  value={buyPrice} 
                  onChange={e => setBuyPrice(e.target.value)} 
                  placeholder="0.0" 
                  className="pl-9"
                />
              </div>
            </div>
            <Button type="submit" className="w-full sm:w-auto flex items-center gap-2">
              <Plus size={18} /> Add
            </Button>
          </form>
        </CardContent>
      </Card>

      <Card className="overflow-hidden">
        <Table>
          <TableHeader>
            <TableRow className="bg-muted/50 hover:bg-muted/50">
              <TableHead className="font-semibold">Asset</TableHead>
              <TableHead className="font-semibold">Amount</TableHead>
              <TableHead className="font-semibold">Entry Price</TableHead>
              <TableHead className="font-semibold">Total Cost</TableHead>
              <TableHead className="text-right font-semibold">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {items.map((item) => {
              const totalCost = item.amount * item.buy_price;
              return (
                <TableRow key={item.id}>
                  <TableCell className="font-semibold">{item.symbol}</TableCell>
                  <TableCell>{item.amount}</TableCell>
                  <TableCell>${item.buy_price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 6 })}</TableCell>
                  <TableCell>${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2 })}</TableCell>
                  <TableCell className="text-right">
                    <Button 
                      variant="ghost" 
                      size="icon" 
                      onClick={() => handleDelete(item.id)}
                      className="text-destructive hover:text-destructive hover:bg-destructive/10"
                    >
                      <Trash2 size={18} />
                    </Button>
                  </TableCell>
                </TableRow>
              );
            })}
            {items.length === 0 && (
              <TableRow>
                <TableCell colSpan="5" className="h-32 text-center text-muted-foreground">
                  <div className="flex flex-col items-center justify-center gap-2">
                    <Wallet size={32} className="text-muted-foreground/50" />
                    <p>No assets in portfolio</p>
                  </div>
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </Card>
    </div>
  );
}
