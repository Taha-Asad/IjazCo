import { API_BASE_URL } from './api';
import { getToken } from './auth';

export interface StockAdjustmentRequest {
  item_id: string;
  branch_id: string;
  quantity_change: number;
  reason?: string;
}

export interface StockTransferRequest {
  item_id: string;
  from_branch_id: string;
  to_branch_id: string;
  quantity: number;
  notes?: string;
}

export async function adjustStock(payload: StockAdjustmentRequest): Promise<any> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/stock/adjust`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error('Failed to adjust stock');
  const data = await response.json();
  return data.data || data;
}

export async function transferStock(payload: StockTransferRequest): Promise<any> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/stock/transfer`, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error('Failed to transfer stock');
  const data = await response.json();
  return data.data || data;
}

export async function listStockMovements(params?: { item_id?: string; branch_id?: string; limit?: number; offset?: number }): Promise<any[]> {
  const token = getToken();
  const query = new URLSearchParams();
  if (params?.item_id) query.append('item_id', params.item_id);
  if (params?.branch_id) query.append('branch_id', params.branch_id);
  if (params?.limit) query.append('limit', String(params.limit));
  if (params?.offset) query.append('offset', String(params.offset));
  const response = await fetch(`${API_BASE_URL}/stock/movements${query.toString() ? `?${query.toString()}` : ''}`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch stock movements');
  const data = await response.json();
  return data.data || data || [];
}

export async function getLowStockAlerts(): Promise<any[]> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/stock/low-stock-alerts`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch low stock alerts');
  const data = await response.json();
  return data.data || data || [];
}
