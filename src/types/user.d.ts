export interface User {
  id: string;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  company_id: string;
  role_id: string;
  role_name?: string;
  is_active: boolean;
  email_verified: boolean;
  last_login?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateUserRequest {
  username: string;
  email: string;
  password: string;
  first_name: string;
  last_name: string;
  role_id: string;
  company_id: string;
}

export interface UpdateUserRequest {
  first_name?: string;
  last_name?: string;
  email?: string;
  role_id?: string;
}

export interface UpdateUserStatusRequest {
  is_active: boolean;
}
