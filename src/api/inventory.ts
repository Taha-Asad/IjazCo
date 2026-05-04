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

  getById: (id: string) => apiClient.get<any>(`inventory/${id}`),

  create: (data: CreateInventoryItemRequest) => {
    // Clean up null/empty values and map fields
    const cleanData: any = { ...data };
    // Map frontend 'unit_price' to backend 'selling_price'
    if (cleanData.unit_price !== undefined) {
      cleanData.selling_price = cleanData.unit_price;
      delete cleanData.unit_price;
    }
    // Map frontend 'unit' to backend 'unit_of_measure'
    if (cleanData.unit) {
      cleanData.unit_of_measure = cleanData.unit;
      delete cleanData.unit;
    }
    if (!cleanData.category_id) {
      delete cleanData.category_id;
    }
    if (cleanData.max_stock_level === null || cleanData.max_stock_level === "") {
      delete cleanData.max_stock_level;
    }
    if (!cleanData.serial_number) {
      delete cleanData.serial_number;
    }
    // Remove fields not expected by backend
    delete cleanData.is_active;
    // Add required fields with defaults
    cleanData.is_serialized = false;
    cleanData.is_batch_tracked = false;
    cleanData.reorder_level = 0;
    cleanData.reorder_quantity = 0;
    cleanData.lead_time_days = 0;
    return apiClient.post<ApiResponse<InventoryItem>>("inventory", cleanData);
  },

  update: (id: string, data: UpdateInventoryItemRequest) => {
    // Clean up null/empty values and map fields
    const cleanData: any = { ...data };
    // Map frontend 'unit_price' to backend 'selling_price'
    if (cleanData.unit_price !== undefined) {
      cleanData.selling_price = cleanData.unit_price;
      delete cleanData.unit_price;
    }
    // Map frontend 'unit' to backend 'unit_of_measure'
    if (cleanData.unit) {
      cleanData.unit_of_measure = cleanData.unit;
      delete cleanData.unit;
    }
    if (!cleanData.category_id) {
      delete cleanData.category_id;
    }
    if (cleanData.max_stock_level === null || cleanData.max_stock_level === "") {
      delete cleanData.max_stock_level;
    }
    if (!cleanData.serial_number) {
      delete cleanData.serial_number;
    }
    // Remove fields not expected by backend
    delete cleanData.is_active;
    return apiClient.put<ApiResponse<InventoryItem>>(`inventory/${id}`, cleanData);
  },

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`inventory/${id}`),

  getStock: (id: string) =>
    apiClient.get<ApiResponse<ItemWithStock>>(`inventory/${id}/stock`),

  getLowStock: (params?: PaginationParams) =>
    apiClient.get<PaginatedResponse<InventoryItem>>("inventory/low-stock", {
      params,
    }),
};
