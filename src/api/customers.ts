import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface Customer {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  credit_limit: number;
  current_balance: number;
  company_id: string;
  is_active: boolean;
  total_invoices?: number;
  total_spent?: number;
  created_at: string;
}

export const customersApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<Customer>>("/customers", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<Customer>>(`/customers/${id}`),

  create: (data: Omit<Customer, "id" | "created_at" | "current_balance">) =>
    apiClient.post<ApiResponse<Customer>>("/customers", data),

  update: (id: string, data: Partial<Customer>) =>
    apiClient.put<ApiResponse<Customer>>(`/customers/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`/customers/${id}`),
};
