import { API_URL } from '$lib/config/api';
import type { Todo } from '$lib/types/todo';
import type { RequestEvent } from '@sveltejs/kit';

export async function getTodos(
	fetch: RequestEvent['fetch'],
	token: string | null
): Promise<Todo[]> {
	if (!token) return [];

	const res = await fetch(`${API_URL}/todos`, {
		headers: { Authorization: `Bearer ${token}` }
	});

	if (!res.ok) return [];
	return await res.json();
}

export async function addTodo(
	fetch: RequestEvent['fetch'],
	token: string,
	title: string
): Promise<Todo> {
	const res = await fetch(`${API_URL}/todos`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify({ title })
	});
	if (!res.ok) {
		const message = await res.text();
		throw new Error(`Failed to add todo: ${message}`);
	}
	return await res.json();
}

export async function toggleTodo(
	fetch: RequestEvent['fetch'],
	token: string,
	id: number,
	completed: boolean
): Promise<Todo> {
	const res = await fetch(`${API_URL}/todos/${id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify({ completed })
	});
	if (!res.ok) {
		const message = await res.text();
		throw new Error(`Failed to toggle todo: ${message}`);
	}
	return await res.json();
}

export async function renameTodo(
	fetch: RequestEvent['fetch'],
	token: string,
	id: number,
	title: string
): Promise<Todo> {
	const res = await fetch(`${API_URL}/todos/${id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${token}`
		},
		body: JSON.stringify({ title })
	});
	if (!res.ok) {
		const message = await res.text();
		throw new Error(`Failed to rename todo: ${message}`);
	}
	return await res.json();
}

export async function deleteTodo(
	fetch: RequestEvent['fetch'],
	token: string,
	id: number
): Promise<void> {
	const res = await fetch(`${API_URL}/todos/${id}`, {
		method: 'DELETE',
		headers: { Authorization: `Bearer ${token}` }
	});
	if (!res.ok) {
		const message = await res.text();
		throw new Error(`Failed to delete todo: ${message}`);
	}
	return;
}
