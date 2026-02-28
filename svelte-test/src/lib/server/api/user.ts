import { API_URL } from '$lib/config/api';
import type { User } from '$lib/types/user';
import type { RequestEvent } from '@sveltejs/kit';

export async function getMe(
	fetch: RequestEvent['fetch'],
	token: string | null
): Promise<User | null> {
	if (!token) return null;

	const res = await fetch(`${API_URL}/me`, {
		headers: { Authorization: `Bearer ${token}` }
	});

	if (!res.ok) return null;
	return await res.json();
}
