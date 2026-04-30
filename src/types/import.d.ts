export type ImportOrderStatus =
  | "draft"
  | "in_transit"
  | "customs"
  | "delivered"
  | "cancelled";

export interface ImportOrder {
  id: string;
  import_number: string;
  supplier_id: string;
  supplier_name?: string;
  purchase_order_id?: string;
  company_id: string;
  status: ImportOrderStatus;
  origin_country: string;
  shipping_method?: string;
  shipping_cost: number;
  customs_duty: number;
  other_charges: number;
  total_cost: number;
  estimated_arrival?: string;
  actual_arrival?: string;
  tracking_number?: string;
  notes?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateImportOrderRequest {
  supplier_id: string;
  purchase_order_id?: string;
  origin_country: string;
  shipping_method?: string;
  shipping_cost?: number;
  customs_duty?: number;
  other_charges?: number;
  estimated_arrival?: string;
  tracking_number?: string;
  notes?: string;
}
