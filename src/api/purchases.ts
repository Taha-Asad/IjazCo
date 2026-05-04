import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface PurchaseOrder {
  id: string;
  po_number: string;
  supplier_id: string;
  supplier_name?: string;
  company_id: string;
  branch_id: string;
  status: string;
  total_amount: number;
  notes?: string;
  expected_date?: string;
  created_at: string;
}

export interface PurchaseOrderItem {
  id: string;
  order_id: string;
  item_id: string;
  item_name?: string;
  quantity: number;
  unit_price: number;
  total: number;
}

export interface CreatePurchaseOrderRequest {
  supplier_id: string;
  branch_id: string;
  items: { item_id: string; quantity: number; unit_price: number }[];
  notes?: string;
  expected_date?: string;
}

export const purchasesApi = {
  list: (params?: PaginationParams & { status?: string }) =>
    apiClient.get<PaginatedResponse<PurchaseOrder>>("purchases/orders", {
      params,
    }),

  getById: (id: string) => apiClient.get<any>(`purchases/orders/${id}`),

  create: (data: CreatePurchaseOrderRequest) =>
    apiClient.post<ApiResponse<PurchaseOrder>>("purchases/orders", data),

  update: (id: string, data: Partial<CreatePurchaseOrderRequest>) =>
    apiClient.put<ApiResponse<PurchaseOrder>>(`purchases/orders/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`purchases/orders/${id}`),

  submit: (id: string) =>
    apiClient.post<ApiResponse<PurchaseOrder>>(
      `purchases/orders/${id}/submit`,
    ),

  receiveGoods: (id: string, data?: { notes?: string }) =>
    apiClient.post<ApiResponse<PurchaseOrder>>(
      `purchases/orders/${id}/receive`,
      data,
    ),

  getItems: (id: string) =>
    apiClient.get<ApiResponse<PurchaseOrderItem[]>>(
      `purchases/orders/${id}/items`,
    ),
};
