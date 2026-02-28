import { redirect } from '@sveltejs/kit';
import { logout } from '$lib/server/api/auth';
import { clearAuthCookie } from '$lib/server/auth/cookies';

export const actions = {
	default: async ({ fetch, cookies }) => {
		try {
			await logout(fetch);
		} catch {
			return;
		}

		clearAuthCookie(cookies);

		throw redirect(302, '/login');
	}
};
