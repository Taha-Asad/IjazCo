export type PurchaseOrderStatus =
  | "draft"
  | "submitted"
  | "received"
  | "cancelled";

export interface PurchaseOrder {
  id: string;
  po_number: string;
  supplier_id: string;
  supplier_name?: string;
  company_id: string;
  branch_id: string;
  status: PurchaseOrderStatus;
  subtotal: number;
  tax_amount: number;
  total_amount: number;
  notes?: string;
  expected_date?: string;
  received_date?: string;
  created_at: string;
  updated_at: string;
}

export interface PurchaseOrderItem {
  id: string;
  order_id: string;
  item_id: string;
  item_name?: string;
  item_sku?: string;
  quantity: number;
  unit_price: number;
  total: number;
}

export interface CreatePurchaseOrderRequest {
  supplier_id: string;
  branch_id: string;
  items: {
    item_id: string;
    quantity: number;
    unit_price: number;
  }[];
  notes?: string;
  expected_date?: string;
}
