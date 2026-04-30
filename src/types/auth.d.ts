export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
  full_name: string;
  company_id?: string;
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
  full_name: string;
  company_id: string;
  role_id: string;
  role_name: string;
  is_active: boolean;
  permissions: Record<string, string[]>;
}

export interface ChangePasswordRequest {
  current_password: string;
  new_password: string;
}

export interface ResetPasswordRequest {
  token: string;
  new_password: string;
}
