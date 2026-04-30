// src/services/users.ts
// Users API calls

import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface User {
  id: string;
  company_id: string;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  role_id: string;
  is_active: boolean;
  created_at: string;
}

export async function getUsers(params?: {
  search?: string;
  is_active?: boolean;
  limit?: number;
  offset?: number;
}): Promise<{ users: User[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.search) queryParams.append('search', params.search);
  if (params?.is_active !== undefined) queryParams.append('is_active', String(params.is_active));
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/users/${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch users');
  const data = await response.json();
  return data.data || { users: [], total: 0 };
}

export async function getUser(id: string): Promise<User> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/users/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch user');
  const data = await response.json();
  return data.data || data;
}

export async function createUser(user: {
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  password: string;
  role_id: string;
}): Promise<User> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/users`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(user),
  });
  if (!response.ok) throw new Error('Failed to create user');
  const data = await response.json();
  return data.data || data;
}

export async function updateUser(id: string, user: Partial<User>): Promise<User> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/users/${id}`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(user),
  });
  if (!response.ok) throw new Error('Failed to update user');
  const data = await response.json();
  return data.data || data;
}

export async function deleteUser(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/users/${id}`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) throw new Error('Failed to delete user');
}
