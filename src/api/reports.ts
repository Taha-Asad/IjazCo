import apiClient from "./client";
import type { ApiResponse } from "../types";

export interface SalesReportParams {
  start_date: string;
  end_date: string;
  branch_id?: string;
  customer_id?: string;
}

export interface InventoryReportParams {
  branch_id?: string;
  category_id?: string;
}

export const reportsApi = {
  salesReport: (params: SalesReportParams) =>
    apiClient.get<ApiResponse<any>>("reports/sales", { params }),

  inventoryReport: (params?: InventoryReportParams) =>
    apiClient.get<ApiResponse<any>>("reports/inventory", { params }),

  exportPdf: (data: {
    report_type: "sales" | "inventory";
    params: Record<string, any>;
  }) => apiClient.post("reports/export/pdf", data, { responseType: "blob" }),
};
