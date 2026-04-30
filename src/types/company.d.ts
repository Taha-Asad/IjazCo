export interface Company {
  id: string;
  name: string;
  slug: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  logo_url?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateCompanyRequest {
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
}
