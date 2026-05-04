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

  getById: (id: string) => apiClient.get<any>(`leads/${id}`),

  create: (data: CreateLeadRequest) => {
    // Clean up empty strings for optional fields
    const cleanData: any = { name: data.name };
    if (data.email && data.email.trim()) cleanData.email = data.email;
    if (data.phone && data.phone.trim()) cleanData.phone = data.phone;
    if (data.company_name && data.company_name.trim()) cleanData.company_name = data.company_name;
    if (data.status && data.status.trim()) cleanData.status = data.status;
    if (data.source && data.source.trim()) cleanData.source = data.source;
    if (data.estimated_value !== undefined && data.estimated_value !== null && data.estimated_value !== "") cleanData.estimated_value = data.estimated_value;
    if (data.description && data.description.trim()) cleanData.description = data.description;
    if (data.expected_close_date && data.expected_close_date.trim()) cleanData.expected_close_date = data.expected_close_date;
    return apiClient.post<ApiResponse<Lead>>("leads", cleanData);
  },

  update: (id: string, data: Partial<CreateLeadRequest>) => {
    // Clean up empty strings for optional fields
    const cleanData: any = {};
    if (data.name && data.name.trim()) cleanData.name = data.name;
    if (data.email && data.email.trim()) cleanData.email = data.email;
    if (data.phone && data.phone.trim()) cleanData.phone = data.phone;
    if (data.company_name && data.company_name.trim()) cleanData.company_name = data.company_name;
    if (data.status && data.status.trim()) cleanData.status = data.status;
    if (data.source && data.source.trim()) cleanData.source = data.source;
    if (data.estimated_value !== undefined && data.estimated_value !== null && data.estimated_value !== "") cleanData.estimated_value = data.estimated_value;
    if (data.description && data.description.trim()) cleanData.description = data.description;
    if (data.expected_close_date && data.expected_close_date.trim()) cleanData.expected_close_date = data.expected_close_date;
    return apiClient.put<ApiResponse<Lead>>(`leads/${id}`, cleanData);
  },

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`leads/${id}`),
};
