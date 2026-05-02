import apiClient from "./client";
import type {
  DashboardStats,
  SalesChartData,
  InventoryValuation,
  ApiResponse,
} from "../types";

export const dashboardApi = {
  getStats: () =>
    apiClient.get<ApiResponse<DashboardStats>>("dashboard/stats"),

  getSalesSummary: (params?: { period?: "daily" | "monthly" | "yearly" }) =>
    apiClient.get<ApiResponse<SalesChartData[]>>("dashboard/sales-summary", {
      params,
    }),

  getInventoryValuation: () =>
    apiClient.get<ApiResponse<InventoryValuation[]>>(
      "dashboard/inventory-valuation",
    ),

  getSalesChart: (params?: { period?: "daily" | "monthly"; months?: number }) =>
    apiClient.get<ApiResponse<SalesChartData[]>>("dashboard/sales-chart", {
      params,
    }),
};
