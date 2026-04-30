// src/services/dashboard.ts
// Dashboard API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface OverviewMetrics {
  total_revenue: number;
  revenue_change_percent: number;
  total_orders: number;
  orders_change_percent: number;
  total_customers: number;
  new_customers: number;
  inventory_value: number;
}

export interface SalesMetrics {
  total_sales: number;
  invoice_count: number;
  average_order_value: number;
  outstanding_amount: number;
  top_items: TopItem[];
  by_status: StatusBreakdown;
}

export interface TopItem {
  item_id: string;
  item_name: string;
  sku: string;
  quantity_sold: number;
  total_revenue: number;
}

export interface StatusBreakdown {
  draft: number;
  pending: number;
  approved: number;
  paid: number;
  cancelled: number;
}

export interface InventoryMetrics {
  total_items: number;
  total_quantity: number;
  total_cost_value: number;
  total_selling_value: number;
  low_stock_items: number;
  out_of_stock_items: number;
  by_category: CategoryBreakdown[];
}

export interface CategoryBreakdown {
  category_id: string;
  category_name: string;
  item_count: number;
  total_value: number;
}

export interface PurchaseMetrics {
  total_purchases: number;
  po_count: number;
  pending_pos: number;
  average_po_value: number;
  top_suppliers: TopSupplier[];
}

export interface TopSupplier {
  supplier_id: string;
  supplier_name: string;
  po_count: number;
  total_amount: number;
}

export interface ActivityItem {
  id: string;
  activity_type: string;
  description: string;
  user_name: string;
  timestamp: string;
}

export interface PendingApprovals {
  sales_invoices: number;
  purchase_orders: number;
}

export interface DashboardStats {
  overview: OverviewMetrics;
  sales: SalesMetrics;
  inventory: InventoryMetrics;
  purchases: PurchaseMetrics;
  recent_activities: ActivityItem[];
  low_stock_count: number;
  pending_approvals: PendingApprovals;
}

export async function getDashboardStats(
  startDate?: string,
  endDate?: string,
  branchId?: string
): Promise<DashboardStats> {
  const token = getToken();
  const params = new URLSearchParams();
  
  if (startDate) params.append('start_date', startDate);
  if (endDate) params.append('end_date', endDate);
  if (branchId) params.append('branch_id', branchId);

  const queryString = params.toString();
  const url = `${API_BASE_URL}/dashboard/stats${queryString ? '?' + queryString : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) {
    throw new Error('Failed to fetch dashboard stats');
  }

  const data = await response.json();
  return data.data || data;
}
