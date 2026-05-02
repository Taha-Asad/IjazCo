import { ApiResponse, PaginatedResponse, PaginationParams } from "../types";
import apiClient from "./client";

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
  list: (params?: PaginationParams) => {
    // We create a clean object to ensure we don't send "" or undefined
    // which can also cause 400 errors.
    const searchParams: any = {};

    if (params?.page) searchParams.page = Number(params.page);
    if (params?.per_page) searchParams.per_page = Number(params.per_page);
    if (params?.search?.trim()) searchParams.search = params.search.trim();

    return apiClient.get<PaginatedResponse<Company>>("companies", {
      params: searchParams,
    });
  },

  getById: (id: string) =>
    apiClient.get<ApiResponse<Company>>(`companies/${id}`),

  create: (data: CreateCompanyRequest) =>
    apiClient.post<ApiResponse<Company>>("companies", data),

  update: (id: string, data: Partial<CreateCompanyRequest>) =>
    apiClient.put<ApiResponse<Company>>(`companies/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`companies/${id}`),
};
