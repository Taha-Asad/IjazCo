import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from "axios";
import { useAuthStore } from "../store"; // Adjust path to your store

const BASE_URL = import.meta.env.VITE_API_URL || "http://localhost:8000/api/v1";

class ApiClient {
  private instance: AxiosInstance;

  constructor() {
    this.instance = axios.create({
      baseURL: BASE_URL,
      timeout: 30000,
      headers: {
        "Content-Type": "application/json",
      },
      // THIS FIXES THE "i64" ERROR:
      // It ensures params are serialized correctly and empty values are removed
      paramsSerializer: {
        serialize: (params) => {
          const parts: string[] = [];
          Object.entries(params).forEach(([key, value]) => {
            if (value === null || value === undefined || value === "") return;
            // Don't convert numbers to strings
            parts.push(
              `${encodeURIComponent(key)}=${encodeURIComponent(value)}`,
            );
          });
          return parts.join("&");
        },
      },
    });

    this.setupInterceptors();
  }

  private setupInterceptors() {
    this.instance.interceptors.request.use(
      (config) => {
        const token = useAuthStore.getState().accessToken;
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => Promise.reject(error),
    );

    this.instance.interceptors.response.use(
      (response) => response,
      async (error) => {
        const originalRequest = error.config;

        if (error.response?.status === 401 && !originalRequest._retry) {
          originalRequest._retry = true;

          try {
            const refreshToken = useAuthStore.getState().refreshToken;
            if (!refreshToken) throw new Error("No refresh token");

            const response = await axios.post(`${BASE_URL}/auth/refresh`, {
              refresh_token: refreshToken,
            });

            const { access_token } = response.data.data;
            useAuthStore.getState().setTokens(access_token, refreshToken);

            originalRequest.headers.Authorization = `Bearer ${access_token}`;
            return this.instance(originalRequest);
          } catch (refreshError) {
            useAuthStore.getState().logout();
            window.location.href = "/login";
            return Promise.reject(refreshError);
          }
        }
        return Promise.reject(error);
      },
    );
  }

  async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response: AxiosResponse<T> = await this.instance.get(url, config);
    // Handle both {data: {...}} and direct {...} responses
    const data = response.data as any;
    if (
      data &&
      typeof data === "object" &&
      "data" in data &&
      "success" in data
    ) {
      return data.data;
    }
    return response.data;
  }

  async post<T>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    const response: AxiosResponse<T> = await this.instance.post(
      url,
      data,
      config,
    );
    // Handle both {data: {...}} and direct {...} responses
    const respData = response.data as any;
    if (
      respData &&
      typeof respData === "object" &&
      "data" in respData &&
      "success" in respData
    ) {
      return respData.data;
    }
    return response.data;
  }

  async put<T>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    const response: AxiosResponse<T> = await this.instance.put(
      url,
      data,
      config,
    );
    // Handle both {data: {...}} and direct {...} responses
    const respData = response.data as any;
    if (
      respData &&
      typeof respData === "object" &&
      "data" in respData &&
      "success" in respData
    ) {
      return respData.data;
    }
    return response.data;
  }

  async patch<T>(
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig,
  ): Promise<T> {
    const response: AxiosResponse<T> = await this.instance.patch(
      url,
      data,
      config,
    );
    // Handle both {data: {...}} and direct {...} responses
    const respData = response.data as any;
    if (
      respData &&
      typeof respData === "object" &&
      "data" in respData &&
      "success" in respData
    ) {
      return respData.data;
    }
    return response.data;
  }

  async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response: AxiosResponse<T> = await this.instance.delete(url, config);
    // Handle both {data: {...}} and direct {...} responses
    const respData = response.data as any;
    if (
      respData &&
      typeof respData === "object" &&
      "data" in respData &&
      "success" in respData
    ) {
      return respData.data;
    }
    return response.data;
  }
}

export const apiClient = new ApiClient();
export default apiClient;
