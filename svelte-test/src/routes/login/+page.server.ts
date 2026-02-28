import { fail, redirect } from '@sveltejs/kit';
import { login } from '$lib/server/api/auth';
import { setAuthCookie } from '$lib/server/auth/cookies';

export const actions = {
	default: async ({ request, fetch, cookies }) => {
		const fd = await request.formData();
		const username = String(fd.get('username') ?? '');
		const password = String(fd.get('password') ?? '');

		if (!username || !password) {
			return fail(400, { error: 'username and password are required' });
		}

		let data;
		try {
			data = await login(fetch, username, password);
		} catch (err) {
			return fail(400, { error: (err as Error).message ?? 'login failed' });
		}
		setAuthCookie(cookies, data.token);
		throw redirect(302, '/todos');
	}
};
