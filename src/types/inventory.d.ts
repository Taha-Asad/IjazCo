export interface InventoryItem {
  id: string;
  name: string;
  sku: string;
  description?: string;
  category_id?: string;
  category_name?: string;
  company_id: string;
  unit_price: number;
  cost_price: number;
  unit: string;
  min_stock_level: number;
  max_stock_level?: number;
  is_active: boolean;
  serial_number?: string;
  created_at: string;
  updated_at: string;
  total_stock?: number;
}

export interface CreateInventoryItemRequest {
  name: string;
  sku: string;
  description?: string;
  category_id?: string;
  unit_price: number;
  cost_price: number;
  unit_of_measure: string;
  unit?: string; // Keep for frontend compatibility
  min_stock_level: number;
  max_stock_level?: number;
  serial_number?: string;
}

export interface UpdateInventoryItemRequest extends Partial<CreateInventoryItemRequest> {}

export interface ItemWithStock extends InventoryItem {
  stock_levels: StockLevel[];
}

export interface StockLevel {
  branch_id: string;
  branch_name: string;
  quantity: number;
}
