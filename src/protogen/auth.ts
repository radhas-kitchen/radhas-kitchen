import { invoke } from '@tauri-apps/api/core';

export interface Authorization {
    user_id: string,
    token: string,
}

export interface LoginRequest {
	email: string,
    password: string,
}

export interface LoginResponse {
    token: string,
    expires: Date,
    user_id: string,
    kind: UserKind,
}

interface RawLoginResponse {
	token: string,
	expires: string,
	user_id: string,
	kind: 0 | 1 | 2,
}

function convert(raw: RawLoginResponse): LoginResponse {
	console.log(raw.expires);

	return {
		token: raw.token,
		expires: new Date(raw.expires),
		user_id: raw.user_id,
		kind: raw.kind,
	}
}

export interface DataUserProvider {
    location: string,
}

export interface DataUserConsumer {
    location: string,
}

export interface CreateUserRequest {
	email: string,
    password: string,
    name: string,
    kind: DataUserConsumer | DataUserProvider | undefined,
}

export interface UpdatePasswordRequest {
	user_id: string,
    old: string,
    new: string,
}

export enum UserKind { 
	Provider = 0,
	Driver = 1,
	Consumer = 2,
}

export const Auth = {
	Login: (request: LoginRequest) => invoke<RawLoginResponse>('auth_login', { request }).then(convert),
	CreateUser: (request: CreateUserRequest) => invoke<void>('auth_create_user', { request }),
};
