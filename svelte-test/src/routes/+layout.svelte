<script lang="ts">
import '../app.css';
import Navbar from '../lib/components/Navbar.svelte';
import type { User } from '$lib/types/user';
export let data: { user: User | null };
</script>

<svelte:head>
	<meta charset="utf-8" />
	<meta name="viewport" content="width=device-width, initial-scale=1" />
	<link rel="icon" href="/favicon.png" />
	<title>Svelte Test Project</title>
</svelte:head>

<Navbar user={data.user} />

<main>
	<slot />
</main>
