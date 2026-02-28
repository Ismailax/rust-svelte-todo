import type { User } from './user';

export type AuthResponse = {
	message: string;
	user: User;
	token: string;
};

export interface LogoutResponse {
	message: string;
}
