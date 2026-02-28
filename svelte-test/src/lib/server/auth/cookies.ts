import type { Cookies } from '@sveltejs/kit';

export const setAuthCookie = (cookies: Cookies, token: string) => {
	cookies.set('access_token', token, {
		path: '/',
		httpOnly: true,
		sameSite: 'lax',
		secure: false,
		maxAge: 60 * 60
	});
};

export const clearAuthCookie = (cookies: Cookies) => {
	cookies.delete('access_token', {
		path: '/',
		httpOnly: true,
		sameSite: 'lax',
		secure: false
	});
};
