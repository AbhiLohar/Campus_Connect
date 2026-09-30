'use client';

import React, { createContext, useEffect, useState, ReactNode } from 'react';
import { useRouter } from 'next/navigation';
import { User, AuthResponse } from '../types';
import { apiGet, apiPost } from '../lib/api';
import { isAuthenticated as checkIsAuth, setTokens, clearTokens } from '../lib/auth';
import { LoginInput, RegisterInput } from '../lib/validators';

interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (data: LoginInput) => Promise<void>;
  register: (data: RegisterInput) => Promise<void>;
  logout: () => void;
}

export const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const router = useRouter();

  useEffect(() => {
    const initAuth = async () => {
      if (checkIsAuth()) {
        try {
          const userData = await apiGet<User>('/auth/me');
          setUser(userData);
        } catch (error) {
          clearTokens();
          setUser(null);
        }
      }
      setIsLoading(false);
    };
    initAuth();
  }, []);

  const login = async (data: LoginInput) => {
    const res = await apiPost<AuthResponse>('/auth/login', data);
    setTokens(res.accessToken, res.refreshToken);
    setUser(res.user);
    router.push('/home');
  };

  const register = async (data: RegisterInput) => {
    const res = await apiPost<AuthResponse>('/auth/register', data);
    setTokens(res.accessToken, res.refreshToken);
    setUser(res.user);
    router.push('/home');
  };

  const logout = () => {
    clearTokens();
    setUser(null);
    router.push('/login');
  };

  return (
    <AuthContext.Provider value={{ user, isAuthenticated: !!user, isLoading, login, register, logout }}>
      {children}
    </AuthContext.Provider>
  );
}
