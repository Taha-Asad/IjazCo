import { API_BASE_URL } from './api';
import { getToken } from './auth';

export interface Company {
  id: string;
  name: string;
  code?: string;
  email?: string;
  country?: string;
  is_active?: boolean;
}

export async function getCompanies(): Promise<Company[]> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/companies/`, {
    method: 'GET',
    headers: {
      Authorization: `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch companies');
  const data = await response.json();
  return data.data || data || [];
}
