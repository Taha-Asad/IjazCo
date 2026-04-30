import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface StockRecord {
  id: string;
  item_id: string;
  item_name: string;
  item_sku: string;
  branch_id: string;
  branch_name: string;
  quantity: number;
  min_stock_level: number;
  unit: string;
}

export interface StockMovement {
  id: string;
  item_id: string;
  item_name: string;
  branch_id: string;
  movement_type: string;
  quantity: number;
  reference?: string;
  notes?: string;
  created_at: string;
}

export interface AdjustStockRequest {
  item_id: string;
  branch_id: string;
  quantity: number;
  reason: string;
  notes?: string;
}

export interface TransferStockRequest {
  item_id: string;
  from_branch_id: string;
  to_branch_id: string;
  quantity: number;
  notes?: string;
}

export const stockApi = {
  list: (params?: PaginationParams & { branch_id?: string }) =>
    apiClient.get<PaginatedResponse<StockRecord>>("/stock", { params }),

  adjust: (data: AdjustStockRequest) =>
    apiClient.post<ApiResponse<StockRecord>>("/stock/adjust", data),

  transfer: (data: TransferStockRequest) =>
    apiClient.post<ApiResponse<null>>("/stock/transfer", data),

  listMovements: (params?: PaginationParams & { item_id?: string }) =>
    apiClient.get<PaginatedResponse<StockMovement>>("/stock/movements", {
      params,
    }),

  physicalCount: (data: {
    item_id: string;
    branch_id: string;
    counted_quantity: number;
  }) => apiClient.post<ApiResponse<null>>("/stock/physical-count", data),

  getLowStockAlerts: () =>
    apiClient.get<ApiResponse<StockRecord[]>>("/stock/low-stock-alerts"),
};
