export * as user from './user';

export const API_URL = 'http://localhost:8084';
export const fetch_api = (path: string, options: RequestInit) => fetch(API_URL + path, options);
