import * as jspb from 'google-protobuf'

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
    expires: string,
    user_id: string,
    kind: UserKind,
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
    provider?: DataUserProvider.AsObject,
    consumer?: DataUserConsumer.AsObject,
}

export interface UpdatePasswordRequest {
	user_id: string,
    old: string,
    new: string,
}
e
export enum UserKind { 
  PROVIDER = 0,
  DRIVER = 1,
  CONSUMER = 2,
}
