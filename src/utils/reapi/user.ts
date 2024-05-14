import { fetch_api } from '.';

export enum UserKind {
	Driver = 'Driver',
	Restaurant = 'Restaurant',
	Farm = 'Farm',
}

export interface UserData {
	user_id: string;
	token: string;
	expires: Date;
	kind: UserKind;
}

export async function login(email: string, password: string): Promise<UserData> {
	const body = {
		email,
		password,
	};

	const response = await fetch_api('/auth', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body),
	});

	if (!response.ok) {
		const text = await response.text();
		throw new Error(`Failed to login: ${response.status}: ${text.length <= 30 ? text : response.statusText}`);
	}

	const data = await response.json();

	return {
		user_id: data.user_id,
		token: data.token,
		expires: new Date(data.expires),
		kind: UserKind[data.kind as keyof typeof UserKind],
	};
}

export async function signup(email: string, password: string, name: string, kind: UserKind): Promise<void> {
	const body = {
		email,
		password,
		name,
		kind: kind.toString(),
	};

	const response = await fetch_api('/user', {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body),
	});

	if (!response.ok) {
		const text = await response.text();
		throw new Error(`Failed to sign up: ${response.status}: ${text.length <= 30 ? text : response.statusText}`);
	}
}
