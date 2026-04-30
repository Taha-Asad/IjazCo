export interface Role {
  id: string;
  name: string;
  description?: string;
  company_id: string;
  permissions: Record<string, string[]>;
  user_count?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateRoleRequest {
  name: string;
  description?: string;
  permissions?: Record<string, string[]>;
}

export const AVAILABLE_RESOURCES = [
  "users",
  "companies",
  "roles",
  "categories",
  "customers",
  "suppliers",
  "inventory",
  "stock",
  "sales",
  "purchases",
  "imports",
  "reports",
  "dashboard",
] as const;

export type Resource = (typeof AVAILABLE_RESOURCES)[number];

export const AVAILABLE_ACTIONS = [
  "create",
  "read",
  "update",
  "delete",
] as const;
export type Action = (typeof AVAILABLE_ACTIONS)[number];
