// src/services/suppliers.ts
// Suppliers API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface Supplier {
  id: string;
  company_id: string;
  name: string;
  contact_person: string;
  email: string;
  phone: string;
  address: string;
  city?: string;
  country?: string;
  tax_number: string;
  is_active: boolean;
  created_at: string;
}

export async function getSuppliers(params?: {
  search?: string;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}): Promise<{ suppliers: Supplier[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.search) queryParams.append('search', params.search);
  if (params?.is_active !== undefined) queryParams.append('is_active', String(params.is_active));
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/suppliers/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch suppliers');
  const data = await response.json();
  return data.data || { suppliers: [], total: 0 };
}

export async function getSupplier(id: string): Promise<Supplier> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/suppliers/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch supplier');
  const data = await response.json();
  return data.data || data;
}

export async function createSupplier(supplier: Omit<Supplier, 'id' | 'company_id' | 'created_at'>): Promise<Supplier> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/suppliers`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(supplier),
  });
  if (!response.ok) throw new Error('Failed to create supplier');
  const data = await response.json();
  return data.data || data;
}

export async function updateSupplier(id: string, supplier: Partial<Supplier>): Promise<Supplier> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/suppliers/${id}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(supplier),
  });
  if (!response.ok) throw new Error('Failed to update supplier');
  const data = await response.json();
  return data.data || data;
}

export async function deleteSupplier(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/suppliers/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete supplier');
}
