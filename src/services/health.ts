import { API_BASE_URL } from './api';

export async function checkHealth(): Promise<any> {
  const response = await fetch(`${API_BASE_URL}/health/`);
  if (!response.ok) throw new Error('Health check failed');
  return response.json();
}

export async function checkReadiness(): Promise<any> {
  const response = await fetch(`${API_BASE_URL}/health/ready`);
  if (!response.ok) throw new Error('Readiness check failed');
  return response.json();
}

export async function checkLiveness(): Promise<any> {
  const response = await fetch(`${API_BASE_URL}/health/live`);
  if (!response.ok) throw new Error('Liveness check failed');
  return response.json();
}
