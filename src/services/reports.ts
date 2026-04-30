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
  const response = await fetch(`${API_BASE_URL}/reports/sales`, {
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
    total_revenue: data.data?.total_sales || 0,
    total_orders: data.data?.invoice_count || 0,
    total_purchases: data.data?.total_purchases || 0,
    inventory_valuation: data.data?.inventory_valuation || 0,
    low_stock_items: data.data?.low_stock_count || 0,
    active_customers: data.data?.active_customers || 0,
    revenue_trend: data.data?.revenue_trend || 0,
  };
}

export async function getMonthlyChartData(): Promise<MonthlyData[]> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/dashboard/sales-chart`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) {
    return [];
  }
  const data = await response.json();
  const points = data.data || [];
  return points.map((point: any) => ({
    month: point.period || point.month || 'N/A',
    revenue: Number(point.sales || point.revenue || 0),
    expenses: Number(point.expenses || 0),
  }));
}

export async function exportReportsPdf(payload: Record<string, unknown>): Promise<Blob> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/reports/export/pdf`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error('Failed to export PDF report');
  return response.blob();
}
