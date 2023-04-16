<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<h1>Index</h1>

{#each data.posts as post}
	<a href="/posts/{post}">{post}</a>
{/each}
