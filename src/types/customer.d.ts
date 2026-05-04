export interface Customer {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  credit_limit: number;
  current_balance?: number;
  outstanding_balance?: number;
  company_id: string;
  is_active: boolean;
  total_invoices?: number;
  total_spent?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateCustomerRequest {
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  credit_limit?: number;
}
