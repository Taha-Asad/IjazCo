import apiClient from "./client";
import type {
  LoginRequest,
  RegisterRequest,
  AuthTokens,
  AuthUser,
  ChangePasswordRequest,
  ResetPasswordRequest,
  ApiResponse,
  LoginResponse,
  SuccessResponse,
} from "../types";

export const authApi = {
  login: (data: LoginRequest) =>
    apiClient.post<LoginResponse>("auth/login", data),

  register: (data: RegisterRequest) =>
    apiClient.post<SuccessResponse<AuthUser>>("auth/register", data),

  refresh: (refresh_token: string) =>
    apiClient.post<ApiResponse<AuthTokens>>("auth/refresh", { refresh_token }),

  logout: () => apiClient.post<ApiResponse<null>>("auth/logout"),

  me: () => apiClient.get<AuthUser>("auth/me"),

  changePassword: (data: ChangePasswordRequest) =>
    apiClient.post<ApiResponse<null>>("auth/change-password", data),

  verifyEmail: (token: string) =>
    apiClient.post<ApiResponse<null>>("auth/verify-email", { token }),

  requestPasswordReset: (email: string) =>
    apiClient.post<ApiResponse<null>>("auth/request-password-reset", {
      email,
    }),

  resetPassword: (data: ResetPasswordRequest) =>
    apiClient.post<ApiResponse<null>>("auth/reset-password", data),
};
