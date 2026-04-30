import apiClient from "./client";
import type {
  SalesInvoice,
  SalesInvoiceItem,
  CreateSalesInvoiceRequest,
  RecordPaymentRequest,
  SalesSummary,
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "@/types";

export const salesApi = {
  list: (
    params?: PaginationParams & { status?: string; customer_id?: string },
  ) =>
    apiClient.get<PaginatedResponse<SalesInvoice>>("/sales/invoices", {
      params,
    }),

  getById: (id: string) =>
    apiClient.get<ApiResponse<SalesInvoice>>(`/sales/invoices/${id}`),

  create: (data: CreateSalesInvoiceRequest) =>
    apiClient.post<ApiResponse<SalesInvoice>>("/sales/invoices", data),

  update: (id: string, data: Partial<CreateSalesInvoiceRequest>) =>
    apiClient.put<ApiResponse<SalesInvoice>>(`/sales/invoices/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`/sales/invoices/${id}`),

  approve: (id: string) =>
    apiClient.post<ApiResponse<SalesInvoice>>(`/sales/invoices/${id}/approve`),

  recordPayment: (id: string, data: RecordPaymentRequest) =>
    apiClient.post<ApiResponse<SalesInvoice>>(
      `/sales/invoices/${id}/payment`,
      data,
    ),

  getItems: (id: string) =>
    apiClient.get<ApiResponse<SalesInvoiceItem[]>>(
      `/sales/invoices/${id}/items`,
    ),

  getSummary: (params?: { start_date?: string; end_date?: string }) =>
    apiClient.get<ApiResponse<SalesSummary>>("/sales/invoices/summary", {
      params,
    }),
};
