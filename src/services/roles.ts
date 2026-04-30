import { API_BASE_URL } from './api';
import { getToken } from './auth';

export interface Role {
  id: string;
  company_id: string;
  name: string;
  role_type?: string;
  permissions: Record<string, unknown>;
  is_system: boolean;
  is_active: boolean;
}

function authHeaders() {
  return {
    Authorization: `Bearer ${getToken()}`,
    'Content-Type': 'application/json',
  };
}

export async function getRoles(): Promise<Role[]> {
  const response = await fetch(`${API_BASE_URL}/roles/`, { headers: authHeaders() });
  if (!response.ok) throw new Error('Failed to fetch roles');
  return response.json();
}

export async function getRole(id: string): Promise<Role> {
  const response = await fetch(`${API_BASE_URL}/roles/${id}`, { headers: authHeaders() });
  if (!response.ok) throw new Error('Failed to fetch role');
  return response.json();
}

export async function createRole(payload: { name: string; role_type?: string; permissions?: Record<string, unknown> }): Promise<Role> {
  const response = await fetch(`${API_BASE_URL}/roles/`, {
    method: 'POST',
    headers: authHeaders(),
    body: JSON.stringify(payload),
  });
  if (!response.ok) throw new Error('Failed to create role');
  const data = await response.json();
  return data.data || data;
}
