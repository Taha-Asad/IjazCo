import { ApiResponse, PaginatedResponse, PaginationParams } from "../types";
import apiClient from "./client";

export interface Category {
  id: string;
  name: string;
  description?: string;
  parent_id?: string;
  parent_name?: string;
  company_id: string;
  item_count?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateCategoryRequest {
  name: string;
  description?: string;
  parent_id?: string;
  company_id: string;
}

export const categoriesApi = {
  list: (params?: PaginationParams & { parent_id?: string }) =>
    apiClient.get<PaginatedResponse<Category>>("/categories", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<Category>>(`/categories/${id}`),

  create: (data: CreateCategoryRequest) =>
    apiClient.post<ApiResponse<Category>>("/categories", data),

  update: (id: string, data: Partial<CreateCategoryRequest>) =>
    apiClient.put<ApiResponse<Category>>(`/categories/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`/categories/${id}`),
};
