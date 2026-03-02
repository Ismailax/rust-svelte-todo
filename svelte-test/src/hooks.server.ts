import { redirect, type Handle } from '@sveltejs/kit';
import { getMe } from '$lib/server/api/user';
import { clearAuthCookie } from '$lib/server/auth/cookies';

const PUBLIC_PAGES = ['/login', '/register'];
const PROTECTED_PREFIX = '/todos';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('access_token') ?? null;
	const { pathname } = event.url;

	// 1. ดึงข้อมูล User (ทำครั้งเดียวที่นี่)
	// ถ้าไม่มี token ไม่ต้องวิ่งไป API ให้เสียเวลา (ลด Latency ไป 1 รอบ)
	if (token) {
		event.locals.user = await getMe(event.fetch, token);
		if (!event.locals.user) {
			// ถ้า token เน่า หรือ API บอกว่าไม่ผ่าน ให้ล้างทิ้ง
			clearAuthCookie(event.cookies);
		}
	} else {
		event.locals.user = null;
	}

	const isLogged = !!event.locals.user;

	// 2. Logic การไล่ที่ (Redirects)
	if (pathname === '/') {
		throw redirect(302, isLogged ? '/todos' : '/login');
	}

	if (isLogged && PUBLIC_PAGES.includes(pathname)) {
		throw redirect(302, '/todos');
	}

	if (!isLogged && pathname.startsWith(PROTECTED_PREFIX)) {
		throw redirect(302, '/login');
	}

	// 3. ไปต่อ
	return resolve(event);
};
