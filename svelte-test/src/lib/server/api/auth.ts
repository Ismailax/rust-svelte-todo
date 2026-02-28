import { API_URL } from '$lib/config/api';
import type { RequestEvent } from '@sveltejs/kit';
import type { AuthResponse, LogoutResponse } from '$lib/types/auth';

export async function register(
	fetch: RequestEvent['fetch'],
	username: string,
	password: string,
	password_confirmation: string
): Promise<AuthResponse> {
	const res = await fetch(`${API_URL}/register`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ username, password, password_confirmation })
	});

	if (!res.ok) {
		const message = await res.text();
		throw new Error(`register failed: ${message}`);
	}

	return await res.json();
}

export async function login(
	fetch: RequestEvent['fetch'],
	username: string,
	password: string
): Promise<AuthResponse> {
	const res = await fetch(`${API_URL}/login`, {
		method: 'POST',
		headers: { 'content-type': 'application/json' },
		body: JSON.stringify({ username, password })
	});

	if (!res.ok) {
		const message = await res.text();
		throw new Error(`login failed: ${message}`);
	}

	return await res.json();
}

export async function logout(fetch: RequestEvent['fetch']): Promise<LogoutResponse> {
	const res = await fetch(`${API_URL}/logout`, {
		method: 'POST',
		credentials: 'include'
	});

	if (!res.ok) {
		const message = await res.text();
		throw new Error(`logout failed: ${message}`);
	}

	return await res.json();
}
