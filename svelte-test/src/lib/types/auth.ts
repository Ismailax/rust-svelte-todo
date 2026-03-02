import type { User } from './user';

export interface AuthResponse {
	message: string;
	user: User;
	token: string;
}

export interface LogoutResponse {
	message: string;
}
