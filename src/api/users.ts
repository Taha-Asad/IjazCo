import apiClient from "./client";
import type {
  User,
  CreateUserRequest,
  UpdateUserRequest,
  UpdateUserStatusRequest,
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export const usersApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<User>>("users", { params }),

  getById: (id: string) => apiClient.get<ApiResponse<User>>(`users/${id}`),

  create: (data: CreateUserRequest) =>
    apiClient.post<ApiResponse<User>>("users", data),

  update: (id: string, data: UpdateUserRequest) =>
    apiClient.put<ApiResponse<User>>(`users/${id}`, data),

  delete: (id: string) => apiClient.delete<ApiResponse<null>>(`users/${id}`),

  changePassword: (id: string, data: { new_password: string }) =>
    apiClient.post<ApiResponse<null>>(`users/${id}/change-password`, data),

  updateStatus: (id: string, data: UpdateUserStatusRequest) =>
    apiClient.patch<ApiResponse<User>>(`users/${id}/status`, data),
};
