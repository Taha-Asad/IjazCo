// src/services/customers.ts
// Customers API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface Customer {
  id: string;
  company_id: string;
  name: string;
  email: string;
  phone: string;
  address: string;
  city: string;
  country: string;
  tax_number: string;
  credit_limit: number;
  is_active: boolean;
  created_at: string;
}

export interface CreateCustomerRequest {
  name: string;
  email: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  tax_number?: string;
  credit_limit?: number;
}

export async function getCustomers(params?: {
  search?: string;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}): Promise<{ customers: Customer[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.search) queryParams.append('search', params.search);
  if (params?.is_active !== undefined) queryParams.append('is_active', String(params.is_active));
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/customers/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch customers');
  const data = await response.json();
  return data.data || { customers: [], total: 0 };
}

export async function getCustomer(id: string): Promise<Customer> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/customers/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch customer');
  const data = await response.json();
  return data.data || data;
}

export async function createCustomer(customer: CreateCustomerRequest): Promise<Customer> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/customers/`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(customer),
  });
  if (!response.ok) throw new Error('Failed to create customer');
  const data = await response.json();
  return data.data || data;
}

export async function updateCustomer(id: string, customer: Partial<CreateCustomerRequest>): Promise<Customer> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/customers/${id}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(customer),
  });
  if (!response.ok) throw new Error('Failed to update customer');
  const data = await response.json();
  return data.data || data;
}

export async function deleteCustomer(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/customers/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete customer');
}
