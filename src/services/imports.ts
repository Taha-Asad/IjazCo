import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface ImportJob {
  id: string;
  company_id: string;
  user_id: string;
  entity_type: string;
  file_name: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  total_records: number;
  processed_records: number;
  successful_records: number;
  failed_records: number;
  error_log?: any;
  created_at: string;
  updated_at: string;
}

export interface CreateImportRequest {
  supplier_id: string;
  po_id?: string;
  shipment_date?: string;
  arrival_date?: string;
  shipping_method?: string;
  tracking_number?: string;
  container_number?: string;
  freight_cost?: number;
  insurance_cost?: number;
  customs_duty?: number;
  other_charges?: number;
  notes?: string;
  metadata?: Record<string, unknown>;
}

export async function getImports(params?: {
  entity_type?: string;
  status?: string;
  limit?: number;
  offset?: number;
}): Promise<{ imports: ImportJob[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.entity_type) queryParams.append('entity_type', params.entity_type);
  if (params?.status) queryParams.append('status', params.status);
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/imports${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch imports');
  const data = await response.json();
  return data.data || { imports: data || [], total: 0 };
}

export async function getImport(id: string): Promise<ImportJob> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/imports/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch import job');
  const data = await response.json();
  return data.data || data;
}

export async function createImport(payload: CreateImportRequest): Promise<ImportJob> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/imports`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(payload),
  });

  if (!response.ok) {
    const error = await response.json().catch(() => ({ message: 'Failed to create import order' }));
    throw new Error(error.message || 'Failed to create import order');
  }

  const data = await response.json();
  return data.data || data;
}

// Backward-compatible alias used by older pages.
export async function uploadImport(_entityType: string, _file: File): Promise<ImportJob> {
  throw new Error('File upload import is not available on this backend. Create import orders instead.');
}

export async function deleteImport(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/imports/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to delete import job');
  }
}
