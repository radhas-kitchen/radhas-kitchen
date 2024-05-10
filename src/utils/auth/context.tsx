'use client';

import { createContext, useContext, useEffect, useState } from 'react';
import { Token } from '.';
import { useRouter } from 'next/navigation';

export interface Auth {
	token: Token | undefined;
	setToken: (token: Token) => void;
	loading: boolean;
}

export interface AuthProviderProps {
	children: React.ReactNode;
}

const AuthContext = createContext(undefined as Auth | undefined);

export function AuthProvider({ children }: AuthProviderProps) {
	const [token, setToken] = useState<Token | undefined>(undefined);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		const lstoken = localStorage.getItem('token');
		if (lstoken) setToken(JSON.parse(lstoken));

		setLoading(false);
	}, []);

	useEffect(() => {
		if (token) localStorage.setItem('token', JSON.stringify(token));
	}, [token]);

	return (
		<AuthContext.Provider
			value={{
				token,
				setToken,
				loading,
			}}
		>
			{children}
		</AuthContext.Provider>
	);
}

export function ensureToken(auth: Auth): Token | undefined {
	if (auth.loading) return undefined;
	if (!auth.token) return useRouter().push('/login') as never;
	if (auth.token.expires < new Date()) return useRouter().push('/login') as never;
	return auth.token;
}

export function useAuth() {
	const context = useContext(AuthContext);

	if (!context) throw new Error('useAuth must be used within an AuthProvider');
	return context;
}
