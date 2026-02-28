<script lang="ts">
import { page } from '$app/state';
</script>

<div class="flex h-screen items-center justify-center">
	<h1 class="flex text-2xl font-bold">{page.status} {page.error?.message}</h1>
</div>
