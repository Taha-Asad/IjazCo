import apiClient from "./client";
import type { ApiResponse, PaginatedResponse, PaginationParams } from "@/types";

export interface Company {
  id: string;
  name: string;
  slug: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  logo_url?: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateCompanyRequest {
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
}

export const companiesApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<Company>>("/companies", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<Company>>(`/companies/${id}`),

  create: (data: CreateCompanyRequest) =>
    apiClient.post<ApiResponse<Company>>("/companies", data),

  update: (id: string, data: Partial<CreateCompanyRequest>) =>
    apiClient.put<ApiResponse<Company>>(`/companies/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`/companies/${id}`),
};
