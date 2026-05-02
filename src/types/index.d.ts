export * from "./auth";
export * from "./user";
export * from "./company";
export * from "./role";
export * from "./category";
export * from "./customer";
export * from "./supplier";
export * from "./inventory";
export * from "./stock";
export * from "./sales";
export * from "./purchase";
export * from "./import";
export * from "./dashboard";

export interface PaginatedResponse<T> {
  data: T[];
  current_page: number;
  per_page: number;
  total_items: number;
  total_pages: number;
}

export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data: T;
}

export interface PaginationParams {
  page?: number;
  per_page?: number;
  search?: string;
  sort_by?: string;
  sort_order?: "asc" | "desc";
  company_id?: string;
}
