// src/services/auth.ts
// Authentication API calls

// src/services/auth.ts
// Authentication API calls
import { API_BASE_URL } from './api';

export interface LoginRequest {
  username: string;
  password: string;
  remember_me?: boolean;
}

export interface UserInfo {
  id: string;
  username: string;
  email: string;
  first_name: string;
  last_name: string;
  role_id: string;
  company_id: string;
}

export interface LoginResponse {
  access_token: string;
  refresh_token: string;
  token_type: string;
  expires_in: number;
  user: UserInfo;
}

export interface RegisterRequest {
  company_name?: string;
  company_id?: string;
  username: string;
  email: string;
  password: string;
  password_confirmation: string;
  first_name: string;
  last_name: string;
  phone?: string;
  role_id: string;
}

export interface VerifyEmailRequest {
  token: string;
}

export interface RequestPasswordResetRequest {
  email: string;
}

// Since Tauri apps can use either Tauri commands or HTTP,
// we'll use Tauri commands for auth (more native)
// For HTTP API calls, we'd use fetch

export async function login(data: LoginRequest): Promise<LoginResponse> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      let errorMessage = 'Login failed';
      try {
        const errorData = await response.json();
        errorMessage = errorData.message || errorMessage;
      } catch (e) {
        errorMessage = await response.text() || errorMessage;
      }
      throw new Error(errorMessage);
    }

    return await response.json();
  } catch (error: any) {
    console.error('Login error:', error);
    if (error.message === 'Failed to fetch') {
      throw new Error('Cannot connect to the server. Is the backend running?');
    }
    throw error;
  }
}

export async function register(data: RegisterRequest): Promise<any> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'Registration failed');
    }

    return await response.json();
  } catch (error) {
    console.error('Register error:', error);
    throw error;
  }
}

export async function verifyEmailToken(token: string): Promise<any> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/verify-email`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ token } satisfies VerifyEmailRequest),
    });

    const json = await response.json().catch(() => ({}));
    if (!response.ok) {
      throw new Error(json?.message || "OTP verification failed.");
    }

    return json;
  } catch (error: any) {
    console.error("Verify email/token error:", error);
    throw error;
  }
}

export async function requestPasswordReset(email: string): Promise<any> {
  try {
    const response = await fetch(`${API_BASE_URL}/auth/request-password-reset`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email } satisfies RequestPasswordResetRequest),
    });

    const json = await response.json().catch(() => ({}));
    if (!response.ok) {
      throw new Error(json?.message || "Failed to resend OTP.");
    }

    return json;
  } catch (error: any) {
    console.error("Request password reset error:", error);
    throw error;
  }
}

export async function logout(): Promise<void> {
  const token = localStorage.getItem('access_token');
  
  try {
    await fetch(`${API_BASE_URL}/auth/logout`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });
  } catch (error) {
    console.error('Logout error:', error);
  } finally {
    localStorage.removeItem('access_token');
    localStorage.removeItem('refresh_token');
    localStorage.removeItem('user');
  }
}

export async function getCurrentUser(): Promise<UserInfo | null> {
  const token = localStorage.getItem('access_token');
  
  if (!token) return null;

  try {
    const response = await fetch(`${API_BASE_URL}/auth/me`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    });

    if (!response.ok) {
      throw new Error('Failed to get user info: ' + response.statusText);
    }

    const data = await response.json();
    return data.data || data;
  } catch (error: any) {
    console.error('Get current user error:', error);
    if (error.message === 'Failed to fetch') {
      throw new Error('Cannot connect to the server.');
    }
    throw error;
  }
}

export function isAuthenticated(): boolean {
  return !!localStorage.getItem('access_token');
}

export function getToken(): string | null {
  return localStorage.getItem('access_token');
}
