import apiClient from "./client";
import type { ApiResponse, PaginatedResponse, PaginationParams } from "@/types";

export interface Role {
  id: string;
  name: string;
  description?: string;
  company_id: string;
  permissions: Record<string, string[]>;
  user_count?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateRoleRequest {
  name: string;
  description?: string;
  company_id: string;
  permissions?: Record<string, string[]>;
}

export interface UpdatePermissionsRequest {
  permissions: Record<string, string[]>;
}

export const rolesApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<Role>>("/roles", { params }),

  getById: (id: string) => apiClient.get<ApiResponse<Role>>(`/roles/${id}`),

  create: (data: CreateRoleRequest) =>
    apiClient.post<ApiResponse<Role>>("/roles", data),

  update: (id: string, data: Partial<CreateRoleRequest>) =>
    apiClient.put<ApiResponse<Role>>(`/roles/${id}`, data),

  delete: (id: string) => apiClient.delete<ApiResponse<null>>(`/roles/${id}`),

  getPermissions: (id: string) =>
    apiClient.get<ApiResponse<Record<string, string[]>>>(
      `/roles/${id}/permissions`,
    ),

  updatePermissions: (id: string, data: UpdatePermissionsRequest) =>
    apiClient.put<ApiResponse<Role>>(`/roles/${id}/permissions`, data),
};
