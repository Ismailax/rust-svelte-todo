import type { LayoutServerLoad } from './$types';
import { clearAuthCookie } from '$lib/server/auth/cookies';
import { getMe } from '$lib/server/api/user';

export const load: LayoutServerLoad = async ({ cookies, fetch }) => {
	const token = cookies.get('access_token') ?? null;
	const user = await getMe(fetch, token);
	if (!user) {
		clearAuthCookie(cookies);
	}
	return { user };
};
