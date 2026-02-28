<script>
import { resolve } from '$app/paths';
let form = { error: null };
</script>

<div class="flex min-h-screen items-center justify-center bg-gray-50">
	<form method="POST" class="w-full max-w-sm rounded-xl bg-white p-8 shadow">
		<h2 class="mb-6 text-center text-2xl font-semibold text-gray-800">Login</h2>

		{#if form.error}
			<p class="mb-4 rounded bg-red-100 p-2 text-sm text-red-700">{form.error}</p>
		{/if}

		<div class="mb-4">
			<label for="username" class="mb-1 block text-sm font-medium text-gray-600">Username</label>
			<input
				id="username"
				name="username"
				placeholder="username"
				required
				class="w-full rounded-lg border px-3 py-2 focus:ring focus:ring-blue-300 focus:outline-none"
			/>
		</div>
		<div class="mb-6">
			<label for="password" class="mb-1 block text-sm font-medium text-gray-600">Password</label>
			<input
				id="password"
				name="password"
				type="password"
				placeholder="password"
				required
				class="w-full rounded-lg border px-3 py-2 focus:ring focus:ring-blue-300 focus:outline-none"
			/>
		</div>

		<button
			class="w-full rounded-lg bg-blue-600 py-2 font-medium text-white transition hover:bg-blue-700"
		>
			Login
		</button>

		<p class="mt-4 text-center text-sm text-gray-600">
			Don't have an account?
			<a href={resolve('/register')} class="text-blue-600 hover:underline"> Register </a>
		</p>
	</form>
</div>
