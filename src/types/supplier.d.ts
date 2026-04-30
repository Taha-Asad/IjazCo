export interface Supplier {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  contact_person?: string;
  payment_terms?: number;
  company_id: string;
  is_active: boolean;
  total_orders?: number;
  total_spent?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateSupplierRequest {
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  contact_person?: string;
  payment_terms?: number;
}
