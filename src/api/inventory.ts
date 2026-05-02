import apiClient from "./client";
import type {
  InventoryItem,
  CreateInventoryItemRequest,
  UpdateInventoryItemRequest,
  ItemWithStock,
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export const inventoryApi = {
  list: (params?: PaginationParams & { category_id?: string }) =>
    apiClient.get<PaginatedResponse<InventoryItem>>("inventory", { params }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<InventoryItem>>(`inventory/${id}`),

  create: (data: CreateInventoryItemRequest) =>
    apiClient.post<ApiResponse<InventoryItem>>("inventory", data),

  update: (id: string, data: UpdateInventoryItemRequest) =>
    apiClient.put<ApiResponse<InventoryItem>>(`inventory/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`inventory/${id}`),

  getStock: (id: string) =>
    apiClient.get<ApiResponse<ItemWithStock>>(`inventory/${id}/stock`),

  getLowStock: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<InventoryItem>>("inventory/low-stock", {
      params,
    }),
};
