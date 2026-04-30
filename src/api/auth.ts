import apiClient from "./client";
import type {
  LoginRequest,
  RegisterRequest,
  AuthTokens,
  AuthUser,
  ChangePasswordRequest,
  ResetPasswordRequest,
  ApiResponse,
} from "../types";

export const authApi = {
  login: (data: LoginRequest) =>
    apiClient.post<ApiResponse<{ user: AuthUser; tokens: AuthTokens }>>(
      "/auth/login",
      data,
    ),

  register: (data: RegisterRequest) =>
    apiClient.post<ApiResponse<{ user: AuthUser; tokens: AuthTokens }>>(
      "/auth/register",
      data,
    ),

  refresh: (refresh_token: string) =>
    apiClient.post<ApiResponse<AuthTokens>>("/auth/refresh", { refresh_token }),

  logout: () => apiClient.post<ApiResponse<null>>("/auth/logout"),

  me: () => apiClient.get<ApiResponse<AuthUser>>("/auth/me"),

  changePassword: (data: ChangePasswordRequest) =>
    apiClient.post<ApiResponse<null>>("/auth/change-password", data),

  verifyEmail: (token: string) =>
    apiClient.post<ApiResponse<null>>("/auth/verify-email", { token }),

  requestPasswordReset: (email: string) =>
    apiClient.post<ApiResponse<null>>("/auth/request-password-reset", {
      email,
    }),

  resetPassword: (data: ResetPasswordRequest) =>
    apiClient.post<ApiResponse<null>>("/auth/reset-password", data),
};
