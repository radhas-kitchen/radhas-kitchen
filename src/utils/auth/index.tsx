'use client';

import { createContext, useContext, useEffect, useState } from 'react';
import { LoginResponse } from '@proto/auth';
import { useRouter } from 'next/navigation';

export interface Auth {
	token: LoginResponse | undefined;
	loading: boolean;

	setToken: (token: LoginResponse) => void;
	logout: () => void;
}

export interface AuthProviderProps {
	children: React.ReactNode;
}

const AuthContext = createContext(undefined as Auth | undefined);

export function AuthProvider({ children }: AuthProviderProps) {
	const [token, setRawToken] = useState<LoginResponse | undefined>(undefined);
	const [loading, setLoading] = useState(true);
	const [isLoggedout, forceLogout] = useState(false);

	useEffect(() => {
		if (!loading) return;
		if (isLoggedout) return;

		const token = localStorage.getItem('token');
		if (token) setRawToken(JSON.parse(token));

		setLoading(false);
	});

	useEffect(() => {
		if (isLoggedout) localStorage.removeItem('token');
	}, [isLoggedout]);

	useEffect(() => {
		if (token) localStorage.setItem('token', JSON.stringify(token));
	}, [token]);

	const setToken = (token: LoginResponse) => {
		setRawToken(token);
		forceLogout(false);
	};

	const logout = () => {
		forceLogout(true);
		setRawToken(undefined);
	};

	return (
		<AuthContext.Provider
			value={{
				token,
				setToken,
				loading,
				logout,
			}}
		>
			{children}
		</AuthContext.Provider>
	);
}

export function ensureToken(auth: Auth): LoginResponse | undefined {
	if (auth.loading) return undefined;
	if (!auth.token) return useRouter().push('/login') as never;
	if (new Date(auth.token.expires) < new Date()) return useRouter().push('/login') as never;

	return auth.token;
}

export function useAuth() {
	const context = useContext(AuthContext);

	if (!context) throw new Error('useAuth must be used within an AuthProvider');
	return context;
}
