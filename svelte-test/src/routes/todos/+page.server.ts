import { redirect, fail } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';
import { getTodos, addTodo, toggleTodo, renameTodo, deleteTodo } from '$lib/server/api/todos';

export const load: PageServerLoad = async ({ parent, fetch, cookies }) => {
	const { user } = await parent();
	if (!user) throw redirect(302, '/login');

	const token = cookies.get('access_token');
	if (!token) throw redirect(302, '/login');

	const todos = await getTodos(fetch, token);

	return { user, todos };
};

export const actions: Actions = {
	add: async ({ request, fetch, cookies }) => {
		const token = cookies.get('access_token');
		if (!token) return fail(401, { error: 'no token' });

		const fd = await request.formData();
		const title = String(fd.get('title') ?? '').trim();

		if (!title) return fail(400, { error: 'title required' });

		let todo;
		try {
			todo = await addTodo(fetch, token, title);
		} catch (err) {
			return fail(500, { error: (err as Error).message ?? 'add failed' });
		}

		return { todo };
	},

	toggle: async ({ request, fetch, cookies }) => {
		const token = cookies.get('access_token');
		if (!token) return fail(401, { error: 'no token' });

		const fd = await request.formData();
		const id = Number(fd.get('id'));
		const completed = fd.get('completed') === 'on';

		let todo;
		try {
			todo = await toggleTodo(fetch, token, id, completed);
		} catch (err) {
			return fail(500, { error: (err as Error).message ?? 'toggle failed' });
		}

		return { todo };
	},

	rename: async ({ request, fetch, cookies }) => {
		const token = cookies.get('access_token');
		if (!token) return fail(401, { error: 'no token' });

		const fd = await request.formData();
		const id = Number(fd.get('id'));
		const title = String(fd.get('title') ?? '').trim();

		if (!title) return fail(400, { error: 'title required' });

		let todo;
		try {
			todo = await renameTodo(fetch, token, id, title);
		} catch (err) {
			return fail(500, { error: (err as Error).message ?? 'rename failed' });
		}

		return { todo };
	},

	delete: async ({ request, fetch, cookies }) => {
		const token = cookies.get('access_token');
		if (!token) return fail(401, { error: 'no token' });

		const fd = await request.formData();
		const id = Number(fd.get('id'));

		try {
			await deleteTodo(fetch, token, id);
		} catch (err) {
			return fail(500, { error: (err as Error).message ?? 'delete failed' });
		}

		return { ok: true };
	}
};
