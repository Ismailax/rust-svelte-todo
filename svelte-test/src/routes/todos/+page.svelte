<script lang="ts">
import { enhance } from '$app/forms';
import type { User } from '$lib/types/user';
import type { Todo } from '$lib/types/todo';
export let data: { user: User; todos: Todo[] };

let editingId: number | null = null;
let editingTitle = '';
let addError: string | null = null;

function startEdit(t: Todo) {
	editingId = t.id;
	editingTitle = t.title;
}

function cancelEdit() {
	editingId = null;
	editingTitle = '';
}
</script>

<div class="mx-auto max-w-xl space-y-6 p-6">
	<header class="flex justify-center">
		<h1 class="text-2xl font-bold">Todo List</h1>
	</header>

	<!-- Add form -->
	<form method="POST" action="?/add" use:enhance class="flex gap-2">
		<input
			name="title"
			placeholder="What needs to be done?"
			class="flex-1 rounded-lg border px-3 py-2 focus:ring focus:outline-none"
			on:input={() => (addError = null)}
		/>
		<button class="rounded-lg bg-blue-600 px-4 py-2 text-white hover:bg-blue-700">Add</button>
	</form>
	{#if addError}
		<p class="text-sm text-red-600">{addError}</p>
	{/if}

	<!-- List -->
	{#if data.todos.length === 0}
		<div class="flex justify-center text-xl">No todos yet. Add one!</div>
	{/if}

	{#each data.todos as t (t.id)}
		<ul class="divide-y rounded-xl border">
			<li class="flex items-center gap-3 p-4">
				<!-- toggle -->
				<form method="POST" action="?/toggle" use:enhance class="mr-1">
					<input type="hidden" name="id" value={t.id} />
					<input
						type="checkbox"
						name="completed"
						checked={t.completed}
						on:change={(e) => (e.currentTarget.form as HTMLFormElement).requestSubmit()}
					/>
				</form>

				<!-- title / edit -->
				{#if editingId === t.id}
					<form
						method="POST"
						action="?/rename"
						use:enhance={() => {
							return async ({ result, update }) => {
								if (result.type === 'success') {
									await update();
									editingId = null;
									editingTitle = '';
								}
							};
						}}
						class="flex flex-1 gap-2"
					>
						<input type="hidden" name="id" value={t.id} />
						<input name="title" class="flex-1 rounded border px-3 py-2" bind:value={editingTitle} />
						<button class="rounded bg-emerald-600 px-3 py-2 text-white">Save</button>
						<button type="button" class="rounded bg-gray-200 px-3 py-2" on:click={cancelEdit}
							>Cancel</button
						>
					</form>
				{:else}
					<div class="flex-1">
						<span class="select-text {t.completed ? 'text-gray-400 line-through' : ''}">
							{t.title}
						</span>
					</div>
					<button
						class="rounded bg-gray-100 px-3 py-1 hover:bg-gray-200"
						on:click={() => startEdit(t)}>Edit</button
					>
				{/if}

				<!-- delete -->
				<form method="POST" action="?/delete" use:enhance>
					<input type="hidden" name="id" value={t.id} />
					<button class="rounded bg-red-600 px-3 py-1 text-white hover:bg-red-700">Delete</button>
				</form>
			</li>
		</ul>
	{/each}
</div>

<style>
.line-through {
	text-decoration: line-through;
}
</style>
