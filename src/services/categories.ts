import { getToken } from './auth';
import { API_BASE_URL } from './api';

export interface Category {
  id: string;
  company_id: string;
  name: string;
  description?: string;
  parent_id?: string;
  slug: string;
  metadata?: any;
  created_at: string;
}

export interface CreateCategoryRequest {
  name: string;
  description?: string;
  parent_id?: string;
}

export async function getCategories(params?: {
  search?: string;
  limit?: number;
  offset?: number;
}): Promise<{ categories: Category[]; total: number }> {
  const token = getToken();
  const queryParams = new URLSearchParams();
  
  if (params?.search) queryParams.append('search', params.search);
  if (params?.limit) queryParams.append('limit', String(params.limit));
  if (params?.offset) queryParams.append('offset', String(params.offset));

  const url = `${API_BASE_URL}/categories${queryParams.toString() ? '?' + queryParams.toString() : ''}`;

  const response = await fetch(url, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });

  if (!response.ok) throw new Error('Failed to fetch categories');
  const data = await response.json();
  return data.data || { categories: data || [], total: 0 };
}

export async function getCategory(id: string): Promise<Category> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/categories/${id}`, {
    method: 'GET',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
  });
  if (!response.ok) throw new Error('Failed to fetch category');
  const data = await response.json();
  return data.data || data;
}

export async function createCategory(category: CreateCategoryRequest): Promise<Category> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/categories/create-category`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(category),
  });
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to create category');
  }
  const data = await response.json();
  return data.data || data;
}

export async function updateCategory(id: string, category: Partial<CreateCategoryRequest>): Promise<Category> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/categories/${id}/update-category`, {
    method: 'PUT',
    headers: {
      'Authorization': `Bearer ${token}`,
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(category),
  });
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to update category');
  }
  const data = await response.json();
  return data.data || data;
}

export async function deleteCategory(id: string): Promise<void> {
  const token = getToken();
  const response = await fetch(`${API_BASE_URL}/categories/${id}/delete-category`, {
    method: 'DELETE',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message || 'Failed to delete category');
  }
}
