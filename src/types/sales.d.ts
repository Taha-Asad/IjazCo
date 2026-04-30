export type InvoiceStatus =
  | "draft"
  | "approved"
  | "paid"
  | "partial"
  | "cancelled";

export interface SalesInvoice {
  id: string;
  invoice_number: string;
  customer_id: string;
  customer_name?: string;
  company_id: string;
  branch_id: string;
  status: InvoiceStatus;
  subtotal: number;
  tax_amount: number;
  discount_amount: number;
  total_amount: number;
  paid_amount: number;
  due_amount: number;
  notes?: string;
  due_date?: string;
  created_at: string;
  updated_at: string;
}

export interface SalesInvoiceItem {
  id: string;
  invoice_id: string;
  item_id: string;
  item_name?: string;
  quantity: number;
  unit_price: number;
  discount: number;
  total: number;
}

export interface CreateSalesInvoiceRequest {
  customer_id: string;
  branch_id: string;
  items: CreateSalesItemRequest[];
  tax_rate?: number;
  discount_amount?: number;
  notes?: string;
  due_date?: string;
}

export interface CreateSalesItemRequest {
  item_id: string;
  quantity: number;
  unit_price: number;
  discount?: number;
}

export interface RecordPaymentRequest {
  amount: number;
  payment_method: string;
  payment_date: string;
  reference?: string;
}

export interface SalesSummary {
  total_revenue: number;
  total_invoices: number;
  paid_invoices: number;
  pending_invoices: number;
  overdue_invoices: number;
}
