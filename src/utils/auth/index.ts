export {
	AuthProvider, useAuth, ensureToken, type Auth,
} from './context';

export interface Token {
	token: string;
	expires: Date;
}

export async function login(username: string, password: string): Promise<Token> {
	// alert('unimplemented');

	// eslint yelling at me
	await (async () => {})();

	return {
		token: '',
		expires: new Date('2025-01-01T00:00:00Z'),
	};
}
