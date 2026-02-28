import { redirect, type Handle } from '@sveltejs/kit';

const PUBLIC_EXACT = new Set(['/login', '/register']);
const PROTECTED_PREFIXES = ['/todos'];

export const handle: Handle = async ({ event, resolve }) => {
	const { pathname } = event.url;
	const token = event.cookies.get('access_token');

	// root redirect
	if (pathname === '/') {
		throw redirect(302, token ? '/todos' : '/login');
	}

	// redirect if logged in user tries to access public pages
	if (token && PUBLIC_EXACT.has(pathname)) {
		throw redirect(302, '/todos');
	}

	// protect all pages under /todos
	const isProtected = PROTECTED_PREFIXES.some(
		(p) => pathname === p || pathname.startsWith(p + '/')
	);

	if (!token && isProtected) {
		throw redirect(302, '/login');
	}

	return resolve(event);
};
