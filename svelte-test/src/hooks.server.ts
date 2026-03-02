import { redirect, type Handle } from '@sveltejs/kit';
import { decodeJwt } from 'jose';
import { clearAuthCookie } from '$lib/server/auth/cookies';

const PUBLIC_PAGES = ['/login', '/register'];
const PROTECTED_PREFIX = '/todos';

export const handle: Handle = async ({ event, resolve }) => {
	const token = event.cookies.get('access_token') ?? null;
	const { pathname } = event.url;

	// 1. จัดการข้อมูล User จาก JWT (Rich Token)
	if (token) {
		try {
			// แกะข้อมูลจาก Token โดยตรง (ไม่รอ Network Hop)
			// โครงสร้าง payload จะตรงกับ Claims ใน Rust (id, username, exp, iat, iss)
			const payload = decodeJwt(token);

			// ตรวจสอบเบื้องต้นว่า Token หมดอายุหรือยัง (แกะจาก exp claim)
			const isExpired = payload.exp ? Date.now() >= payload.exp * 1000 : false;

			if (isExpired) {
				event.locals.user = null;
				clearAuthCookie(event.cookies);
			} else {
				// เก็บข้อมูลเข้า locals เพื่อให้หน้าอื่นๆ (+layout.server.ts) ใช้งานได้ทันที
				event.locals.user = {
					id: Number(payload.id),
					username: payload.username as string
				};
			}
		} catch {
			// กรณี Token ผิดรูปแบบ หรือ Decode ไม่ได้
			event.locals.user = null;
			clearAuthCookie(event.cookies);
		}
	} else {
		event.locals.user = null;
	}

	const isLogged = !!event.locals.user;

	// 2. Logic การทำ Redirect (เหมือนเดิมแต่ทำงานเร็วขึ้นเพราะไม่ต้องรอ fetch)
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
