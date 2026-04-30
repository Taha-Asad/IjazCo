// src/services/sales.ts
// Sales API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface SalesInvoice {
  id: string;
  company_id: string;
  branch_id: string;
  customer_id: string;
  invoice_number: string;
  invoice_date: string;
  status: 'draft' | 'pending' | 'approved' | 'paid' | 'cancelled';
  subtotal: number;
  tax_amount: number;
  discount_amount: number;
  total_amount: number;
  balance_due: number;
  created_at: string;
}

export interface CreateSalesInvoiceRequest {
  customer_id: string;
  branch_id: string;
  items: Array<{
    item_id: string;
    quantity: number;
    unit_price: number;
  }>;
  tax_rate?: number;
  discount_amount?: number;
}

export async function getSalesInvoices(params?: {
  status?: string;
  customer_id?: string;
  start_date?: string;
  end_date?: string;
  limit?: number;
  offset?: number;
}): Promise<{ invoices: SalesInvoice[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.status) queryParams.append('status', params.status);
  if (params?.customer_id) queryParams.append('customer_id', params.customer_id);
  if (params?.start_date) queryParams.append('start_date', params.start_date);
  if (params?.end_date) queryParams.append('end_date', params.end_date);
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/sales/invoices${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch sales invoices');
  const data = await response.json();
  return data.data || { invoices: [], total: 0 };
}

export async function getSalesInvoice(id: string): Promise<SalesInvoice> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/sales/invoices/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch invoice');
  const data = await response.json();
  return data.data || data;
}

export async function createSalesInvoice(data: CreateSalesInvoiceRequest): Promise<SalesInvoice> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/sales/invoices`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to create invoice');
  }

  const result = await response.json();
  return result.data || result;
}

export async function updateSalesInvoice(id: string, data: Partial<SalesInvoice>): Promise<SalesInvoice> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/sales/invoices/${id}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) throw new Error('Failed to update invoice');
  const result = await response.json();
  return result.data || result;
}

export async function deleteSalesInvoice(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/sales/invoices/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete invoice');
}
