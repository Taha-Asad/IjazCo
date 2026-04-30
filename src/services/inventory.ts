// src/services/inventory.ts
// Inventory & Stock API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface InventoryItem {
  id: string;
  company_id: string;
  category_id: string;
  sku: string;
  name: string;
  description: string;
  cost_price: number;
  selling_price: number;
  reorder_level: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface StockItem {
  item_id: string;
  item_name: string;
  sku: string;
  branch_id: string;
  branch_name: string;
  quantity_on_hand: number;
  reorder_level: number;
  cost_price: number;
  selling_price: number;
}

export interface Category {
  id: string;
  company_id: string;
  name: string;
  description: string;
  is_active: boolean;
}

export interface CreateInventoryItemRequest {
  category_id: string;
  sku: string;
  name: string;
  description?: string;
  cost_price: number;
  selling_price: number;
  reorder_level: number;
}

export async function getInventoryItems(params?: {
  category_id?: string;
  search?: string;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}): Promise<{ items: InventoryItem[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.category_id) queryParams.append('category_id', params.category_id);
  if (params?.search) queryParams.append('search', params.search);
  if (params?.is_active !== undefined) queryParams.append('is_active', String(params.is_active));
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/inventory/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch inventory items');
  const data = await response.json();
  return data.data || { items: [], total: 0 };
}

export async function getInventoryItem(id: string): Promise<InventoryItem> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/inventory/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch item');
  const data = await response.json();
  return data.data || data;
}

export async function createInventoryItem(item: CreateInventoryItemRequest): Promise<InventoryItem> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/inventory/create-item`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(item),
  });
  if (!response.ok) throw new Error('Failed to create item');
  const data = await response.json();
  return data.data || data;
}

export async function updateInventoryItem(id: string, item: Partial<CreateInventoryItemRequest>): Promise<InventoryItem> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/inventory/${id}/update-item`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(item),
  });
  if (!response.ok) throw new Error('Failed to update item');
  const data = await response.json();
  return data.data || data;
}

export async function deleteInventoryItem(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/inventory/${id}/delete-item`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete item');
}

export async function getStock(params?: {
  branch_id?: string;
  low_stock?: boolean;
  out_of_stock?: boolean;
}): Promise<StockItem[]> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.branch_id) queryParams.append('branch_id', params.branch_id);
  if (params?.low_stock) queryParams.append('low_stock', 'true');
  if (params?.out_of_stock) queryParams.append('out_of_stock', 'true');

  const url = `${API_BASE_URL}/stock/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch stock');
  const data = await response.json();
  return data.data || [];
}

export async function getCategories(): Promise<Category[]> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/categories/`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch categories');
  const data = await response.json();
  return data.data || [];
}

export async function getLowStockItems(): Promise<InventoryItem[]> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/inventory/low-stock`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch low stock items');
  const data = await response.json();
  return data.data || [];
}
