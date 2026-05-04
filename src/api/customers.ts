import apiClient from "./client";
import type {
  ApiResponse,
  PaginatedResponse,
  PaginationParams,
} from "../types";

export interface Customer {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  country?: string;
  credit_limit: number;
  current_balance?: number;
  outstanding_balance?: number;
  company_id: string;
  is_active: boolean;
  total_invoices?: number;
  total_spent?: number;
  created_at: string;
}

function mapCustomerBalance(customer: any): Customer {
  return {
    ...customer,
    current_balance: customer.current_balance ?? customer.outstanding_balance ?? 0,
  };
}

export const customersApi = {
  list: (params?: PaginationParams) =>
    apiClient.get<any>("customers", { params }).then(data => {
      if (data?.data) {
        data.data = data.data.map(mapCustomerBalance);
      }
      return data as PaginatedResponse<Customer>;
    }),

  getById: (id: string) =>
    apiClient.get<any>(`customers/${id}`).then(data => {
      // apiClient.get already unwraps {success, data} if present
      const customer = data?.data || data;
      return mapCustomerBalance(customer);
    }),

  create: (data: Omit<Customer, "id" | "created_at" | "current_balance" | "outstanding_balance">) =>
    apiClient.post<ApiResponse<Customer>>("customers", data),

  update: (id: string, data: Partial<Customer>) =>
    apiClient.put<ApiResponse<Customer>>(`customers/${id}`, data),

  delete: (id: string) =>
    apiClient.delete<ApiResponse<null>>(`customers/${id}`),
};
