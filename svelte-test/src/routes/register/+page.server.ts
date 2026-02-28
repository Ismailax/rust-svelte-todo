import { fail, redirect } from '@sveltejs/kit';
import { register } from '$lib/server/api/auth';
import { setAuthCookie } from '$lib/server/auth/cookies';

export const actions = {
	default: async ({ request, fetch, cookies }) => {
		const fd = await request.formData();
		const username = String(fd.get('username') ?? '');
		const password = String(fd.get('password') ?? '');
		const password_confirmation = String(fd.get('password_confirmation') ?? '');

		if (!username || !password || !password_confirmation) {
			return fail(400, { error: 'username, password, and password confirmation are required' });
		}
		let data;
		try {
			data = await register(fetch, username, password, password_confirmation);
		} catch (err) {
			return fail(400, { error: (err as Error).message ?? 'register failed' });
		}
		setAuthCookie(cookies, data.token);
		throw redirect(302, '/todos');
	}
};
