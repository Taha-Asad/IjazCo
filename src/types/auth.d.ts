export interface LoginResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
  user: AuthUser;
}

export interface SuccessResponse<T = unknown> {
  status: number;
  message: string;
  data: T;
  timestamp: string;
}

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
  password_confirmation: string;
  first_name: string;
  last_name: string;
  company_name?: string;
  company_id?: string;
  phone?: string;
  role_id?: string;
}

export interface AuthTokens {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
}

export interface AuthUser {
  id: string;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  company_id: string;
  role_id: string;
  role_name?: string;
  permissions?: Record<string, string[]>;
}

export interface User {
  id: string;
  company_id: string;
  role_id: string;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  phone: string | null;
  avatar_url: string | null;
  status: string;
  is_email_verified: boolean;
  last_login_at: string | null;
  two_factor_enabled: boolean;
  preferences: Record<string, any>;
  created_at: string;
  updated_at: string;
}

export interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

export interface ResetPasswordRequest {
  token: string;
  new_password: string;
}
