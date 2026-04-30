import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface ReportSummary {
  total_revenue: number;
  total_orders: number;
  total_purchases: number;
  inventory_valuation: number;
  low_stock_items: number;
  active_customers: number;
  revenue_trend: number; // Percentage change
}

export interface MonthlyData {
  month: string;
  revenue: number;
  expenses: number;
}

export async function getDashboardSummary(): Promise<ReportSummary> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/dashboard/stats`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch dashboard statistics');
  const data = await response.json();
  
  // Transform or fallback to zeroes if data is missing
  return {
    total_revenue: data.data?.total_revenue || 0,
    total_orders: data.data?.total_orders || 0,
    total_purchases: data.data?.total_purchases || 0,
    inventory_valuation: data.data?.inventory_valuation || 0,
    low_stock_items: data.data?.low_stock_items || 0,
    active_customers: data.data?.active_customers || 0,
    revenue_trend: data.data?.revenue_trend || 0,
  };
}

// Mocking chart data as the backend might not have this exact endpoint yet
export async function getMonthlyChartData(): Promise<MonthlyData[]> {
  // Simulate network request
  await new Promise(resolve => setTimeout(resolve, 600));
  
  return [
    { month: 'Jan', revenue: 12000, expenses: 8000 },
    { month: 'Feb', revenue: 15000, expenses: 9500 },
    { month: 'Mar', revenue: 14000, expenses: 8200 },
    { month: 'Apr', revenue: 18000, expenses: 10000 },
    { month: 'May', revenue: 22000, expenses: 12000 },
    { month: 'Jun', revenue: 25000, expenses: 14000 },
  ];
}
