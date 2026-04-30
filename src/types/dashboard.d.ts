export interface DashboardStats {
  total_revenue: number;
  revenue_change: number;
  total_invoices: number;
  invoices_change: number;
  total_customers: number;
  customers_change: number;
  low_stock_items: number;
  total_inventory_value: number;
  pending_orders: number;
  recent_sales: RecentSale[];
  top_items: TopItem[];
}

export interface RecentSale {
  id: string;
  invoice_number: string;
  customer_name: string;
  total_amount: number;
  status: string;
  created_at: string;
}

export interface TopItem {
  id: string;
  name: string;
  sku: string;
  total_sold: number;
  revenue: number;
}

export interface SalesChartData {
  period: string;
  revenue: number;
  invoices: number;
}

export interface InventoryValuation {
  branch_id: string;
  branch_name: string;
  total_value: number;
  item_count: number;
}
