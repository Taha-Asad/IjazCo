import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface Supplier {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  contact_person?: string;
  payment_terms?: number;
  company_id: string;
  is_active: boolean;
  total_orders?: number;
  total_spent?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateSupplierRequest {
  name: string;
  supplier_code: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  contact_person?: string;
  payment_terms?: number;
  company_id: string;
}

export const suppliersApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<Supplier>>("suppliers", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<Supplier>>(`suppliers/${id}`),

  create: (data: CreateSupplierRequest) =>
    apiClient.post<ApiResponse<Supplier>>("suppliers", data),

  update: (id: string, data: Partial<CreateSupplierRequest>) =>
    apiClient.put<ApiResponse<Supplier>>(`suppliers/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`suppliers/${id}`),
};
