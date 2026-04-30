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
