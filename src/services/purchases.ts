// src/services/purchases.ts
// Purchases API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface PurchaseOrder {
  id: string;
  company_id: string;
  branch_id: string;
  supplier_id: string;
  po_number: string;
  po_date: string;
  expected_delivery_date?: string;
  status: 'draft' | 'submitted' | 'confirmed' | 'shipped' | 'received' | 'cancelled';
  subtotal: number;
  discount_amount: number;
  tax_amount: number;
  shipping_amount: number;
  total_amount: number;
  currency: string;
  exchange_rate: number;
  payment_terms: number;
  shipping_address?: string;
  notes?: string;
  created_at: string;
}

export interface CreatePurchaseOrderRequest {
  company_id?: string;
  branch_id: string;
  supplier_id: string;
  po_date?: string;
  expected_delivery_date?: string;
  items: Array<{
    item_id: string;
    description?: string;
    quantity_ordered: number;
    unit_cost: number;
    tax_percentage?: number;
  }>;
  discount_amount?: number;
  shipping_amount?: number;
  currency?: string;
  exchange_rate?: number;
  payment_terms?: number;
  shipping_address?: string;
  notes?: string;
}

export async function getPurchaseOrders(params?: {
  status?: string;
  supplier_id?: string;
  limit?: number;
  offset?: number;
}): Promise<{ purchases: PurchaseOrder[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.status) queryParams.append('status', params.status);
  if (params?.supplier_id) queryParams.append('supplier_id', params.supplier_id);
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/purchases/orders${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch purchase orders');
  const data = await response.json();
  // Backend returns data.data which contains purchases array, or array directly.
  return data.data || { purchases: data || [], total: 0 };
}

export async function getPurchaseOrder(id: string): Promise<PurchaseOrder> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/purchases/orders/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch purchase order');
  const data = await response.json();
  return data.data || data;
}

export async function createPurchaseOrder(data: CreatePurchaseOrderRequest): Promise<PurchaseOrder> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/purchases/orders`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to create purchase order');
  }

  const result = await response.json();
  return result.data || result;
}

export async function updatePurchaseOrder(id: string, data: Partial<PurchaseOrder>): Promise<PurchaseOrder> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/purchases/orders/${id}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });
  if (!response.ok) throw new Error('Failed to update purchase order');
  const result = await response.json();
  return result.data || result;
}

export async function deletePurchaseOrder(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/purchases/orders/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete purchase order');
}
