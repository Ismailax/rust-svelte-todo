<script lang="ts">
export let user: { id: number; username?: string } | null = null;
</script>

{#if user}
	<nav class="flex items-center justify-between rounded-b-xl bg-gray-200 p-4">
		<span class="text-lg font-semibold">{user.username}</span>
		<form method="POST" action="/logout">
			<button class="rounded bg-gray-800 px-3 py-1 text-white hover:bg-gray-700">Logout</button>
		</form>
	</nav>
{/if}
