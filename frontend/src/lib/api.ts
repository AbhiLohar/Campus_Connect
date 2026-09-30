import { getAccessToken, setTokens, getRefreshToken, clearTokens } from './auth';

const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080/api/v1';

async function refreshAuthToken(): Promise<boolean> {
  const refresh = getRefreshToken();
  if (!refresh) return false;

  try {
    const res = await fetch(`${API_URL}/auth/refresh`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ refreshToken: refresh })
    });

    if (!res.ok) throw new Error('Refresh failed');
    const data = await res.json();
    setTokens(data.accessToken, data.refreshToken);
    return true;
  } catch (error) {
    clearTokens();
    if (typeof window !== 'undefined') {
      window.location.href = '/login';
    }
    return false;
  }
}

async function fetchWithAuth(url: string, options: RequestInit = {}): Promise<Response> {
  let token = getAccessToken();
  const headers = new Headers(options.headers || {});
  headers.set('Content-Type', 'application/json');
  
  if (token) {
    headers.set('Authorization', `Bearer ${token}`);
  }

  let response = await fetch(`${API_URL}${url}`, { ...options, headers });

  if (response.status === 401 && token) {
    const refreshed = await refreshAuthToken();
    if (refreshed) {
      token = getAccessToken();
      headers.set('Authorization', `Bearer ${token}`);
      response = await fetch(`${API_URL}${url}`, { ...options, headers });
    }
  }

  return response;
}

export async function apiGet<T>(url: string): Promise<T> {
  const res = await fetchWithAuth(url);
  if (!res.ok) throw await res.json();
  return res.json();
}

export async function apiPost<T>(url: string, data?: any): Promise<T> {
  const res = await fetchWithAuth(url, {
    method: 'POST',
    body: JSON.stringify(data),
  });
  if (!res.ok) throw await res.json();
  return res.json();
}

export async function apiPatch<T>(url: string, data: any): Promise<T> {
  const res = await fetchWithAuth(url, {
    method: 'PATCH',
    body: JSON.stringify(data),
  });
  if (!res.ok) throw await res.json();
  return res.json();
}

export async function apiDelete<T>(url: string): Promise<T> {
  const res = await fetchWithAuth(url, { method: 'DELETE' });
  if (!res.ok) throw await res.json();
  return res.json();
}
