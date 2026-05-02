import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface Lead {
  id: string;
  lead_number: string;
  name: string;
  email?: string;
  phone?: string;
  company_name?: string;
  status: string;
  source: string;
  estimated_value?: number;
  description?: string;
  assigned_to?: string;
  converted_to_customer?: string;
  expected_close_date?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateLeadRequest {
  name: string;
  email?: string;
  phone?: string;
  company_name?: string;
  status?: string;
  source?: string;
  estimated_value?: number;
  description?: string;
  assigned_to?: string;
  expected_close_date?: string;
}

export const leadsApi = {
  list: (params?: PaginationParams & { status?: string }) =>
    apiClient.get<PaginatedResponse<Lead>>("leads", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<Lead>>(`leads/${id}`),

  create: (data: CreateLeadRequest) =>
    apiClient.post<ApiResponse<Lead>>("leads", data),

  update: (id: string, data: Partial<CreateLeadRequest>) =>
    apiClient.put<ApiResponse<Lead>>(`leads/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`leads/${id}`),
};
