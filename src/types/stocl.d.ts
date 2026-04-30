export interface StockRecord {
  id: string;
  item_id: string;
  item_name: string;
  item_sku: string;
  branch_id: string;
  branch_name: string;
  quantity: number;
  min_stock_level: number;
  unit: string;
}

export interface StockMovement {
  id: string;
  item_id: string;
  item_name: string;
  branch_id: string;
  branch_name: string;
  movement_type: "in" | "out" | "transfer" | "adjustment" | "count";
  quantity: number;
  reference?: string;
  notes?: string;
  created_by?: string;
  created_at: string;
}

export interface AdjustStockRequest {
  item_id: string;
  branch_id: string;
  quantity: number;
  reason: string;
  notes?: string;
}

export interface TransferStockRequest {
  item_id: string;
  from_branch_id: string;
  to_branch_id: string;
  quantity: number;
  notes?: string;
}

export interface PhysicalCountRequest {
  item_id: string;
  branch_id: string;
  counted_quantity: number;
  notes?: string;
}
